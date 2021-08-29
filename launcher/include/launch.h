/**
 * @file loader.h
 * @brief LOADER Service
 */

#pragma once

#ifdef __cplusplus
extern "C"
{
#endif

#include <3ds/exheader.h>
#include <3ds/services/fs.h>

  // Thanks to libctru https://github.com/devkitPro/libctru/blob/09688ea6fc16421041b6dd110ab68bb99ef9df6b/libctru/include/3ds/services/pmapp.h#L39
  Result Custom_PMAPP_LaunchTitle(Handle handle, const FS_ProgramInfo *programInfo, u32 launchFlags);

  Result pkrdInit(void);

  Result PKRD_Setup(void);

#ifdef __cplusplus
}
#endif