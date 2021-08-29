use super::{font::convert_letter_to_font, Color};
use ctr::{res::CtrResult, DebugProcess};

pub struct GspContext {
    is_top_screen: bool,
    addr: u32,
    stride: u32,
    format: u32,
}

impl GspContext {
    pub fn new(is_top_screen: bool, addr: u32, stride: u32, format: u32) -> Self {
        Self {
            is_top_screen,
            addr,
            stride,
            format,
        }
    }
}

pub struct Gsp {
    debug: DebugProcess,
    context: GspContext,
}

// Thanks to NTR for the draw functions - https://github.com/44670/ntr_overlay_samples/blob/5fee35f160190fbcf0eddb54143c1bfd27b2586f/fps/source/ov.c
impl Gsp {
    pub fn new() -> CtrResult<Self> {
        let debug = DebugProcess::new(0x0004013000001C02)?;
        debug.eat_events()?;
        Ok(Self {
            debug,
            context: GspContext::new(false, 0, 0, 0),
        })
    }

    pub fn get_is_top_screen(&self) -> bool {
        self.context.is_top_screen
    }

    pub fn set_context(&mut self, is_top_screen: bool, addr: u32, stride: u32, format: u32) {
        self.context = GspContext::new(is_top_screen, addr, stride, format);
    }

    pub fn draw_pixel(&self, color: &Color, x: u32, y: u32) -> CtrResult<()> {
        let format = self.context.format;
        let addr = self.context.addr;
        let stride = self.context.stride;

        if (format & 0xf) == 2 {
            let pixel = ((color.r as u32) << 11) | ((color.g as u32) << 5) | (color.b as u32);
            self.debug
                .write_bytes(addr + (stride * x) + 480 - (2 * y), &pixel.to_ne_bytes())?;
        } else {
            let bytes: [u8; 3] = [color.b, color.g, color.r];
            self.debug
                .write_bytes(addr + (stride * x) + 720 - (3 * y), &bytes)?;
        }

        Ok(())
    }

    pub fn draw_character(&self, color: &Color, letter: char, x: u32, y: u32) -> CtrResult<()> {
        let font_char = convert_letter_to_font(letter);
        let mask = 0b10000000;

        for (y_offset, draw_line) in font_char.iter().enumerate() {
            for x_offset in 0..8 {
                if ((mask >> x_offset) & *draw_line) != 0 {
                    self.draw_pixel(color, x_offset + x, (y_offset as u32) + y)?;
                }
            }
        }

        Ok(())
    }

    pub fn draw_string(&self, color: &Color, text: &str, x: u32, y: u32) -> CtrResult<()> {
        for (index, letter) in text.chars().enumerate() {
            self.draw_character(color, letter, x + (index * 8) as u32, y)?;
        }

        Ok(())
    }

    pub fn paint_square(
        &self,
        color: &Color,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> CtrResult<()> {
        for current_x in x..(x + width) {
            for current_y in y..(y + height) {
                self.draw_pixel(color, current_x, current_y)?;
            }
        }

        Ok(())
    }
}
