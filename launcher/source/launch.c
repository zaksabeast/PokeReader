#include <string.h>
#include <3ds/result.h>
#include <3ds/svc.h>
#include <3ds/srv.h>
#include <3ds/synchronization.h>
#include <3ds/ipc.h>
#include "launch.h"

static Handle pkrdHandle;
static int pkrdRefcount;

// Thanks to libctru https://github.com/devkitPro/libctru/blob/09688ea6fc16421041b6dd110ab68bb99ef9df6b/libctru/source/services/pmapp.c#L33
Result Custom_PMAPP_LaunchTitle(Handle pmAppHandle, const FS_ProgramInfo *programInfo, u32 launchFlags)
{
  Result ret = 0;
  u32 *cmdbuf = getThreadCommandBuffer();

  cmdbuf[0] = IPC_MakeHeader(0x1, 5, 0); // 0x10140
  memcpy(&cmdbuf[1], programInfo, sizeof(FS_ProgramInfo));
  cmdbuf[5] = launchFlags;

  if (R_FAILED(ret = svcSendSyncRequest(pmAppHandle)))
    return ret;

  return (Result)cmdbuf[1];
}

Result pkrdInit(void)
{
  Result res;
  if (AtomicPostIncrement(&pkrdRefcount))
    return 0;
  res = srvGetServiceHandle(&pkrdHandle, "pkrd:game");
  if (R_FAILED(res))
    AtomicDecrement(&pkrdRefcount);
  return res;
}

Result PKRD_Setup(void)
{
  Result ret = 0;
  u32 *cmdbuf = getThreadCommandBuffer();

  cmdbuf[0] = IPC_MakeHeader(0x1, 0, 1);
  cmdbuf[1] = IPC_Desc_MoveHandles(1);
  cmdbuf[2] = pkrdHandle;

  if (R_FAILED(ret = svcSendSyncRequest(pkrdHandle)))
    return ret;

  return (Result)cmdbuf[1];
}
