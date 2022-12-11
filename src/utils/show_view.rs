use pnp::{is_just_pressed, Button};

// This is abstracted away for consistency.
// Some logic needs to run per frame, regardless of a screen showing.
// This abstraction lets each game decide when to check showing a view.
pub struct ShowView {
    show_view: bool,
}

impl Default for ShowView {
    fn default() -> Self {
        Self { show_view: true }
    }
}

impl ShowView {
    pub fn check(&mut self) -> bool {
        if is_just_pressed(Button::Start | Button::Dup) {
            self.show_view = !self.show_view;
        }

        self.show_view
    }
}
