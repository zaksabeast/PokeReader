#include <3ds.h>
#include "title_info.h"

u64 g_program_id = 0;

u64 get_title_id()
{

  if (g_program_id == 0)
  {
    fsInit();
    u32 process_id = 0;
    svcGetProcessId(&process_id, CUR_PROCESS_HANDLE);
    FS_ProgramInfo info;
    FSUSER_GetProgramLaunchInfo(&info, process_id);
    g_program_id = info.programId;
    fsExit();
  }

  return g_program_id;
}