use super::CircularCounter;
use crate::pnp;

#[derive(Default)]
pub struct SubMenu<const MIN: usize, const MAX: usize> {
    counter: CircularCounter<MIN, MAX>,
}

impl<const MIN: usize, const MAX: usize> SubMenu<MIN, MAX> {
    fn draw_header(&self) {
        pnp::println!("Slot {}", self.counter.value());
        pnp::println!("^ Previous slot");
        pnp::println!("v Next slot");
        pnp::println!("");
    }

    fn update_counter(&mut self, is_locked: bool) {
        if is_locked {
            return;
        }

        if pnp::is_just_pressed(pnp::Button::Ddown) {
            self.counter.increment();
        } else if pnp::is_just_pressed(pnp::Button::Dup) {
            self.counter.decrement();
        }
    }

    pub fn update_and_draw(&mut self, is_locked: bool) -> usize {
        self.update_counter(is_locked);
        self.draw_header();
        self.counter.value()
    }
}
