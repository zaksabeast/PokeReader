use crate::{pnp, pnp::Button, utils::CircularCounter};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_HASH: &str = env!("GIT_HASH");

pub trait MenuOptionValue: Copy {
    fn get_label(option: Self) -> &'static str;
}

pub struct MenuOption<Value: MenuOptionValue> {
    label: &'static str,
    value: Value,
}

impl<Value: MenuOptionValue> MenuOption<Value> {
    pub fn new(value: Value) -> Self {
        Self {
            value,
            label: Value::get_label(value),
        }
    }
}

pub struct Menu<const MAX: usize, Value: MenuOptionValue> {
    is_locked: bool,
    counter: CircularCounter<1, MAX>,
    options: [MenuOption<Value>; MAX],
}

impl<const MAX: usize, Value: MenuOptionValue> Menu<MAX, Value> {
    pub fn new(options: [MenuOption<Value>; MAX]) -> Self {
        Self {
            is_locked: false,
            counter: CircularCounter::default(),
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
        pnp::println!("Ver {} {}", VERSION, GIT_HASH);
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

    pub fn update_lock(&mut self) {
        if pnp::is_just_pressed(Button::X | Button::Y) {
            self.is_locked = !self.is_locked;
        }
    }

    pub fn is_locked(&self) -> bool {
        self.is_locked
    }
}
