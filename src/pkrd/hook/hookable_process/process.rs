use super::config;
use crate::{
    pkrd::{display, game, hook::SupportedTitle, request_handler::get_pkrd_session_handle},
    utils,
};
use alloc::boxed::Box;
use ctr::{
    res::{CtrResult, GenericResultCode},
    DebugProcess, Handle,
};

/// A process that has the ability to be hooked.
pub trait HookableProcess: HookedProcess {
    fn new_from_supported_title(title: SupportedTitle) -> Box<Self>;

    fn install_hook(process: &DebugProcess, pkrd_handle: Handle) -> CtrResult<()>;

    // Same place NTR patches
    fn patch_present_framebuffer(
        process: &DebugProcess,
        pkrd_handle: Handle,
        config: config::PatchPresentFramebufferConfig,
    ) -> CtrResult<()> {
        if (config.hook_vars_addr & 0xFFFF) != 0 || config.hook_vars_addr == 0 {
            return Err(GenericResultCode::InvalidPointer.into());
        }

        let cmd_header = 0x20180;
        let load_hook_vars_into_lr = 0xe3a0e800 | (config.hook_vars_addr >> 16);
        let get_screen_branch = utils::make_arm_branch(
            config.present_framebuffer_addr + (14 * 4),
            config.get_screen_addr,
        );

        let hook_code: [u32; 37] = [
            0xe92d5fff,             // stmdb      sp!,{r0-lr}
            0xe8bd00c0,             // ldmia      sp!,{r6, r7}
            0xe8bd0030,             // ldmia      sp!,{r4, r5}
            0xe28d0028,             // add        r0,sp,#0x28
            0xe8900e00,             // ldmia      r0,{r9, r10, r11}
            load_hook_vars_into_lr, // mov        lr,hook_vars
            0xee1d8f70,             // mrc        p15,0x0,r8,cr13,cr0,0x3
            0xe8be0003,             // ldmia      lr!,{r0, r1}
            0xef000090,             // swi        0x90
            0xe1a0c000,             // cpy        r12,r0
            0xe2882080,             // add        r2,r8,#0x80
            0xe89e4109,             // ldmia      lr,{r0, r3, r8, lr}
            0xe8825658,             // stmia      r2,{r3, r4, r6, r9, r10, r12, lr}
            0xef000032,             // swi        0x32
            get_screen_branch,      // bl         get_screen
            0xe280105c,             // add        r1,r0,#0x5c
            0xe7912106,             // ldr        r2,[r1,r6,lsl #0x2]
            0xe3a03004,             // mov        r3,#0x4
            0xe5d20000,             // ldrb       r0,[r2,#0x0]
            0xe2600001,             // rsb        r0,r0,#0x1
            0xe20000ff,             // and        r0,r0,#0xff
            0xe060e180,             // rsb        lr,r0,r0, lsl #0x3
            0xe083310e,             // add        r3,r3,lr, lsl #0x2
            0xe7a27003,             // str        r7,[r2,r3]!
            0xe2823004,             // add        r3,r2,#0x4
            0xe8830e30,             // stmia      r3,{r4, r5, r9, r10, r11}
            0xee078f9a,             // mcr        p15,0x0,r8,cr7,cr10,0x4
            0xe7912106,             // ldr        r2,[r1,r6,lsl #0x2]
            0xe1923f9f,             // ldrex      r3,[r2]
            0xe3c330ff,             // bic        r3,r3,#0xff
            0xe1833000,             // orr        r3,r3,r0
            0xe3c33cff,             // bic        r3,r3,#0xff00
            0xe3833c01,             // orr        r3,r3,#0x100
            0xe1824f93,             // strex      r4,r3,[r2]
            0xe3540000,             // cmp        r4,#0x0
            0x1afffff6,             // bne        LAB_00279c20
            0xe8bd9ff0,             // ldmia      sp!,{r4-r12, pc}
        ];

        let hook_vars: [u32; 6] = [
            config.get_heap_addr(),           // Heap
            1,                                // svc::convert_va_to_pa write_check
            unsafe { pkrd_handle.get_raw() }, // Session handle
            cmd_header,                       // Command header
            0,                                // Needed for data sync
            config.get_heap_size(),           // Heap size
        ];

        process
            .write_bytes(
                config.present_framebuffer_addr,
                safe_transmute::transmute_to_bytes(&hook_code),
            )
            .unwrap();
        process
            .write_bytes(
                config.hook_vars_addr,
                safe_transmute::transmute_to_bytes(&hook_vars),
            )
            .unwrap();

        Ok(())
    }

    fn patch_inital_seed(process: &DebugProcess, address: u32) -> CtrResult<()> {
        /*
         * The MT table initialization in gen 6 has a very useful nop instruction at the beginning of the function
         * To be more specific it is a mov r0, r0 instruction
         * We overwrite this with str r1, [r0, #-4]
         * r1 is the register that contains the initial seed and r0 is the register that contains the memory address for the MT table
         * The #-4 is to indicate write the initial seed 4 bytes before the MT table
         * After this instruction is executed we can read the memory address 4 bytes before the MT table to get the initial seed
         */
        let patch_code: [u32; 1] = [
            0xe5001004, // str r1, [r0, #-4]
        ];

        process
            .write_bytes(address, safe_transmute::transmute_to_bytes(&patch_code))
            .unwrap();

        Ok(())
    }
}

/// A process that is hooked.
/// This is separate from HookableProcess so it can have a vtable
/// and be used as `dyn HookedProcess`.
pub trait HookedProcess {
    fn run_hook(&mut self, heap: &[u8], screen: &mut display::DirectWriteScreen) -> CtrResult<()>;

    fn get_title(&self) -> SupportedTitle;
}

pub fn get_hooked_process() -> Option<Box<dyn HookedProcess>> {
    let running_app = SupportedTitle::from_running_app().unwrap();

    let hookable_process: Box<dyn HookedProcess> = match running_app {
        SupportedTitle::PokemonX => game::PokemonXY::new_from_supported_title(running_app),
        SupportedTitle::PokemonY => game::PokemonXY::new_from_supported_title(running_app),
        SupportedTitle::PokemonOR => game::PokemonORAS::new_from_supported_title(running_app),
        SupportedTitle::PokemonAS => game::PokemonORAS::new_from_supported_title(running_app),
        SupportedTitle::PokemonS => game::PokemonSM::new_from_supported_title(running_app),
        SupportedTitle::PokemonM => game::PokemonSM::new_from_supported_title(running_app),
        SupportedTitle::PokemonUS => game::PokemonUSUM::new_from_supported_title(running_app),
        SupportedTitle::PokemonUM => game::PokemonUSUM::new_from_supported_title(running_app),
    };

    Some(hookable_process)
}

pub fn install_hook(title: SupportedTitle) -> CtrResult<()> {
    let debug = DebugProcess::new(title.into()).unwrap();
    let process = debug.get_process();
    let pkrd_session_handle = get_pkrd_session_handle();
    let handle_copy = process
        .copy_handle_to_process(&pkrd_session_handle)
        .unwrap();

    match title {
        SupportedTitle::PokemonX => game::PokemonXY::install_hook(&debug, handle_copy),
        SupportedTitle::PokemonY => game::PokemonXY::install_hook(&debug, handle_copy),
        SupportedTitle::PokemonOR => game::PokemonORAS::install_hook(&debug, handle_copy),
        SupportedTitle::PokemonAS => game::PokemonORAS::install_hook(&debug, handle_copy),
        SupportedTitle::PokemonS => game::PokemonSM::install_hook(&debug, handle_copy),
        SupportedTitle::PokemonM => game::PokemonSM::install_hook(&debug, handle_copy),
        SupportedTitle::PokemonUS => game::PokemonUSUM::install_hook(&debug, handle_copy),
        SupportedTitle::PokemonUM => game::PokemonUSUM::install_hook(&debug, handle_copy),
    }
    .unwrap();

    debug.eat_events().unwrap();

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    struct MockGame {}

    impl HookableProcess for MockGame {
        fn new_from_supported_title(_title: SupportedTitle) -> Box<Self> {
            Box::new(Self {})
        }

        fn install_hook(_process: &DebugProcess, _pkrd_handle: Handle) -> CtrResult<()> {
            Ok(())
        }
    }

    impl HookedProcess for MockGame {
        fn run_hook(
            &mut self,
            _heap: &[u8],
            _screen: &mut display::DirectWriteScreen,
        ) -> CtrResult<()> {
            Ok(())
        }

        fn get_title(&self) -> SupportedTitle {
            SupportedTitle::PokemonUM
        }
    }

    mod hookable_process {
        use super::*;
        use crate::pkrd::hook;

        #[test]
        fn should_return_error_if_the_config_hook_vars_addr_is_not_0x10000_aligned() {
            let debug_process = DebugProcess::new(0).unwrap();
            let session_handle = 0.into();
            let config = hook::PatchPresentFramebufferConfig {
                is_extended_memory: true,
                get_screen_addr: 0x27ab38,
                present_framebuffer_addr: 0x279bb4,
                hook_vars_addr: 0x6300FF,
            };

            let result =
                MockGame::patch_present_framebuffer(&debug_process, session_handle, config)
                    .expect_err(
                        "Patching framebuffer should have failed for unaligned hook var address",
                    );

            assert_eq!(result, GenericResultCode::InvalidPointer.into());
        }

        #[test]
        fn should_return_error_if_the_config_hook_vars_addr_is_0() {
            let debug_process = DebugProcess::new(0).unwrap();
            let session_handle = 0.into();
            let config = hook::PatchPresentFramebufferConfig {
                is_extended_memory: true,
                get_screen_addr: 0x27ab38,
                present_framebuffer_addr: 0x279bb4,
                hook_vars_addr: 0,
            };

            let result =
                MockGame::patch_present_framebuffer(&debug_process, session_handle, config)
                    .expect_err(
                        "Patching framebuffer should have failed for null hook var address",
                    );

            assert_eq!(result, GenericResultCode::InvalidPointer.into());
        }

        #[test]
        fn should_succeed_if_the_config_hook_vars_addr_is_not_0_or_is_0x10000_aligned() {
            let debug_process = DebugProcess::new(0).unwrap();
            let session_handle = 0.into();
            let config = hook::PatchPresentFramebufferConfig {
                is_extended_memory: true,
                get_screen_addr: 0x27ab38,
                present_framebuffer_addr: 0x279bb4,
                hook_vars_addr: 0x630000,
            };

            let result =
                MockGame::patch_present_framebuffer(&debug_process, session_handle, config);

            assert_eq!(result, Ok(()));
        }
    }
}
