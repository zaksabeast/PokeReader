use super::reader;
use crate::pkrd::{
    display,
    hook::{HookableProcess, HookedProcess, PatchPresentFramebufferConfig, SupportedTitle},
    views,
};
use alloc::boxed::Box;
use ctr::{res::CtrResult, DebugProcess, Handle};

pub struct PokemonUSUM {
    title: SupportedTitle,
    views: views::Gen7Views,
    reader: reader::PokemonUSUMReader,
}

impl HookedProcess for PokemonUSUM {
    fn run_hook(&mut self, screen: &mut display::DirectWriteScreen) -> CtrResult<()> {
        views::Gen7Views::run_views(&mut self.views, &self.reader, screen)
    }

    fn get_title(&self) -> SupportedTitle {
        self.title
    }
}

impl HookableProcess for PokemonUSUM {
    fn new_from_supported_title(title: SupportedTitle, heap: &'static [u8]) -> Box<Self> {
        Box::new(Self {
            title,
            views: Default::default(),
            reader: reader::PokemonUSUMReader::new(heap),
        })
    }

    fn install_hook(process: &DebugProcess, pkrd_handle: Handle) -> CtrResult<()> {
        let config = PatchPresentFramebufferConfig {
            is_extended_memory: true,
            get_screen_addr: 0x27ab38,
            present_framebuffer_addr: 0x279bb4,
            hook_vars_addr: 0x630000,
        };
        Self::patch_present_framebuffer(process, pkrd_handle, config)
    }
}
