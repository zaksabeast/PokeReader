use crate::pkrd::{display, display::Screen, reader, rng};
use ctr::res::CtrResult;

pub fn run_view(
    game: &impl reader::Gen6Reader,
    rng: &rng::Gen6Rng,
    screen: &mut display::DirectWriteScreen,
) -> CtrResult<()> {
    if screen.get_is_top_screen() {
        let mut x = 190;
        let mut y = 10;

        let black = display::Color::black();
        let white = display::Color::white();

        screen.paint_square(&black, x, y, 200, 68)?;

        x += 10;
        y += 4;
        screen.draw_string(&white, "Hello from rust!", x, y)?;

        y += 16;
        let init_seed = game.get_initial_seed()?;
        let seed_text = &alloc::format!("Init seed: {:08x}", init_seed);
        screen.draw_string(&white, seed_text, x, y)?;

        y += 12;
        let mt_state_index = game.get_mt_state_index()?;
        let state_text = &alloc::format!("MT state index: {}", mt_state_index);
        screen.draw_string(&white, state_text, x, y)?;

        y += 12;
        let rng_advances = rng.get_advances();
        let state_text = &alloc::format!("Advances: {}", rng_advances);
        screen.draw_string(&white, state_text, x, y)?;
    }

    Ok(())
}
