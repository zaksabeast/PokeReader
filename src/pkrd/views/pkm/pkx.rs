use crate::{
    pkrd::{display, display::Screen, views::view},
    utils,
};
use alloc::string::ToString;
use ctr::res::CtrResult;
use pkm_rs::pkm;

pub fn draw(
    screen: &mut display::DirectWriteScreen,
    title: &str,
    pkx: &impl pkm::Pkx,
) -> CtrResult<()> {
    if screen.get_is_top_screen() {
        let species = pkx.species().to_string();
        let ability = pkx.ability().to_string();

        view::draw_left(
            screen,
            title,
            &[
                &alloc::format!("Species: {}", utils::string::ellipse(&species, 13)),
                &alloc::format!("PID: {:08X}", pkx.pid()),
                &alloc::format!("PSV: {:04}", pkx.psv()),
                &alloc::format!("Nature: {}", pkx.nature()),
                &alloc::format!(
                    "Ability: {} ({})",
                    utils::string::ellipse(&ability, 9),
                    pkx.ability_number()
                ),
                &alloc::format!("IVs: {}", pkx.ivs()),
                &alloc::format!("HPower: {}", pkx.hidden_power()),
            ],
        )?;
    }

    Ok(())
}
