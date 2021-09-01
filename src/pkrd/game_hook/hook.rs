use crate::pkrd::display;
use ctr::res::CtrResult;

pub fn run_hook(screen: &mut display::Screen) -> CtrResult<()> {
    if screen.get_is_top_screen() {
        let black = display::Color::black();
        let white = display::Color::white();

        screen.paint_square(&black, 30, 30, 200, 20)?;
        screen.draw_string(&white, "Hello from rust!", 40, 40)?;
    }

    Ok(())
}
