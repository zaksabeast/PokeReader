use crate::pkrd::{display, display::Screen, reader};
use ctr::res::CtrResult;

pub fn run_view(
    game: &impl reader::Gen7Reader,
    screen: &mut display::DirectWriteScreen,
) -> CtrResult<()> {
    if screen.get_is_top_screen() {
        let mut x = 190;
        let mut y = 10;

        let black = display::Color::black();
        let white = display::Color::white();

        screen.paint_square(&black, x, y, 200, 56)?;

        x += 10;
        y += 4;
        screen.draw_string(&white, "Hello from rust!", x, y)?;

        y += 12;
        screen.draw_string(&white, "Official Luma", x, y)?;

        y += 12;
        screen.draw_string(&white, "Not NTR", x, y)?;

        y += 16;
        let init_seed = game.get_initial_seed();
        let seed_text = &alloc::format!("Init seed: {:08x}", init_seed);
        screen.draw_string(&white, seed_text, x, y)?;
    }

    Ok(())
}