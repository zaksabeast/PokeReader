use super::reader;
use crate::pkrd::{display, display::Screen, reader::Gen7Reader};
use ctr::res::CtrResult;

pub(super) fn run(
    game: reader::PokemonSMReader,
    screen: &mut display::DirectWriteScreen,
) -> CtrResult<()> {
    if screen.get_is_top_screen() {
        let init_seed: u32 = game.get_initial_seed()?;

        let black = display::Color::black();
        let white = display::Color::white();

        screen.paint_square(&black, 30, 30, 200, 56)?;
        screen.draw_string(&white, "Hello from rust!", 40, 34)?;
        screen.draw_string(&white, "Official Luma", 40, 46)?;
        screen.draw_string(&white, "Not NTR", 40, 58)?;
        screen.draw_string(
            &white,
            &alloc::format!("Init seed: {:08x}", init_seed),
            40,
            74,
        )?;
    }

    Ok(())
}
