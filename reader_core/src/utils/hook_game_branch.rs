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
            original_address: 0,
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

#[macro_export]
macro_rules! hook_game_branch {
  (game_name = $game_name:expr, $($fn_name:ident = $address:expr,)+) => {
      $(
          mod $fn_name {
              #[allow(non_upper_case_globals)]
              pub(super) static mut return_addr: u32 = 0;
          }
      )*

      let trampoline_addr = $crate::pnp::get_trampoline_addr();

      unsafe {
          $(
              $fn_name::return_addr = $crate::utils::hook_addr($address, trampoline_addr);
          )*
      }

      pub unsafe extern "C" fn route_hook(stack_pointer: *mut u32) {
          let regs = core::slice::from_raw_parts_mut(stack_pointer, 15);

          $(
              if regs[13] == ($address + 4) {
                  $fn_name(regs, stack_pointer.add(15));
                  regs[14] = regs[13];
                  regs[13] = $fn_name::return_addr;
              }
          )*

          regs.rotate_right(1);
      }

      let route_hook_addr = $crate::pnp::get_route_hook_addr();
      $crate::pnp::write(route_hook_addr, &(route_hook as u32));
  };
}

pub use hook_game_branch;

#[cfg(test)]
mod test {
    use super::*;

    mod replace_arm_branch {
        use super::*;

        #[test]
        fn branch_link_before() {
            let result = replace_arm_branch(0x1a8360, 0xeb000a86, 0x14aa9c);
            assert_eq!(result.original_address, 0x1aad80);
            assert_eq!(result.new_branch_instruction, 0xebfe89cd);
        }

        #[test]
        fn branch_link_after() {
            let result = replace_arm_branch(0x1a8360, 0xeb000a86, 0x1a93ac);
            assert_eq!(result.original_address, 0x1aad80);
            assert_eq!(result.new_branch_instruction, 0xeb000411);
        }

        #[test]
        fn ignores_non_branch_instructions() {
            let result = replace_arm_branch(0x10db0c, 0xaabbccdd, 0x1a0000);
            assert_eq!(result.original_address, 0);
            assert_eq!(result.new_branch_instruction, 0xaabbccdd);
        }
    }
}
