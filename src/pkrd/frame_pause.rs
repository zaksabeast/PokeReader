use super::context::PkrdServiceContext;
use ctr::{hid, hid::InterfaceDevice, svc};

pub fn handle_frame_pause(context: &mut PkrdServiceContext, is_top_screen: bool) {
    if hid::Global::is_just_pressed(hid::Button::Start | hid::Button::Select) {
        context.is_paused = true;
    }

    while context.is_paused && is_top_screen {
        hid::Global::scan_input();

        let just_down = hid::Global::just_down_buttons();

        if just_down.select() {
            break;
        }

        if just_down.a() || just_down.start() {
            context.is_paused = false;
            break;
        }

        svc::sleep_thread(50000000);
    }
}
