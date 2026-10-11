use super::hid::map_input_hook;
use super::present::run_hook;
use crate::ctr::res::CtrResult;
use crate::ctr::{pa_from_va_ptr, query_memory};
use crate::{func_ptr, pnp};
use core::slice::from_raw_parts;

/// Replacement for the game's present framebuffer function.
/// It calls `run_hook`, then does the same work as the original function.
/// After it is a trampoline used by `hook_game_branch!`.
const DRAW_PATCH: [u8; 0x94] = [
    0xf0, 0x5f, 0x2d, 0xe9, // stmdb      sp!,{r4 r5 r6 r7 r8 r9 r10 r11 r12 lr}
    0x0f, 0x00, 0x2d, 0xe9, // stmdb      sp!,{r0 r1 r2 r3}
    // Injected reader call
    0x64, 0xc0, 0x9f, 0xe5, // ldr        r12,[run_hook_addr]
    0x3c, 0xff, 0x2f, 0xe1, // blx        r12
    // End reader call
    0xf0, 0x00, 0xbd, 0xe8, // ldmia      sp!,{r4 r5 r6 r7}
    0x28, 0x00, 0x8d, 0xe2, // add        r0,sp,#0x28
    0x00, 0x0e, 0x90, 0xe8, // ldmia      r0,{r9 r10 r11}
    0xd3, 0x03, 0x00, 0xeb, // bl         get_screen
    0x5c, 0x10, 0x80, 0xe2, // add        r1,r0,#0x5c
    0x04, 0x21, 0x91, 0xe7, // ldr        r2,[r1,r4,lsl #0x2]
    0x04, 0x30, 0xa0, 0xe3, // mov        r3,#0x4
    0x00, 0x00, 0xd2, 0xe5, // ldrb       r0,[r2,#0x0]
    0x01, 0x00, 0x60, 0xe2, // rsb        r0,r0,#0x1
    0xff, 0x00, 0x00, 0xe2, // and        r0,r0,#0xff
    0x80, 0xe1, 0x60, 0xe0, // rsb        lr,r0,r0, lsl #0x3
    0x0e, 0x31, 0x83, 0xe0, // add        r3,r3,lr, lsl #0x2
    0x03, 0x30, 0x82, 0xe0, // add        r3,r2,r3
    0xe0, 0x0e, 0x83, 0xe8, // stmia      r3,{ r5 r6 r7 r9 r10 r11 }
    0x9a, 0x8f, 0x07, 0xee, // mcr        p15,0x0,r8,cr7,cr10,0x4
    0x04, 0x21, 0x91, 0xe7, // ldr        r2,[r1,r4,lsl #0x2]
    0x9f, 0x3f, 0x92, 0xe1, // ldrex      r3,[r2]
    0xff, 0x30, 0xc3, 0xe3, // bic        r3,r3,#0xff
    0x00, 0x30, 0x83, 0xe1, // orr        r3,r3,r0
    0xff, 0x3c, 0xc3, 0xe3, // bic        r3,r3,#0xff00
    0x01, 0x3c, 0x83, 0xe3, // orr        r3,r3,#0x100
    0x93, 0x6f, 0x82, 0xe1, // strex      r6,r3,[r2]
    0x00, 0x00, 0x56, 0xe3, // cmp        r6,#0x0
    0xf6, 0xff, 0xff, 0x1a, // bne        loop
    0xf0, 0x9f, 0xbd, 0xe8, // ldmia      sp!,{r4 r5 r6 r7 r8 r9 r10 r11 r12 pc}
    0x00, 0x00, 0x00, 0x00, // run_hook_addr
    // Hook trampoline
    0xff, 0xdf, 0x2d, 0xe9, // stmdb      sp!,{r0-r12, lr, pc}
    0x0d, 0x00, 0xa0, 0xe1, // cpy        r0,sp
    0x08, 0xc0, 0x9f, 0xe5, // ldr        r12,[trampoline_addr]
    0x3c, 0xff, 0x2f, 0xe1, // blx        r12
    0x00, 0x40, 0xbd, 0xe8, // ldmia      sp!,{lr}
    0xff, 0x9f, 0xbd, 0xe8, // ldmia      sp!,{r0-r12, pc}
    0x00, 0x00, 0x00, 0x00, // trampoline_addr
];

const DRAW_PATCH_GET_SCREEN_WORD: usize = 7;
const DRAW_PATCH_RUN_HOOK_WORD: usize = 29;
const DRAW_PATCH_TRAMPOLINE_WORD: usize = 30;
const TRAMPOLINE_ROUTE_HOOK_WORD: usize = 6;

/// Found 8 bytes into the present framebuffer function.
const PRESENT_FRAMEBUFFER_BYTES: [u8; 0x10] = [
    0x28, 0x00, 0x8d, 0xe2, 0x00, 0x80, 0xa0, 0xe3, 0x01, 0x70, 0xa0, 0xe1, 0x00, 0x0e, 0x90, 0xe8,
];

/// The game's `svcMapMemoryBlock` call for the HID shared memory.
const MAP_INPUT_BLOCK: [u8; 0x1c] = [
    0x01, 0x20, 0xa0, 0x13, 0x03, 0x20, 0xa0, 0x03, 0x01, 0x32, 0xa0, 0xe3, 0x1f, 0x00, 0x00, 0xef, 0xa0,
    0x1f, 0xb0, 0xe1, 0x01, 0x10, 0xa0, 0x03, 0x18, 0x10, 0xc4, 0x05,
];

fn find_in_code(base_addr: u32, code: &[u8], bytes: &[u8]) -> CtrResult<u32> {
    let pos = code
        .windows(bytes.len())
        .position(|window| window == bytes)
        .ok_or(-1)?;

    Ok(base_addr + pos as u32)
}

/// Code is read-only, so writes go through the physical address.
fn write_code_word(addr: u32, word_index: usize, value: u32) {
    let pa = pa_from_va_ptr(addr) as *mut u32;
    unsafe { pa.add(word_index).write_unaligned(value) };
}

fn hook_present_framebuffer(base_addr: u32, code: &[u8]) -> CtrResult<()> {
    let present_buffer_addr = find_in_code(base_addr, code, &PRESENT_FRAMEBUFFER_BYTES)? - 8;

    let get_screen_branch = unsafe { ((present_buffer_addr + 0x20) as *const u32).read() } + 1;

    let present_buffer_pa = pa_from_va_ptr(present_buffer_addr) as *mut u8;
    unsafe { core::ptr::copy_nonoverlapping(DRAW_PATCH.as_ptr(), present_buffer_pa, DRAW_PATCH.len()) };

    // The patch has one more instruction before get_screen than the original, so fix the relative branch.
    write_code_word(present_buffer_addr, DRAW_PATCH_GET_SCREEN_WORD, get_screen_branch);
    write_code_word(present_buffer_addr, DRAW_PATCH_RUN_HOOK_WORD, func_ptr!(run_hook));

    let trampoline_addr = present_buffer_addr + (DRAW_PATCH_TRAMPOLINE_WORD as u32 * 4);
    let route_hook_addr = trampoline_addr + (TRAMPOLINE_ROUTE_HOOK_WORD as u32 * 4);
    pnp::set_trampoline_addrs(trampoline_addr, route_hook_addr);

    Ok(())
}

fn hook_hid(base_addr: u32, code: &[u8]) -> CtrResult<()> {
    let addr = find_in_code(base_addr, code, &MAP_INPUT_BLOCK)?;

    write_code_word(addr, 0, 0xe51fe000); // ldr lr,[pc + 0x8]
    write_code_word(addr, 1, 0xe51ff000); // ldr pc,[pc + 0x8]
    // 4 instructions * 4 bytes per instruction
    write_code_word(addr, 2, addr + (4 * 4)); // return address
    write_code_word(addr, 3, func_ptr!(map_input_hook)); // jump address

    Ok(())
}

pub fn install_hooks() -> CtrResult<()> {
    let (mem, _page) = query_memory(0x100000)?;

    let base_addr = mem.base_addr;
    let code = unsafe { from_raw_parts(base_addr as *const u8, mem.size as usize) };

    hook_present_framebuffer(base_addr, code)?;
    hook_hid(base_addr, code)?;

    Ok(())
}
