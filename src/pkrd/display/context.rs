use ctr::res::{CtrResult, GenericResultCode};

const SCREEN_ADDRESS_MIN: u32 = 0x1F000000;
const SCREEN_ADDRESS_MAX: u32 = 0x1F5FFFFF;

#[derive(Debug)]
pub struct ScreenContext {
    pub(super) is_top_screen: bool,
    pub(super) addr: u32,
    pub(super) stride: u32,
    pub(super) format: u32,
}

impl ScreenContext {
    pub fn new(is_top_screen: bool, addr: u32, stride: u32, format: u32) -> CtrResult<Self> {
        if !(SCREEN_ADDRESS_MIN..=SCREEN_ADDRESS_MAX).contains(&addr) {
            return Err(GenericResultCode::InvalidPointer.into());
        }

        Ok(Self {
            is_top_screen,
            addr,
            stride,
            format,
        })
    }
}

impl Default for ScreenContext {
    fn default() -> Self {
        Self::new(false, 0x1F000000, 0, 0).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod new {
        use super::*;

        #[test]
        fn should_error_if_the_address_is_below_screen_address_min() {
            ScreenContext::new(true, SCREEN_ADDRESS_MIN - 1, 0, 0)
                .expect_err("ScreenContext address should have been too low");
        }

        #[test]
        fn should_error_if_the_address_is_above_screen_address_max() {
            ScreenContext::new(true, SCREEN_ADDRESS_MAX + 1, 0, 0)
                .expect_err("ScreenContext address should have been too high");
        }

        #[test]
        fn should_succeed_if_the_address_is_screen_address_min() {
            ScreenContext::new(true, SCREEN_ADDRESS_MIN, 0, 0)
                .expect("ScreenContext address should have been in a good range");
        }

        #[test]
        fn should_succeed_if_the_address_is_screen_address_max() {
            ScreenContext::new(true, SCREEN_ADDRESS_MAX, 0, 0)
                .expect("ScreenContext address should have been in a good range");
        }
    }
}
