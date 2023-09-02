// Thanks to https://github.com/44670/ntr_overlay_samples/blob/master/fps/source/ov.h
#pragma once

#include <3ds.h>

#define SCREEN_WIDTH 400
#define SCREEN_HEIGHT 240

void ovDrawTranspartBlackRect(u32 addr, u32 stride, u32 format, int r, int c, int h, int w, u8 level);
void ovDrawPixel(u32 addr, u32 stride, u32 format, int posR, int posC, u32 r, u32 g, u32 b);
void ovDrawRect(u32 addr, u32 stride, u32 format, int posR, int posC, int h, int w, u32 r, u32 g, u32 b);
void ovDrawChar(u32 addr, u32 stride, u32 format, u8 letter, int y, int x, u32 r, u32 g, u32 b);
void ovDrawString(u32 addr, u32 stride, u32 format, u32 scrnWidth, int posR, int posC, u32 r, u32 g, u32 b, u8 *buf);
