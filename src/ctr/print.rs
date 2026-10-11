use core::fmt;

use super::framebuffer::{Color, Framebuffer, PixelFormat};

const DEFAULT_PRINT_X: usize = 8;
const DEFAULT_PRINT_Y: usize = 10;
// Includes a slot for the old C null terminator, so lines hold at most `print_max_len - 1` characters.
const DEFAULT_PRINT_MAX_LEN: usize = 30;

const MAX_LINES: usize = 18;
const MAX_LINE_LENGTH: usize = 46;

const LINE_HEIGHT: usize = 12;
const BACKGROUND_PADDING: usize = 4;

struct PrintState {
    print_x: usize,
    print_y: usize,
    print_max_len: usize,

    print_buffer: [[u8; MAX_LINE_LENGTH]; MAX_LINES],
    print_buffer_len: [u8; MAX_LINES],
    print_buffer_color: [Color; MAX_LINES],

    line_count: usize,
}

impl PrintState {
    const fn new() -> Self {
        Self {
            print_x: DEFAULT_PRINT_X,
            print_y: DEFAULT_PRINT_Y,
            print_max_len: DEFAULT_PRINT_MAX_LEN,

            print_buffer: [[0; MAX_LINE_LENGTH]; MAX_LINES],
            print_buffer_len: [0; MAX_LINES],
            print_buffer_color: [Color::rgb(255, 255, 255); MAX_LINES],

            line_count: 0,
        }
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.print_x = DEFAULT_PRINT_X;
        self.print_y = DEFAULT_PRINT_Y;
        self.print_max_len = DEFAULT_PRINT_MAX_LEN;
        self.line_count = 0;
    }
}

static mut PRINT_STATE: PrintState = PrintState::new();

struct LineWriter<'a> {
    buffer: &'a mut [u8],
    position: usize,
}

impl<'a> LineWriter<'a> {
    #[inline(always)]
    const fn new(buffer: &'a mut [u8]) -> Self {
        Self { buffer, position: 0 }
    }

    #[inline(always)]
    const fn len(&self) -> usize {
        self.position
    }
}

impl fmt::Write for LineWriter<'_> {
    #[inline]
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let remaining = self.buffer.len() - self.position;
        let bytes = text.as_bytes();

        let count = bytes.len().min(remaining);

        self.buffer[self.position..self.position + count].copy_from_slice(&bytes[..count]);

        self.position += count;

        Ok(())
    }
}

pub fn draw_to_screen(_screen_id: u32, framebuffer: *mut u8, stride: u32, format: u32) {
    let state = unsafe { &mut PRINT_STATE };

    if state.line_count == 0 {
        return;
    }

    let format: PixelFormat = (format & 0x0f).into();

    let mut fb = unsafe { Framebuffer::new(framebuffer, stride as usize, format) };

    let background_height = state.line_count * LINE_HEIGHT + BACKGROUND_PADDING;

    let background_width = state.print_max_len * 8 + BACKGROUND_PADDING * 2;

    fb.darken(
        state.print_y,
        state.print_x,
        background_height,
        background_width,
        1,
    );

    let text_x = state.print_x + BACKGROUND_PADDING;
    let mut text_y = state.print_y + BACKGROUND_PADDING;

    for i in 0..state.line_count {
        let len = state.print_buffer_len[i] as usize;

        if len != 0 {
            fb.string(
                text_y,
                text_x,
                state.print_buffer_color[i],
                &state.print_buffer[i][..len],
            );
        }

        text_y += LINE_HEIGHT;
    }

    state.reset();
}

#[inline]
pub fn set_print_max_len(max_len: usize) {
    let state = unsafe { &mut PRINT_STATE };

    state.print_max_len = (max_len + 1).min(MAX_LINE_LENGTH);
}

#[inline]
pub fn println_impl(args: fmt::Arguments<'_>, color: u32) {
    let state = unsafe { &mut PRINT_STATE };

    if state.line_count >= MAX_LINES {
        return;
    }

    let index = state.line_count;

    let max_len = state.print_max_len - 1;

    let mut writer = LineWriter::new(&mut state.print_buffer[index][..max_len]);

    let _ = fmt::write(&mut writer, args);

    state.print_buffer_len[index] = writer.len() as u8;
    state.print_buffer_color[index] = Color::from_u32(color);

    state.line_count += 1;
}

/// Prints to the console screen.
///
/// # Examples
/// ```ignore
/// ctr::println!("Test: {:x}", 0xaabbccddu32);
/// ```
macro_rules! println {
    () => {
        $crate::ctr::println_impl(core::format_args!(""), 0xffffff)
    };
    (color = $color:expr, $($arg:tt)*) => {{
        $crate::ctr::println_impl(core::format_args!($($arg)*), $color);
    }};
    ($($arg:tt)*) => {{
        $crate::ctr::println_impl(core::format_args!($($arg)*), 0xffffff);
    }};
}

pub(crate) use println;
