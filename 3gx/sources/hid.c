#include <3ds.h>

vu32 *g_key_addr = 0;
u32 g_current_keys = 0;
u32 g_previous_keys = 0;

void set_key_addr(vu32 *key_addr)
{
  g_key_addr = key_addr;
}

void scan_input()
{
  if (g_key_addr != 0)
  {
    g_previous_keys = g_current_keys;
    g_current_keys = *g_key_addr;
  }
}

u32 get_current_keys()
{
  return g_current_keys;
}

u32 get_previous_keys()
{
  return g_previous_keys;
}
