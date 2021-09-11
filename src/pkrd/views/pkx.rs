use crate::pkrd::{display, display::Screen, reader::pkm};
use alloc::string::ToString;
use ctr::res::CtrResult;

pub fn run_view(
    title: &str,
    pkx: &impl pkm::Pkx,
    screen: &mut display::DirectWriteScreen,
) -> CtrResult<()> {
    if screen.get_is_top_screen() {
        let mut x = 10;
        let mut y = 10;

        let black = display::Color::black();
        let white = display::Color::white();

        screen.paint_square(&black, x, y, 170, 32)?;

        x += 10;
        y += 4;
        screen.draw_string(&white, title, x, y)?;

        y += 12;
        let species_text = &alloc::format!("Species: {}", pkx.species().to_string());
        screen.draw_string(&white, species_text, x, y)?;
    }

    Ok(())
}
