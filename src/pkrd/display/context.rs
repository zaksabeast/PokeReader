use ctr::res::{CtrResult, GenericResultCode};

const VRAM_ADDRESS_MIN: u32 = 0x1F000000;
const VRAM_ADDRESS_MAX: u32 = 0x1F5FFFFF;

const FCRAM_ADDRESS_MIN: u32 = 0xA0000000; // Uncached FCRAM
const FCRAM_ADDRESS_MAX: u32 = 0xAFFFFFFF; // Uncached FCRAM

#[derive(Debug)]
pub struct ScreenContext {
    pub(super) is_top_screen: bool,
    pub(super) addr: u32,
    pub(super) stride: u32,
    pub(super) format: u32,
}

impl ScreenContext {
    pub fn new(is_top_screen: bool, addr: u32, stride: u32, format: u32) -> CtrResult<Self> {
        if !(VRAM_ADDRESS_MIN..=VRAM_ADDRESS_MAX).contains(&addr)
            && !(FCRAM_ADDRESS_MIN..=FCRAM_ADDRESS_MAX).contains(&addr)
        {
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
        fn should_error_if_the_address_is_above_fcram_address_max() {
            ScreenContext::new(true, FCRAM_ADDRESS_MAX + 1, 0, 0)
                .expect_err("ScreenContext address should have been too high");
        }

        #[test]
        fn should_error_if_the_address_is_below_vram_address_min() {
            ScreenContext::new(true, VRAM_ADDRESS_MIN - 1, 0, 0)
                .expect_err("ScreenContext address should have been too low");
        }

        #[test]
        fn should_error_if_the_address_is_above_vram_address_max_and_below_fcram_address_min() {
            ScreenContext::new(true, VRAM_ADDRESS_MAX + 1, 0, 0)
                .expect_err("ScreenContext address is in an invalid range");
        }

        #[test]
        fn should_succeed_if_the_address_is_vram_address_min() {
            ScreenContext::new(true, VRAM_ADDRESS_MIN, 0, 0)
                .expect("ScreenContext address should have been in a good range");
        }

        #[test]
        fn should_succeed_if_the_address_is_vram_address_max() {
            ScreenContext::new(true, VRAM_ADDRESS_MAX, 0, 0)
                .expect("ScreenContext address should have been in a good range");
        }

        #[test]
        fn should_succeed_if_the_address_is_fcram_address_min() {
            ScreenContext::new(true, FCRAM_ADDRESS_MIN, 0, 0)
                .expect("ScreenContext address should have been in a good range");
        }

        #[test]
        fn should_succeed_if_the_address_is_fcram_address_max() {
            ScreenContext::new(true, FCRAM_ADDRESS_MAX, 0, 0)
                .expect("ScreenContext address should have been in a good range");
        }
    }
}
