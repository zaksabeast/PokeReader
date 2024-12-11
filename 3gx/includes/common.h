#pragma once

// Those macros are from Luma3DS - https://github.com/AuroraWright/Luma3DS
// {

// For accessing physmem uncached (and directly)
#define PA_PTR(addr)            (void *)((u32)(addr) | 1 << 31)
#define PA_FROM_VA_PTR(addr)    PA_PTR(svcConvertVAToPA((const void *)(addr), false))

// }
