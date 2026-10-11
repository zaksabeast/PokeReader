use super::CircularCounter;
use crate::ctr;
use crate::ctr::hid::{self, Button};

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
        ctr::println!("Slot {}", self.counter.value());
        ctr::println!("[v] Next | Prev [^]");
        ctr::println!("");
    }

    fn update_counter(&mut self, is_locked: bool) {
        if is_locked {
            return;
        }

        if hid::is_just_pressed(Button::Ddown) {
            self.counter.increment();
        } else if hid::is_just_pressed(Button::Dup) {
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
