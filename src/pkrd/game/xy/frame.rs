use super::{reader, PokemonXY};
use crate::pkrd::{display, display::Screen, reader::Gen6Reader};
use ctr::{hid, hid::InterfaceDevice, res::CtrResult};

fn run_main_screen(
    game: &reader::PokemonXYReader,
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

pub(super) fn run(
    context: &mut PokemonXY,
    game: reader::PokemonXYReader,
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
