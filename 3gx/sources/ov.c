// Thanks to https://github.com/44670/ntr_overlay_samples/blob/master/fps/source/ov.c
#include "font.h"
#include "ov.h"

void ovDrawTranspartBlackRect(u32 addr, u32 stride, u32 format, int r, int c, int h, int w, u8 level)
{
  format &= 0x0f;
  int posC;
  for (posC = c; posC < c + w; posC++)
  {
    if (format == 2)
    {
      u16 *sp = (u16 *)(addr + stride * posC + 240 * 2 - 2 * (r + h - 1));
      u16 *spEnd = sp + h;
      while (sp < spEnd)
      {
        u16 pix = *sp;
        u16 r = (pix >> 11) & 0x1f;
        u16 g = (pix >> 5) & 0x3f;
        u16 b = (pix & 0x1f);
        pix = ((r >> level) << 11) | ((g >> level) << 5) | (b >> level);
        *sp = pix;
        sp++;
      }
    }
    else if (format == 3)
    {
      u16 *sp = (u16 *)(addr + stride * posC + 240 * 2 - 2 * (r + h - 1));
      u16 *spEnd = sp + h;
      while (sp < spEnd)
      {
        u16 pix = *sp;
        u16 r = (pix >> 11) & 0x1f;
        u16 b = (pix >> 6) & 0x1f;
        u16 g = (pix & 0x3f);
        pix = ((r >> level) << 11) | ((b >> level) << 6) | (g >> level);
        *sp = pix;
        sp++;
      }
    }
    else if (format == 1)
    {
      u8 *sp = (u8 *)(addr + stride * posC + 240 * 3 - 3 * (r + h - 1));
      u8 *spEnd = sp + 3 * h;
      while (sp < spEnd)
      {
        sp[0] >>= level;
        sp[1] >>= level;
        sp[2] >>= level;
        sp += 3;
      }
    }
  }
}

void ovDrawPixel(u32 addr, u32 stride, u32 format, int posR, int posC, u32 r, u32 g, u32 b)
{
  format &= 0x0f;
  if (format == 2 || format == 3)
  {
    u16 pix = ((r) << 11) | ((g) << 5) | (b);
    *(u16 *)(addr + stride * posC + 480 - 2 * posR) = pix;
  }
  else if (format == 3)
  {
    u16 pix = ((r) << 11) | ((b) << 6) | (g);
    *(u16 *)(addr + stride * posC + 480 - 2 * posR) = pix;
  }
  else
  {
    u8 *sp = (u8 *)(addr + stride * posC + 720 - 3 * posR);
    sp[0] = b;
    sp[1] = g;
    sp[2] = r;
  }
}

void ovDrawRect(u32 addr, u32 stride, u32 format, int posR, int posC, int h, int w, u32 r, u32 g, u32 b)
{
  int r_, c_;
  for (c_ = posC; c_ < posC + w; c_++)
  {
    for (r_ = posR; r_ < posR + h; r_++)
    {
      ovDrawPixel(addr, stride, format, r_, c_, r, g, b);
    }
  }
}

void ovDrawChar(u32 addr, u32 stride, u32 format, u8 letter, int y, int x, u32 r, u32 g, u32 b)
{

  int i;
  int k;
  int c;
  unsigned char mask;
  unsigned char *_letter;
  unsigned char l;

  if ((letter < 32) || (letter > 127))
  {
    letter = '?';
  }

  c = (letter - 32) * 8;

  for (i = 0; i < 8; i++)
  {
    mask = 0b10000000;
    l = font[i + c];
    for (k = 0; k < 8; k++)
    {
      if ((mask >> k) & l)
      {
        ovDrawPixel(addr, stride, format, i + y, k + x, r, g, b);
      }
    }
  }
}

void ovDrawString(u32 addr, u32 stride, u32 format, u32 scrnWidth, int posR, int posC, u32 r, u32 g, u32 b, u8 *buf)
{
  while (*buf)
  {
    if ((posR + 8 > 240) || (posC + 8 > scrnWidth))
    {
      return;
    }
    ovDrawChar(addr, stride, format, *buf, posR, posC, r, g, b);
    buf++;
    posC += 8;
  }
}