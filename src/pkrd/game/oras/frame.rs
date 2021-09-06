use super::reader;
use crate::pkrd::{display, display::Screen};
use ctr::res::CtrResult;

pub(super) fn run(
    game: reader::PokemonORASReader,
    screen: &mut display::DirectWriteScreen,
) -> CtrResult<()> {
    if screen.get_is_top_screen() {
        let mt_state_index = game.get_mt_state_index()?;

        let black = display::Color::black();
        let white = display::Color::white();

        screen.paint_square(&black, 30, 30, 200, 32)?;
        screen.draw_string(&white, "Hello from rust!", 40, 34)?;
        screen.draw_string(
            &white,
            &alloc::format!("MT state index {}", mt_state_index),
            40,
            50,
        )?;
    }

    Ok(())
}
