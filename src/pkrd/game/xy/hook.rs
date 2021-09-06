use super::{frame, reader};
use crate::pkrd::{
    display,
    hook::{HookableProcess, HookedProcess, SupportedTitle},
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
        Self::patch_present_framebuffer(
            process,
            pkrd_handle,
            0x8000000,
            0x6000000,
            0x149354,
            0x5c0000,
            0x1646b4,
        )
    }
}
