#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircularCounter<const MIN: u32, const MAX: u32> {
    value: u32,
}

impl<const MIN: u32, const MAX: u32> CircularCounter<MIN, MAX> {
    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn increment(&mut self) -> u32 {
        if self.value == MAX {
            self.value = MIN;
        } else {
            self.value += 1;
        }

        self.value
    }

    pub fn decrement(&mut self) -> u32 {
        if self.value == MIN {
            self.value = MAX;
        } else {
            self.value -= 1;
        }

        self.value
    }
}

impl<const MIN: u32, const MAX: u32> Default for CircularCounter<MIN, MAX> {
    fn default() -> Self {
        Self { value: MIN }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    type TestCounter = CircularCounter<1, 3>;

    mod value {
        use super::*;

        #[test]
        fn should_return_value() {
            let counter = TestCounter { value: 1 };
            assert_eq!(counter.value(), 1);
        }
    }

    mod increment {
        use super::*;

        #[test]
        fn should_increment() {
            let mut counter = TestCounter { value: 1 };
            let result = counter.increment();
            assert_eq!(result, 2);
            assert_eq!(counter, TestCounter { value: 2 });
        }

        #[test]
        fn should_increment_to_min_on_overflow() {
            let mut counter = TestCounter { value: 3 };
            let result = counter.increment();
            assert_eq!(result, 1);
            assert_eq!(counter, TestCounter { value: 1 });
        }
    }

    mod decrement {
        use super::*;

        #[test]
        fn should_decrement() {
            let mut counter = TestCounter { value: 2 };
            let result = counter.decrement();
            assert_eq!(result, 1);
            assert_eq!(counter, TestCounter { value: 1 });
        }

        #[test]
        fn should_decrement_to_min_on_overflow() {
            let mut counter = TestCounter { value: 1 };
            let result = counter.decrement();
            assert_eq!(result, 3);
            assert_eq!(counter, TestCounter { value: 3 });
        }
    }
}
