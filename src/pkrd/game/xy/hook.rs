use super::{frame, reader};
use crate::pkrd::{
    display,
    hook::{HookableProcess, HookedProcess, PatchPresentFramebufferConfig, SupportedTitle},
    reader::Reader,
};
use alloc::boxed::Box;
use ctr::{res::CtrResult, DebugProcess, Handle};

pub struct PokemonXY {
    title: SupportedTitle,
}

impl HookedProcess for PokemonXY {
    fn run_hook(&self, heap: Reader, screen: &mut display::DirectWriteScreen) -> CtrResult<()> {
        let game_reader = reader::PokemonXYReader::new(heap);
        frame::run(game_reader, screen)
    }

    fn get_title(&self) -> SupportedTitle {
        self.title
    }
}

impl HookableProcess for PokemonXY {
    fn new_from_supported_title(title: SupportedTitle) -> Box<Self> {
        Box::new(Self { title })
    }

    fn install_hook(process: &DebugProcess, pkrd_handle: Handle) -> CtrResult<()> {
        let config = PatchPresentFramebufferConfig {
            is_extended_memory: false,
            get_screen_addr: 0x1646b4,
            present_framebuffer_addr: 0x149354,
            hook_vars_addr: 0x5c0000,
        };
        Self::patch_present_framebuffer(process, pkrd_handle, config)
    }
}
