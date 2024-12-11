/*   This paricular file is licensed under the following terms: */

/*
*   This software is provided 'as-is', without any express or implied warranty. In no event will the authors be held liable
*   for any damages arising from the use of this software.
*
*   Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it
*   and redistribute it freely, subject to the following restrictions:
*
*    The origin of this software must not be misrepresented; you must not claim that you wrote the original software.
*    If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.
*
*    Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.
*    This notice may not be removed or altered from any source distribution.
*/

#pragma once

#include <3ds/types.h>

/// Operations for svcControlService
typedef enum ServiceOp
{
    SERVICEOP_STEAL_CLIENT_SESSION = 0, ///< Steal a client session given a service or global port name
    SERVICEOP_GET_NAME,                 ///< Get the name of a service or global port given a client or session handle
} ServiceOp;

///@name I/O
///@{
/**
 * @brief Gives the physical address corresponding to a virtual address.
 * @param VA Virtual address.
 * @param writeCheck whether to check if the VA is writable in supervisor mode
 * @return The corresponding physical address, or NULL.
*/
u32 svcConvertVAToPA(const void *VA, bool writeCheck);

/**
 * @brief Invalidates the data cache entirely.
*/
void svcInvalidateEntireInstructionCache(void);
///@}


/// Operations for svcControlProcess
typedef enum ProcessOp
{
    PROCESSOP_GET_ALL_HANDLES,  ///< List all handles of the process, varg3 can be either 0 to fetch all handles, or token of the type to fetch
                                ///< svcControlProcess(handle, PROCESSOP_GET_ALL_HANDLES, (u32)&outBuf, 0)
    PROCESSOP_SET_MMU_TO_RWX,   ///< Set the whole memory of the process with rwx access
                                ///< svcControlProcess(handle, PROCESSOP_SET_MMU_TO_RWX, 0, 0)
    PROCESSOP_GET_ON_MEMORY_CHANGE_EVENT,
    PROCESSOP_GET_ON_EXIT_EVENT,
    PROCESSOP_GET_PA_FROM_VA,   ///< Get the physical address of the va within the process
                                ///< svcControlProcess(handle, PROCESSOP_GET_PA_FROM_VA, (u32)&outPa, va)
} ProcessOp;

Result  svcControlProcess(Handle process, ProcessOp op, u32 varg2, u32 varg3);
///@}
