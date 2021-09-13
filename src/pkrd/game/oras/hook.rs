use super::reader;
use crate::pkrd::{
    display,
    hook::{HookableProcess, HookedProcess, PatchPresentFramebufferConfig, SupportedTitle},
    rng, views,
};
use alloc::boxed::Box;
use ctr::{res::CtrResult, DebugProcess, Handle};

pub struct PokemonORAS {
    title: SupportedTitle,
    views: views::Gen6Views,
    rng: rng::Gen6Rng,
}

impl HookedProcess for PokemonORAS {
    fn run_hook(&mut self, heap: &[u8], screen: &mut display::DirectWriteScreen) -> CtrResult<()> {
        let game = reader::PokemonORASReader::new(heap);
        views::Gen6Views::run_views(&mut self.views, &game, &mut self.rng, screen)
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
            rng: Default::default(),
        })
    }

    fn install_hook(process: &DebugProcess, pkrd_handle: Handle) -> CtrResult<()> {
        let config = PatchPresentFramebufferConfig {
            is_extended_memory: false,
            get_screen_addr: 0x164ee0,
            present_framebuffer_addr: 0x148758,
            hook_vars_addr: 0x5d0000,
        };
        let inital_seed_address = 0x12e5c8;

        Self::patch_present_framebuffer(process, pkrd_handle, config)?;
        Self::patch_inital_seed(process, inital_seed_address)
    }
}
