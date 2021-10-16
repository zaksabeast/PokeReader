use crate::{
    pkrd::{display, display::Screen},
    utils,
};
use alloc::string::ToString;
use ctr::res::CtrResult;
use pkm_rs::pkm;

pub fn run_view(
    title: &str,
    pkx: &impl pkm::Pkx,
    screen: &mut display::DirectWriteScreen,
) -> CtrResult<()> {
    if screen.get_is_top_screen() {
        let mut x = 6;
        let mut y = 10;

        let black = display::Color::black();
        let white = display::Color::white();

        screen.paint_square(&black, x, y, 184, 104)?;

        x += 4;
        y += 4;
        screen.draw_string(&white, title, x, y)?;

        y += 12;
        let species = pkx.species().to_string();
        let species_text = &alloc::format!("Species: {}", utils::string::ellipse(&species, 13));
        screen.draw_string(&white, species_text, x, y)?;

        y += 12;
        let pid_text = &alloc::format!("PID: {:08X}", pkx.pid());
        screen.draw_string(&white, pid_text, x, y)?;

        y += 12;
        let psv_text = &alloc::format!("PSV: {:04}", pkx.psv());
        screen.draw_string(&white, psv_text, x, y)?;

        y += 12;
        let nature_text = &alloc::format!("Nature: {}", pkx.nature().to_string());
        screen.draw_string(&white, nature_text, x, y)?;

        y += 12;
        let ability = pkx.ability().to_string();
        let ability_text = &alloc::format!(
            "Ability: {} ({})",
            utils::string::ellipse(&ability, 9),
            pkx.ability_number()
        );
        screen.draw_string(&white, ability_text, x, y)?;

        y += 12;
        let iv_text = &alloc::format!("IVs: {}", pkx.ivs());
        screen.draw_string(&white, iv_text, x, y)?;

        y += 12;
        let hp_text = &alloc::format!("HPower: {}", pkx.hidden_power());
        screen.draw_string(&white, hp_text, x, y)?;
    }

    Ok(())
}
