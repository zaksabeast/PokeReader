@   This paricular file is licensed under the following terms:

@   This software is provided 'as-is', without any express or implied warranty. In no event will the authors be held liable
@   for any damages arising from the use of this software.
@
@   Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it
@   and redistribute it freely, subject to the following restrictions:
@
@    The origin of this software must not be misrepresented; you must not claim that you wrote the original software.
@    If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.
@
@    Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.
@    This notice may not be removed or altered from any source distribution.

@    This file was modified from https://github.com/LumaTeam/Luma3DS to only have svcControlService

@    This file was take from PKSM: https://github.com/FlagBrew/PKSM/blob/aaf1a35ccfda893258a261b3ea42d452a8f2ea77/3ds/source/utils/csvc.s

.arm
.balign 4

.macro SVC_BEGIN name
    .section .text.\name, "ax", %progbits
    .global \name
    .type \name, %function
    .align 2
    .cfi_startproc
\name:
.endm

.macro SVC_END
    .cfi_endproc
.endm

SVC_BEGIN svcControlService
    svc 0xB0
    bx lr
SVC_END