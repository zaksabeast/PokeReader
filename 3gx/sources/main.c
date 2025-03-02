#include <3ds.h>
#include <string.h>
#include <stdio.h>
#include "plgldr.h"
#include "csvc.h"
#include "common.h"
#include "ov.h"
#include "pnp.h"
#include "pokereader.h"
#include "title_info.h"
#include "hid.h"
#include "memmem.h"

static Handle thread;
static Handle memLayoutChanged;
static u8 stack[0x1000] __attribute__((aligned(8)));
static bool is_paused = false;

void handle_freeze(bool isTopScreen)
{
    if (host_is_just_pressed(KEY_START | KEY_SELECT) || host_is_just_pressed(KEY_L | KEY_R))
    {
        is_paused = true;
    }

    while (is_paused && !isTopScreen)
    {
        scan_input();

        u32 just_pressed = host_just_pressed();

        if (just_pressed == KEY_SELECT || just_pressed == KEY_L)
        {
            break;
        }

        if (just_pressed == KEY_A || just_pressed == KEY_START || just_pressed == KEY_R)
        {
            is_paused = false;
            break;
        }

        svcSleepThread(50000000);
    }
}

void run_hook(u32 _1, u32 _2, u32 _3, u32 _4, u32 screenId, u32 swap, u8 *fb_a, u8 *fb_b, u32 stride, u32 format)
{
    bool isTopScreen = screenId == 0;
    if (isTopScreen)
    {
        scan_input();
        run_frame();
        draw_to_screen(screenId, fb_a, stride, format);
    }

    svcFlushProcessDataCache(CUR_PROCESS_HANDLE, (u32)fb_a, SCREEN_WIDTH * SCREEN_HEIGHT);
    // Thanks to https://github.com/44670/NTR/blob/c764c0f68c08f3518a9f284f5fda1bf3b2636123/source/plg.c#L868-L870
    if (isTopScreen && fb_a != fb_b && fb_b != 0)
    {
        svcFlushProcessDataCache(CUR_PROCESS_HANDLE, (u32)fb_b, SCREEN_WIDTH * SCREEN_HEIGHT);
    }

    handle_freeze(isTopScreen);
}

Result map_input_hook(u32 memblock_handle, u32 addr, u32 _r2, u32 _r3, u32 _r4, u32 _r5)
{
    bool has_write_perm = _r5 == 0;
    if (!has_write_perm)
    {
        set_key_addr((vu32 *)(addr + 0x28));
    }
    u32 my_perm = has_write_perm ? MEMPERM_READ | MEMPERM_WRITE : MEMPERM_READ;
    // Mimic the game's permissions
    return svcMapMemoryBlock(memblock_handle, addr, my_perm, MEMPERM_DONTCARE);
}

u8 DRAW_PATCH[0x94] = {
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
};

u8 HID_INPUT_MAP_PATCH[0x8] = {
    0x00, 0xe0, 0x1f, 0xe5, //     ldr        lr,[pc + 0x8]
    0x00, 0xf0, 0x1f, 0xe5, //     ldr        pc,[pc + 0x8]
};

u8 PRESENT_FRAMEBUFFER_BYTES[0X10] = {
    0x28, 0x00, 0x8d, 0xe2, 0x00, 0x80, 0xa0, 0xe3, 0x01, 0x70, 0xa0, 0xe1, 0x00, 0x0e, 0x90, 0xe8,
};

u8 MAP_INPUT_BLOCK[] = {
    0x01, 0x20, 0xa0, 0x13, 0x03, 0x20, 0xa0, 0x03, 0x01, 0x32, 0xa0, 0xe3, 0x1f, 0x00,
    0x00, 0xef, 0xa0, 0x1f, 0xb0, 0xe1, 0x01, 0x10, 0xa0, 0x03, 0x18, 0x10, 0xc4, 0x05
};

extern char *fake_heap_start;
extern char *fake_heap_end;
extern u32 __ctru_heap;
extern u32 __ctru_linear_heap;

u32 __ctru_heap_size = 0;
u32 __ctru_linear_heap_size = 0;

void __system_allocateHeaps(PluginHeader *header)
{
    __ctru_heap_size = header->heapSize;
    __ctru_heap = header->heapVA;

    // Set up newlib heap
    fake_heap_start = (char *)__ctru_heap;
    fake_heap_end = fake_heap_start + __ctru_heap_size;
}

// Entrypoint, game will starts when you exit this function
void main(void)
{
    PluginHeader *header = (PluginHeader *)0x07000000;

    // Init heap
    __system_allocateHeaps(header);

    // Init services
    srvInit();

    // NTP epoch (milliseconds since 1st Jan 1900 00:00)
    u64 ms = osGetTime();
    // Adjust to Jan 2000, which is what the games use.
    u64 game_ms = ms - 3155673600000;
    set_game_start_ms(game_ms);

    // Get memory layout changed event
    svcControlProcess(CUR_PROCESS_HANDLE, PROCESSOP_GET_ON_MEMORY_CHANGE_EVENT, (u32)&memLayoutChanged, 0);

    MemInfo info;
    PageInfo out;
    svcQueryMemory(&info, &out, 0x100000);
 
    u32 present_buffer_ptr = (u32)memmem((u8*)info.base_addr, info.size, PRESENT_FRAMEBUFFER_BYTES, sizeof(PRESENT_FRAMEBUFFER_BYTES)) - 8;
    u32 map_input_memory_block = (u32)memmem((u8*)info.base_addr, info.size, MAP_INPUT_BLOCK, sizeof(MAP_INPUT_BLOCK));

    u32 get_screen_branch = *(u32 *)(present_buffer_ptr + 0x20) + 1;
    u32 *present_buffer_pa = (u32 *)PA_FROM_VA_PTR(present_buffer_ptr);
    memcpy(present_buffer_pa, DRAW_PATCH, 0x94);
    present_buffer_pa[7] = get_screen_branch; // fix get_screen branch instruction
    present_buffer_pa[29] = (u32)run_hook;
    u32 trampoline_addr = (u32)present_buffer_ptr + (30 * 4);
    set_trampoline_addr(trampoline_addr);
    set_route_hook_addr(trampoline_addr + (6 * 4));

    u32 *map_input_memory_block_pa = (u32 *)PA_FROM_VA_PTR(map_input_memory_block);
    memcpy(map_input_memory_block_pa, HID_INPUT_MAP_PATCH, 0x8);
    // 4 instructions * 4 bytes per instruction
    map_input_memory_block_pa[0x2] = (u32)map_input_memory_block + (0x4 * 0x4); // set return address
    map_input_memory_block_pa[0x3] = (u32)map_input_hook;                       // set jump address

    initialize();
    svcInvalidateEntireInstructionCache();
}
