use crate::pkrd::{display, display::Screen, reader, rng, views::view};
use ctr::{res::CtrResult, safe_transmute};

pub mod input {
    use ctr::hid::{Button, Global, InterfaceDevice};

    pub fn toggle() -> bool {
        Global::is_just_pressed(Button::Start | Button::Dup)
    }
}

pub fn draw(
    screen: &mut display::DirectWriteScreen,
    game: &impl reader::Gen7Reader,
    rng: &rng::Gen7Rng,
) -> CtrResult<()> {
    if screen.get_is_top_screen() {
        let init_seed = game.get_initial_seed();
        let sfmt_state = game.get_sfmt_state();
        let sfmt_state_bytes = sfmt_state.to_ne_bytes();
        let sfmt_state_parts: [u32; 2] = safe_transmute::transmute_one_pedantic(&sfmt_state_bytes)?;
        let sfmt_advances = rng.get_sfmt_advances();

        view::draw_top_right(
            screen,
            "Main RNG View",
            &[
                &alloc::format!("Init seed: {:08X}", init_seed),
                &alloc::format!("Curr state[1]: {:08X}", sfmt_state_parts[1]),
                &alloc::format!("Curr state[0]: {:08X}", sfmt_state_parts[0]),
                &alloc::format!("Advances: {}", sfmt_advances),
            ],
        )?;
    }

    Ok(())
}
