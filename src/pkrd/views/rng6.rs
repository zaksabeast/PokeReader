use crate::pkrd::{display, display::Screen, reader, rng};
use ctr::{
    hid,
    hid::{Button, InterfaceDevice},
    res::CtrResult,
};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Rng6View {
    is_active: bool,
}

impl Rng6View {
    pub fn get_is_active(&mut self) -> bool {
        if hid::Global::is_just_pressed(Button::Start | Button::Dup) {
            self.is_active = !self.is_active;
        }

        self.is_active
    }

    pub fn set_is_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }

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

            screen.paint_square(&black, x, y, 200, 148)?;

            x += 10;
            y += 4;
            screen.draw_string(&white, "Hello from rust!", x, y)?;

            y += 16;
            let init_seed = game.get_initial_seed();
            let seed_text = &alloc::format!("Init seed: {:08X}", init_seed);
            screen.draw_string(&white, seed_text, x, y)?;

            y += 12;
            let mt_state = game.get_mt_state();
            let state_text = &alloc::format!("Curr state: {:08X}", mt_state);
            screen.draw_string(&white, state_text, x, y)?;

            y += 12;
            let mt_advances = rng.get_mt_advances();
            let advances_text = &alloc::format!("MT Advances: {}", mt_advances);
            screen.draw_string(&white, advances_text, x, y)?;

            y += 12;
            let tinymt_advances = rng.get_tinymt_advances();
            let tinymt_advances_text = &alloc::format!("TinyMT Advances: {}", tinymt_advances);
            screen.draw_string(&white, tinymt_advances_text, x, y)?;

            y += 16;
            screen.draw_string(&white, "TinyMT Initial state:", x, y)?;

            y += 12;
            let initial_state = rng.get_initial_tinymt_state();
            let tinymt_text =
                &alloc::format!("[3]{:08X} [2]{:08X}", initial_state[3], initial_state[2]);
            screen.draw_string(&white, tinymt_text, x, y)?;

            y += 12;
            let tinymt_text =
                &alloc::format!("[1]{:08X} [0]{:08X}", initial_state[1], initial_state[0]);
            screen.draw_string(&white, tinymt_text, x, y)?;

            y += 16;
            screen.draw_string(&white, "TinyMT Current state:", x, y)?;

            y += 12;
            let current_state = game.get_tinymt_state();
            let tinymt_text =
                &alloc::format!("[3]{:08X} [2]{:08X}", current_state[3], current_state[2]);
            screen.draw_string(&white, tinymt_text, x, y)?;

            y += 12;
            let tinymt_text =
                &alloc::format!("[1]{:08X} [0]{:08X}", current_state[1], current_state[0]);
            screen.draw_string(&white, tinymt_text, x, y)?;
        }

        Ok(())
    }
}
