use super::sub_menu::SubMenu;
use crate::draw::{draw_controls_help, draw_misc_help};
use crate::pnp;

fn no_game_specific_help() {
    pnp::println!("No Game-Specific info.");
}

enum HelpView {
    Controls,
    SpecificControls,
    Misc,
}

pub struct HelpMenu {
    specific_help: fn() -> (),
    sub_menu: SubMenu,
}

impl Default for HelpMenu {
    fn default() -> Self {
        HelpMenu::new(no_game_specific_help)
    }
}

fn view(slot: usize) -> HelpView {
    match slot {
        1 => HelpView::Controls,
        2 => HelpView::SpecificControls,
        3 => HelpView::Misc,
        _ => HelpView::Misc,
    }
}

fn print_view(help_view: &HelpView) {
    match help_view {
        HelpView::Controls => pnp::println!("Controls"),
        HelpView::SpecificControls => pnp::println!("Game-Specific Controls"),
        HelpView::Misc => pnp::println!("Additional Info"),
    }
}

impl HelpMenu {
    pub fn new(specific_help: fn() -> ()) -> Self {
        Self {
            specific_help,
            sub_menu: SubMenu::new(1, 3),
        }
    }

    pub fn update_and_draw(&mut self, is_locked: bool) {
        let help_view = view(self.sub_menu.update_headless(is_locked));
        print_view(&help_view);
        pnp::println!("[v] Next | Prev [^]");
        pnp::println!("");

        match help_view {
            HelpView::Controls => draw_controls_help(),
            HelpView::SpecificControls => (self.specific_help)(),
            HelpView::Misc => draw_misc_help(),
        }
    }
}
