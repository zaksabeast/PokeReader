use super::{frame, reader};
use crate::pkrd::{
    display,
    hook::{HookableProcess, HookedProcess, PatchPresentFramebufferConfig, SupportedTitle},
};
use alloc::boxed::Box;
use ctr::{res::CtrResult, DebugProcess, Handle};

pub(super) struct Views {
    pub main: bool,
}

pub struct PokemonSM {
    title: SupportedTitle,
    pub(super) views: Views,
}

impl HookedProcess for PokemonSM {
    fn run_hook(&mut self, heap: &[u8], screen: &mut display::DirectWriteScreen) -> CtrResult<()> {
        let game_reader = reader::PokemonSMReader::new(heap);
        frame::run(self, game_reader, screen)
    }

    fn get_title(&self) -> SupportedTitle {
        self.title
    }
}

impl HookableProcess for PokemonSM {
    fn new_from_supported_title(title: SupportedTitle) -> Box<Self> {
        Box::new(Self {
            title,
            views: Views { main: false },
        })
    }

    fn install_hook(process: &DebugProcess, pkrd_handle: Handle) -> CtrResult<()> {
        let config = PatchPresentFramebufferConfig {
            is_extended_memory: true,
            get_screen_addr: 0x2794c4,
            present_framebuffer_addr: 0x278540,
            hook_vars_addr: 0x600000,
        };
        Self::patch_present_framebuffer(process, pkrd_handle, config)
    }
}
