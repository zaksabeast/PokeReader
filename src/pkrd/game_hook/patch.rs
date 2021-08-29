use crate::pkrd::pkrd_game;
use alloc::{vec, vec::Vec};
use ctr::{log, res::CtrResult, DebugProcess, Handle};

pub fn patch_game() -> CtrResult<()> {
    if let Ok(debug) = DebugProcess::new(0x0004000000055E00) {
        log("Found Pokemon Y!");
        let process = debug.get_process();
        let handle: Handle = pkrd_game::get_raw_pkrd_handle().into();
        let handle_copy = process.copy_handle_to_process(&handle)?;

        let cmd_header = 0x20040; // The normal param is the stack pointer (not translated)

        let hook_code: Vec<u32> = vec![
            0xe92d5fff, // stmdb      {r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, lr}
            // Store values where they belong with less instructions
            0xe8bd00c0, // ldmia      sp!,{r6, r7}
            0xe8bd0030, // ldmia      sp!,{r4, r5}
            0xe28d0028, // add        r0,sp,#0x28
            0xe8900e00, // ldmia      r0,{r9, r10, r11}
            // Setup and run command using registers we can toss
            0xe3a0c717, // mov        r12,#0x5c0000
            0xee1d8f70, // mrc        p15,0x0,r8,cr13,cr0,0x3
            0xe2888080, // add        r8,r8,#0x80
            0xe89c1001, // ldmia      r12,{r0 r12}
            0xe8883000, // stmia      r8,{r12 sp}
            0xef000032, // swi        0x32
            // Resume normal (and moved) instructions
            0xeb006ccb, // bl         FUN_001646b4
            0xe280105c, // add        r1,r0,#0x5c
            0xe7912106, // ldr        r2,[r1,r6, lsl #2]
            0xe3a03004, // mov        r3,#0x4
            0xe5d20000, // ldrb       r0,[fb_a,#0x0]
            0xe2600001, // rsb        r0,r0,#0x1
            0xe20000ff, // and        r0,r0,#0xff
            0xe060e180, // rsb        lr,r0,r0, lsl #0x3
            0xe083310e, // add        r3,r3,lr, lsl #0x2
            0xe7a27003, // str        r7,[r2,r3]!
            0xe2823004, // add        r3,r2,#4
            0xe8830e30, // stmia      r3,{r4, r5, r9, r10, r11}
            0xee078f9a, // mcr        p15,0x0,r8,cr7,cr10,0x4
            0xe3a0c001, // mov        r12,#0x1
        ];

        let hook_vars = vec![
            unsafe { handle_copy.get_raw() }, // Session handle
            cmd_header,                       // Command header
        ];

        debug.write_bytes(0x149354, safe_transmute::transmute_to_bytes(&hook_code))?;
        debug.write_bytes(0x5c0000, safe_transmute::transmute_to_bytes(&hook_vars))?;

        debug.eat_events()?;
    } else {
        log("Did not find Pokemon Y");
    }

    Ok(())
}
