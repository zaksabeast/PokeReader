// PNP Compatibility file
// PNP uses wasm, so all functions are wasm compatible - hence the weirdness.

#pragma once

#include <3ds.h>

void draw_to_screen(u32 screenId, u8 *framebuffer, u32 stride, u32 format);
void host_print(u32 ptr, u32 size);
void host_read_mem(u32 game_addr, u32 size, u32 out_ptr);
void host_write_mem(u32 game_addr, u32 size, u32 in_ptr);
void scan_input();
u32 host_just_pressed();
u32 host_is_just_pressed(u32 io_bits);
void host_set_print_max_len(u32 max_len);
u64 host_get_game_title_id();
void set_game_start_ms(u64 time);
u64 host_game_start_ms();
