use crate::pkrd::{display, display::Screen, reader};
use alloc::string::String;
use ctr::{
    hid,
    hid::{Button, InterfaceDevice},
    res::CtrResult,
};
use pkm_rs::pkm;

fn is_masuda_method(parent1: &impl pkm::Pkx, parent2: &impl pkm::Pkx) -> bool {
    parent1.language() != parent2.language()
        && parent1.language() != pkm::Language::Invalid
        && parent2.language() != pkm::Language::Invalid
}

fn is_daycare_masuda_method(
    parent1: &Option<impl pkm::Pkx>,
    parent2: &Option<impl pkm::Pkx>,
) -> bool {
    match (parent1, parent2) {
        (Some(inner1), Some(inner2)) => is_masuda_method(inner1, inner2),
        (_, _) => false,
    }
}

fn format_egg_parent(parent_num: u8, parent: &Option<impl pkm::Pkx>) -> String {
    let formatted_parent = match parent {
        Some(parent) => alloc::format!(
            "Par{}: {} {}",
            parent_num,
            parent.species(),
            parent.gender_ratio()
        ),
        None => alloc::format!("Par{}: {}", parent_num, pkm::Species::None),
    };

    formatted_parent
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Daycare7View {
    is_active: bool,
}

impl Daycare7View {
    pub fn get_is_active(&mut self) -> bool {
        if hid::Global::is_just_pressed(Button::Start | Button::Ddown) {
            self.is_active = !self.is_active;
        }

        self.is_active
    }

    pub fn set_is_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }

    pub fn run_view(
        game: &impl reader::Gen7Reader,
        screen: &mut display::DirectWriteScreen,
    ) -> CtrResult<()> {
        if screen.get_is_top_screen() {
            let mut x = 200;
            let mut y = 10;

            let black = display::Color::black();
            let white = display::Color::white();

            screen.paint_square(&black, x, y, 192, 136)?;

            x += 4;
            y += 4;
            screen.draw_string(&white, "Daycare View", x, y)?;

            y += 16;
            let is_egg_ready = game.get_is_egg_ready();
            screen.draw_string(&white, &alloc::format!("Egg Ready: {}", is_egg_ready), x, y)?;

            let parent1 = game.get_egg_parent_1();
            let parent2 = game.get_egg_parent_2();

            y += 12;
            let parent1_text = format_egg_parent(1, &parent1);
            screen.draw_string(&white, &parent1_text, x, y)?;

            y += 12;
            let parent2_text = format_egg_parent(2, &parent2);
            screen.draw_string(&white, &parent2_text, x, y)?;

            y += 16;
            let egg_seed = game.get_egg_seed();
            screen.draw_string(&white, &alloc::format!("Egg[0]: {:08X}", egg_seed[3]), x, y)?;

            y += 12;
            screen.draw_string(&white, &alloc::format!("Egg[1]: {:08X}", egg_seed[2]), x, y)?;

            y += 12;
            screen.draw_string(&white, &alloc::format!("Egg[2]: {:08X}", egg_seed[1]), x, y)?;

            y += 12;
            screen.draw_string(&white, &alloc::format!("Egg[3]: {:08X}", egg_seed[0]), x, y)?;

            y += 16;
            let shiny_charm_text = &alloc::format!("Shiny Charm: {}", game.get_has_shiny_charm());
            screen.draw_string(&white, shiny_charm_text, x, y)?;

            y += 12;
            let is_masuda_method = is_daycare_masuda_method(&parent1, &parent2);
            let masuda_method_text = &alloc::format!("Masuda Method: {}", is_masuda_method);
            screen.draw_string(&white, masuda_method_text, x, y)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Daycare6View {
    is_active: bool,
}

impl Daycare6View {
    pub fn get_is_active(&mut self) -> bool {
        if hid::Global::is_just_pressed(Button::Start | Button::Ddown) {
            self.is_active = !self.is_active;
        }

        self.is_active
    }

    pub fn set_is_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }

    pub fn run_view(
        game: &impl reader::Gen6Reader,
        screen: &mut display::DirectWriteScreen,
    ) -> CtrResult<()> {
        if screen.get_is_top_screen() {
            let mut x = 200;
            let mut y = 10;

            let black = display::Color::black();
            let white = display::Color::white();

            screen.paint_square(&black, x, y, 192, 136)?;

            x += 4;
            y += 4;
            screen.draw_string(&white, "Daycare View", x, y)?;

            y += 16;
            screen.draw_string(&white, "Route 117", x, y)?;

            y += 12;
            let is_egg_ready = game.get_is_egg_ready();
            screen.draw_string(&white, &alloc::format!("Egg Ready: {}", is_egg_ready), x, y)?;

            let parent1 = game.get_egg_parent_1();
            let parent2 = game.get_egg_parent_2();

            y += 12;
            let parent1_text = format_egg_parent(1, &parent1);
            screen.draw_string(&white, &parent1_text, x, y)?;

            y += 12;
            let parent2_text = format_egg_parent(2, &parent2);
            screen.draw_string(&white, &parent2_text, x, y)?;

            y += 16;
            let egg_seed = game.get_egg_seed();
            screen.draw_string(&white, &alloc::format!("Egg[0]: {:08X}", egg_seed[1]), x, y)?;

            y += 12;
            screen.draw_string(&white, &alloc::format!("Egg[1]: {:08X}", egg_seed[0]), x, y)?;

            y += 12;
            let is_masuda_method = is_daycare_masuda_method(&parent1, &parent2);
            let masuda_method_text = &alloc::format!("Masuda Method: {}", is_masuda_method);
            screen.draw_string(&white, masuda_method_text, x, y)?;
        }

        Ok(())
    }
}
