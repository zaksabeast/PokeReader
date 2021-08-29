use crate::pkrd::gsp;
use ctr::res::CtrResult;

// We want screen moved to this function so it drops
// and releases the Gsp hold once it's finished.
// This prevents the game from running before Gsp is ready,
// and viewing the home menu before Gsp is ready.
pub fn run_hook(screen: gsp::Gsp) -> CtrResult<()> {
    if screen.get_is_top_screen() {
        let black = gsp::Color::black();
        let white = gsp::Color::white();

        screen.paint_square(&black, 30, 30, 200, 20)?;
        screen.draw_string(&white, "Hello from rust!", 40, 40)?;
    }

    Ok(())
}
