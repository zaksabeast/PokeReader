use super::{context, font, Color};
use ctr::res::{CtrResult, GenericResultCode, ResultCode};

const SCREEN_WIDTH_TOP: u32 = 400;
const SCREEN_WIDTH_BOTTOM: u32 = 320;
const SCREEN_HEIGHT: u32 = 240;

pub struct DirectWriteScreen {
    context: context::ScreenContext,
}

impl Screen for DirectWriteScreen {
    fn new() -> Self {
        Self {
            context: context::ScreenContext::default(),
        }
    }

    fn get_context(&self) -> &context::ScreenContext {
        &self.context
    }

    fn set_context(
        &mut self,
        is_top_screen: bool,
        addr: u32,
        stride: u32,
        format: u32,
    ) -> CtrResult<()> {
        let writable_addr = match addr & 0xff000000 {
            0x1f000000 => Ok(addr),
            0x33000000 => Ok(addr + 0x70000000),
            _ => Err(ResultCode::from(GenericResultCode::InvalidPointer)),
        }?;
        self.context = context::ScreenContext::new(is_top_screen, writable_addr, stride, format)?;
        Ok(())
    }
}

// Thanks to NTR for the draw functions - https://github.com/44670/ntr_overlay_samples/blob/5fee35f160190fbcf0eddb54143c1bfd27b2586f/fps/source/ov.c
pub trait Screen {
    fn new() -> Self;

    fn get_context(&self) -> &context::ScreenContext;

    fn set_context(
        &mut self,
        is_top_screen: bool,
        addr: u32,
        stride: u32,
        format: u32,
    ) -> CtrResult<()>;

    fn get_is_top_screen(&self) -> bool {
        self.get_context().is_top_screen
    }

    fn get_is_bottom_screen(&self) -> bool {
        self.get_is_top_screen() == false
    }

    /// # Safety
    /// The caller needs to make sure:
    /// - The x is never above 320 for a bottom screen
    /// - The x is never above 340 for a top screen
    /// - The y is never above 240
    unsafe fn draw_pixel(&mut self, color: &Color, x: u32, y: u32) -> CtrResult<()> {
        let context = self.get_context();
        let format = context.format;
        let addr = context.addr;
        let stride = context.stride;

        if (format & 0xf) == 2 {
            let pixel = ((color.r as u32) << 11) | ((color.g as u32) << 5) | (color.b as u32);
            let vram = (addr + (stride * x) + 480 - (2 * y)) as *mut u32;
            vram.write(pixel);
        } else {
            let bytes: [u8; 3] = [color.b, color.g, color.r];
            let vram = (addr + (stride * x) + 720 - (3 * y)) as *mut u8;
            vram.write(bytes[0]);
            vram.add(1).write(bytes[1]);
            vram.add(2).write(bytes[2]);
        }

        Ok(())
    }

    #[inline(always)]
    fn is_safe_pixel(&self, x: u32, y: u32) -> bool {
        if y > SCREEN_HEIGHT {
            return false;
        }

        let is_top_screen = self.get_is_top_screen();

        if is_top_screen && x > SCREEN_WIDTH_TOP {
            return false;
        }

        if !is_top_screen && x > SCREEN_WIDTH_BOTTOM {
            return false;
        }

        true
    }

    #[inline(always)]
    fn is_safe_pixel_range(&self, x1: u32, y1: u32, x2: u32, y2: u32) -> bool {
        self.is_safe_pixel(x1, y1) && self.is_safe_pixel(x2, y2)
    }

    unsafe fn draw_character(
        &mut self,
        color: &Color,
        letter: char,
        x: u32,
        y: u32,
    ) -> CtrResult<()> {
        let font_char = font::convert_letter_to_font(letter);
        let mask = 0b10000000;

        for (y_offset, draw_line) in font_char.iter().enumerate() {
            for x_offset in 0..font::CHAR_WIDTH {
                if ((mask >> x_offset) & *draw_line) != 0 {
                    self.draw_pixel(color, x_offset + x, (y_offset as u32) + y)?;
                }
            }
        }

        Ok(())
    }

    fn draw_string(&mut self, color: &Color, text: &str, x: u32, y: u32) -> CtrResult<()> {
        let text_len = text.len() as u32;
        if !self.is_safe_pixel_range(
            x,
            y,
            x + (text_len * font::CHAR_WIDTH),
            y + font::CHAR_HEGHT,
        ) {
            return Err(GenericResultCode::InvalidValue.into());
        }

        for (index, letter) in text.chars().enumerate() {
            // This is safe because the pixels are validated ahead of time
            unsafe { self.draw_character(color, letter, x + (index * 8) as u32, y) }?;
        }

        Ok(())
    }

    fn paint_square(
        &mut self,
        color: &Color,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> CtrResult<()> {
        let x_max = x + width;
        let y_max = y + height;

        if !self.is_safe_pixel_range(x, y, x_max, y_max) {
            return Err(GenericResultCode::InvalidValue.into());
        }

        for current_x in x..x_max {
            for current_y in y..y_max {
                // This is safe because the pixels are validated ahead of time
                unsafe { self.draw_pixel(color, current_x, current_y) }?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod is_safe_pixel {
        use super::*;

        #[test]
        fn should_return_false_if_the_y_is_too_large() {
            let mut screen = DirectWriteScreen::new();
            screen.set_context(true, 0x1F000000, 0, 0).unwrap();

            let result = screen.is_safe_pixel(0, SCREEN_HEIGHT + 1);
            assert_eq!(result, false);
        }

        #[test]
        fn should_return_false_if_the_x_is_too_large_for_the_top_screen() {
            let mut screen = DirectWriteScreen::new();
            screen.set_context(true, 0x1F000000, 0, 0).unwrap();

            let result = screen.is_safe_pixel(SCREEN_WIDTH_TOP + 1, 0);
            assert_eq!(result, false);
        }

        #[test]
        fn should_return_false_if_the_x_is_too_large_for_the_bottom_screen() {
            let mut screen = DirectWriteScreen::new();
            screen.set_context(false, 0x1F000000, 0, 0).unwrap();

            let result = screen.is_safe_pixel(SCREEN_WIDTH_BOTTOM + 1, 0);
            assert_eq!(result, false);
        }

        #[test]
        fn should_return_true_if_the_coordinates_are_under_the_max_sizes_for_the_top_screen() {
            let mut screen = DirectWriteScreen::new();
            screen.set_context(true, 0x1F000000, 0, 0).unwrap();

            let result = screen.is_safe_pixel(SCREEN_WIDTH_TOP, SCREEN_HEIGHT);
            assert_eq!(result, true);
        }

        #[test]
        fn should_return_true_if_the_coordinates_are_under_the_max_sizes_for_the_bottom_screen() {
            let mut screen = DirectWriteScreen::new();
            screen.set_context(false, 0x1F000000, 0, 0).unwrap();

            let result = screen.is_safe_pixel(SCREEN_WIDTH_BOTTOM, SCREEN_HEIGHT);
            assert_eq!(result, true);
        }
    }
}
