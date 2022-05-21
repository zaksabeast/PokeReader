use super::reader;
use crate::pkrd::{
    display,
    hook::{HookableProcess, HookedProcess, PatchPresentFramebufferConfig, SupportedTitle},
    rng, views,
};
use alloc::boxed::Box;
use ctr::{res::CtrResult, DebugProcess, Handle};

pub struct PokemonXY {
    title: SupportedTitle,
    views: views::gen6::Gen6Views,
    rng: rng::Gen6Rng,
    reader: reader::PokemonXYReader,
}

impl HookedProcess for PokemonXY {
    fn run_hook(&mut self, screen: &mut display::DirectWriteScreen) -> CtrResult<()> {
        self.views.run_views(screen, &self.reader, &mut self.rng)
    }

    fn get_title(&self) -> SupportedTitle {
        self.title
    }
}

impl HookableProcess for PokemonXY {
    fn new_from_supported_title(title: SupportedTitle, heap: &'static [u8]) -> Box<Self> {
        Box::new(Self {
            title,
            views: Default::default(),
            rng: Default::default(),
            reader: reader::PokemonXYReader::new(heap),
        })
    }

    fn install_hook(process: &DebugProcess, pkrd_handle: Handle) -> CtrResult<()> {
        let config = PatchPresentFramebufferConfig {
            is_extended_memory: false,
            get_screen_addr: 0x1646b4,
            present_framebuffer_addr: 0x149354,
            hook_vars_addr: 0x5c0000,
        };
        let inital_seed_address = 0x1254f8;

        Self::patch_present_framebuffer(process, pkrd_handle, config)?;
        Self::patch_inital_seed(process, inital_seed_address)
    }
}
