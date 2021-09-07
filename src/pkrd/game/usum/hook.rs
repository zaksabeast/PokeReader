use super::{frame, reader};
use crate::pkrd::{
    display,
    hook::{HookableProcess, HookedProcess, PatchPresentFramebufferConfig, SupportedTitle},
    reader::Reader,
};
use alloc::boxed::Box;
use ctr::{res::CtrResult, DebugProcess, Handle};

pub struct PokemonUSUM {
    title: SupportedTitle,
}

impl HookedProcess for PokemonUSUM {
    fn run_hook(&self, heap: Reader, screen: &mut display::DirectWriteScreen) -> CtrResult<()> {
        let game_reader = reader::PokemonUSUMReader::new(heap);
        frame::run(game_reader, screen)
    }

    fn get_title(&self) -> SupportedTitle {
        self.title
    }
}

impl HookableProcess for PokemonUSUM {
    fn new_from_supported_title(title: SupportedTitle) -> Box<Self> {
        Box::new(Self { title })
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
