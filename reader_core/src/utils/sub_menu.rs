use super::CircularCounter;
use crate::pnp;

pub struct SubMenu {
    counter: CircularCounter,
}

impl SubMenu {
    pub fn new(min: usize, max: usize) -> Self {
        Self {
            counter: CircularCounter::new(min, max),
        }
    }

    fn draw_header(&self) {
        pnp::println!("Slot {}", self.counter.value());
        pnp::println!("[v] Next | Prev [^]");
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

    // For Updating when we want a custom header
    pub fn update_headless(&mut self, is_locked: bool) -> usize {
        self.update_counter(is_locked);
        self.counter.value()
    }
}
