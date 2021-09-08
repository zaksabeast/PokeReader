use super::{reader, PokemonUSUM};
use crate::pkrd::{display, display::Screen, reader::Gen7Reader};
use ctr::{hid, hid::InterfaceDevice, res::CtrResult};

fn run_main_screen(
    game: &reader::PokemonUSUMReader,
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

pub(super) fn run(
    context: &mut PokemonUSUM,
    game: reader::PokemonUSUMReader,
    screen: &mut display::DirectWriteScreen,
) -> CtrResult<()> {
    let mut views = &mut context.views;

    if hid::Global::is_just_pressed(hid::Button::Start | hid::Button::Dup) {
        views.main = !views.main;
    }

    if views.main {
        run_main_screen(&game, screen)?;
    }

    Ok(())
}
