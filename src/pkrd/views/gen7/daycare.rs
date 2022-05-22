use crate::pkrd::{display, display::Screen, reader, views::view};
use crate::utils::daycare;
use ctr::res::CtrResult;

pub mod input {
    use ctr::hid::{Button, Global, InterfaceDevice};

    pub fn toggle() -> bool {
        Global::is_just_pressed(Button::Start | Button::Ddown)
    }
}

pub fn draw(
    screen: &mut display::DirectWriteScreen,
    game: &impl reader::Gen7Reader,
) -> CtrResult<()> {
    if screen.get_is_top_screen() {
        let is_egg_ready = game.get_is_egg_ready();
        let parent1 = game.get_egg_parent_1();
        let parent2 = game.get_egg_parent_2();
        let egg_seed = game.get_egg_seed();
        let has_shiny_charm = game.get_has_shiny_charm();
        let is_masuda_method = daycare::is_daycare_masuda_method(&parent1, &parent2);

        view::draw_right(
            screen,
            "Daycare View",
            &[
                &alloc::format!("Egg Ready: {}", is_egg_ready),
                &daycare::format_egg_parent(1, &parent1),
                &daycare::format_egg_parent(2, &parent2),
                "",
                &alloc::format!("Egg[0]: {:08X}", egg_seed[3]),
                &alloc::format!("Egg[1]: {:08X}", egg_seed[2]),
                &alloc::format!("Egg[2]: {:08X}", egg_seed[1]),
                &alloc::format!("Egg[3]: {:08X}", egg_seed[0]),
                "",
                &alloc::format!("Shiny Charm: {}", has_shiny_charm),
                &alloc::format!("Masuda Method: {}", is_masuda_method),
            ],
        )?;
    }

    Ok(())
}
