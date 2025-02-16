use crate::pnp;

struct ReplacedBranch {
    original_address: u32,
    new_branch_instruction: u32,
}

fn replace_arm_branch(
    instruction_address: u32,
    branch_instruction: u32,
    new_jump_address: u32,
) -> ReplacedBranch {
    let original_offset = (branch_instruction << 8) as i32 >> 6;
    let original_address = (instruction_address as i32 + original_offset + 8) as u32;

    if branch_instruction >> 24 != 0xeb {
        return ReplacedBranch {
            original_address,
            new_branch_instruction: branch_instruction,
        };
    }

    let new_offset =
        (((new_jump_address as i32 - instruction_address as i32 - 8) >> 2) & 0x00FFFFFF) as u32;
    let new_branch_instruction = (branch_instruction & 0xFF000000) | new_offset;

    ReplacedBranch {
        original_address,
        new_branch_instruction,
    }
}

pub fn hook_addr(address: u32, new_jump_address: u32) -> u32 {
    let branch_instruction: u32 = pnp::read(address);
    let replaced_branch = replace_arm_branch(address, branch_instruction, new_jump_address);
    pnp::write(address, &replaced_branch.new_branch_instruction);
    replaced_branch.original_address
}

pub fn install_hook_router(base_addr: u32, route_hook_addr: u32) {
    let hook_patch: [u32; 9] = [
        0xe92d4000, // stmdb // sp!,{lr}
        0xe92d5fff, // stmdb // sp!,{r0,r1,r2,r3,r4,r5,r6,r7,r8,r9,r10,r11,r12
        0xe1a0000d, // mov r0, sp
        0xe59fe008, // ldr // lr,[0x5bbff8]
        0xe59ff008, // ldr // pc,[0x5bbffc]
        0xe8bd5fff, // ldmia // sp!,{r0,r1,r2,r3,r4,r5,r6,r7,r8,r9,r10,r11,r12
        0xe8bd8000, // ldmia // sp!,{pc}
        base_addr + 0x14,
        route_hook_addr,
    ];

    for (index, instruction) in hook_patch.iter().enumerate() {
        pnp::write(base_addr + (index as u32 * 4), instruction);
    }
}

#[macro_export]
macro_rules! hook_game_branch {
  (hook_router_addr = $router:expr, $($fn_name:ident = $address:expr,)+) => {
      $(
          mod $fn_name {
              #[allow(non_upper_case_globals)]
              pub(super) static mut return_addr: u32 = 0;
          }
      )*

      unsafe {
          $(
              $fn_name::return_addr = $crate::utils::hook_addr($address, $router);
          )*
      }

      pub unsafe extern "C" fn route_hook(stack_pointer: *mut u32) {
          let regs = core::slice::from_raw_parts_mut(stack_pointer, 15);

          $(
              if regs[13] == ($address + 4) {
                  $fn_name(regs, stack_pointer.add(15));
                  regs[14] = $fn_name::return_addr;
              }
          )*
      }

      $crate::utils::install_hook_router($router, route_hook as u32);
  };
}

pub use hook_game_branch;
