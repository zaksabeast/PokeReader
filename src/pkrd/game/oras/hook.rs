use super::reader;
use crate::pkrd::{
    display,
    hook::{HookableProcess, HookedProcess, PatchPresentFramebufferConfig, SupportedTitle},
    views,
};
use alloc::boxed::Box;
use ctr::{res::CtrResult, DebugProcess, Handle};

pub struct PokemonORAS {
    title: SupportedTitle,
    views: views::Views,
}

impl HookedProcess for PokemonORAS {
    fn run_hook(&mut self, heap: &[u8], screen: &mut display::DirectWriteScreen) -> CtrResult<()> {
        let game = reader::PokemonORASReader::new(heap);
        views::run_gen6_views(&mut self.views, &game, screen)
    }

    fn get_title(&self) -> SupportedTitle {
        self.title
    }
}

impl HookableProcess for PokemonORAS {
    fn new_from_supported_title(title: SupportedTitle) -> Box<Self> {
        Box::new(Self {
            title,
            views: Default::default(),
        })
    }

    fn install_hook(process: &DebugProcess, pkrd_handle: Handle) -> CtrResult<()> {
        let config = PatchPresentFramebufferConfig {
            is_extended_memory: false,
            get_screen_addr: 0x164ee0,
            present_framebuffer_addr: 0x148758,
            hook_vars_addr: 0x5d0000,
        };
        Self::patch_present_framebuffer(process, pkrd_handle, config)
    }
}
