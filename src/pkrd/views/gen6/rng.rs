use crate::pkrd::{display, display::Screen, reader, rng, views::view};
use ctr::res::CtrResult;

pub mod input {
    use ctr::hid::{Button, Global, InterfaceDevice};

    pub fn toggle() -> bool {
        Global::is_just_pressed(Button::Start | Button::Dup)
    }
}

pub fn draw(
    screen: &mut display::DirectWriteScreen,
    game: &impl reader::Gen6Reader,
    rng: &rng::Gen6Rng,
) -> CtrResult<()> {
    if screen.get_is_top_screen() {
        let init_seed = game.get_initial_seed();
        let mt_state = game.get_mt_state();
        let mt_advances = rng.get_mt_advances();
        let tinymt_advances = rng.get_tinymt_advances();
        let initial_state = rng.get_initial_tinymt_state();
        let current_state = game.get_tinymt_state();

        view::draw_top_right(
            screen,
            "Main RNG View",
            &[
                &alloc::format!("Init seed: {:08X}", init_seed),
                &alloc::format!("Curr state: {:08X}", mt_state),
                &alloc::format!("MT Advances: {}", mt_advances),
                &alloc::format!("TinyMT Advances: {}", tinymt_advances),
                &alloc::format!("[3]{:08X} [2]{:08X}", initial_state[3], initial_state[2]),
                &alloc::format!("[1]{:08X} [0]{:08X}", initial_state[1], initial_state[0]),
                &alloc::format!("[3]{:08X} [2]{:08X}", current_state[3], current_state[2]),
                &alloc::format!("[1]{:08X} [0]{:08X}", current_state[1], current_state[0]),
            ],
        )?;
    }

    Ok(())
}
