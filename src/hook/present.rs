use crate::app::loaded_title;
use crate::ctr::framebuffer;
use crate::ctr::hid::{Button, just_pressed, scan_input};
use crate::ctr::{draw_to_screen, flush_current_process_data_cache, sleep_thread};
use crate::pnp;

const SCREEN_WIDTH: u32 = framebuffer::WIDTH as u32;
const SCREEN_HEIGHT: u32 = framebuffer::HEIGHT as u32;

static mut IS_PAUSED: bool = false;

fn handle_freeze(is_top_screen: bool) {
    let is_gen_2 = matches!(loaded_title(), Ok(title) if title.is_crystal());
    if pnp::is_just_pressed(Button::Start | Button::Select)
        || (is_gen_2 && pnp::is_just_pressed(Button::L | Button::R))
    {
        unsafe { IS_PAUSED = true };
    }

    while unsafe { IS_PAUSED } && !is_top_screen {
        scan_input();

        let just_pressed = just_pressed();

        if just_pressed & (Button::Select | Button::L) != 0 {
            break;
        }

        if just_pressed & (Button::A | Button::Start | Button::R) != 0 {
            unsafe { IS_PAUSED = false };
            break;
        }

        sleep_thread(50000000);
    }
}

/// Called by the patched present framebuffer function (see `DRAW_PATCH`).
///
/// The patch pushes the game's r0-r3 before calling this, so `screen_id` through `fb_b`
/// are the game's original r0-r3, and `stride`/`format` are its r4/r5.
pub extern "C" fn run_hook(
    _r0: u32,
    _r1: u32,
    _r2: u32,
    _r3: u32,
    screen_id: u32,
    _swap: u32,
    fb_a: *mut u8,
    fb_b: *mut u8,
    stride: u32,
    format: u32,
) {
    let is_top_screen = screen_id == 0;
    if is_top_screen {
        scan_input();
        crate::app::run_frame();
        draw_to_screen(screen_id, fb_a, stride, format);
    }

    let _ = flush_current_process_data_cache(fb_a as u32, SCREEN_WIDTH * SCREEN_HEIGHT);
    // Thanks to https://github.com/44670/NTR/blob/c764c0f68c08f3518a9f284f5fda1bf3b2636123/source/plg.c#L868-L870
    if is_top_screen && fb_a != fb_b && !fb_b.is_null() {
        let _ = flush_current_process_data_cache(fb_b as u32, SCREEN_WIDTH * SCREEN_HEIGHT);
    }

    handle_freeze(is_top_screen);
}
