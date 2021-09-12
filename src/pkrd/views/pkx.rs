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

        screen.paint_square(&black, x, y, 190, 80)?;

        x += 10;
        y += 4;
        screen.draw_string(&white, title, x, y)?;

        y += 12;
        let species_text = &alloc::format!("Species: {}", pkx.species().to_string());
        screen.draw_string(&white, species_text, x, y)?;

        y += 12;
        let pid_text = &alloc::format!("PID: {:08X}", pkx.pid());
        screen.draw_string(&white, pid_text, x, y)?;

        y += 12;
        let tsv_text = &alloc::format!("TSV: {:04}", pkx.tsv());
        screen.draw_string(&white, tsv_text, x, y)?;

        y += 12;
        let nature_text = &alloc::format!("Nature: {}", pkx.nature().to_string());
        screen.draw_string(&white, nature_text, x, y)?;

        y += 12;
        let ability_text = &alloc::format!(
            "Ability: {} ({})",
            pkx.ability().to_string(),
            pkx.ability_number()
        );
        screen.draw_string(&white, ability_text, x, y)?;
    }

    Ok(())
}
