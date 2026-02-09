use crate::{draw::draw_version, pnp, pnp::Button, utils::CircularCounter};

pub struct MenuOption<Value> {
    label: &'static str,
    value: Value,
}

impl<Value> MenuOption<Value> {
    pub const fn new(value: Value, label: &'static str) -> Self {
        Self { value, label }
    }
}

pub struct Menu<Value: 'static + Copy> {
    is_locked: bool,
    counter: CircularCounter,
    options: &'static [MenuOption<Value>],
}

impl<Value: Copy> Menu<Value> {
    pub fn new(options: &'static [MenuOption<Value>]) -> Self {
        Self {
            is_locked: false,
            counter: CircularCounter::new(1, options.len()),
            options,
        }
    }

    fn value(&self) -> Value {
        let index = self.counter.value() - 1;
        self.options[index].value
    }

    pub fn next_view(&self, main_menu: Value, current_view: Value) -> Value {
        match (self.is_locked, current_view) {
            (false, _main_menu) if pnp::is_just_pressed(Button::Dright) => self.value(),
            (false, _) if pnp::is_just_pressed(Button::Dleft) => main_menu,
            (_, _) => current_view,
        }
    }

    fn cursor_str(&self, index: usize) -> &str {
        if self.counter.value() == index {
            ">"
        } else {
            " "
        }
    }

    pub fn draw(&self) {
        for (index, option) in self.options.iter().enumerate() {
            pnp::println!("{} {}", self.cursor_str(index + 1), option.label);
        }
        pnp::println!("");
        draw_version();
    }

    pub fn update_view(&mut self) {
        if self.is_locked {
            return;
        }

        if pnp::is_just_pressed(Button::Dup) {
            self.counter.decrement();
        } else if pnp::is_just_pressed(Button::Ddown) {
            self.counter.increment();
        }
    }

    pub fn update_lock(&mut self) -> bool {
        if pnp::is_just_pressed(Button::X | Button::Y) {
            self.is_locked = !self.is_locked;
        }
        self.is_locked
    }
}
