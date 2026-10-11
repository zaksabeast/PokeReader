use super::CircularCounter;
use crate::ctr::hid::{self, Button};

pub struct SubMenuCapture {
    counter: CircularCounter,
    value: u32,
}

impl SubMenuCapture {
    pub fn new(min: usize, max: usize) -> Self {
        Self {
            counter: CircularCounter::new(min, max),
            value: 0,
        }
    }

    fn update_counter(&mut self, is_locked: bool, capture_value: u32, set_value: usize) {
        if is_locked {
            return;
        }

        if hid::is_just_pressed(Button::Ddown | Button::X) {
            self.counter.increment();
            self.value = capture_value;
        } else if hid::is_just_pressed(Button::Dup | Button::X) {
            self.counter.decrement();
            self.value = capture_value;
        } else if hid::is_just_pressed(Button::Dright | Button::X) {
            self.counter.set(set_value);
            self.value = capture_value;
        }
    }

    pub fn update_headless(&mut self, is_locked: bool, capture_value: u32, set_value: usize) -> usize {
        self.update_counter(is_locked, capture_value, set_value);
        self.counter.value()
    }

    pub fn captured_value(&mut self) -> u32 {
        self.value
    }

    pub fn counter_value(&self) -> usize {
        self.counter.value()
    }
}
