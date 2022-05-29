use crate::pkrd::{display, display::Screen};
use ctr::res::CtrResult;

fn draw(
    screen: &mut display::DirectWriteScreen,
    title: &str,
    content: &[&str],
    width: u32,
    mut x: u32,
    mut y: u32,
) -> CtrResult<()> {
    let black = display::Color::black();
    let white = display::Color::white();

    let height: u32 = ((content.len() * 12) + 20) as u32;

    screen.paint_square(&black, x, y, width, height)?;

    x += 4;
    y += 4;
    screen.draw_string(&white, title, x, y)?;
    y += 16;

    for line in content.iter() {
        screen.draw_string(&white, line, x, y)?;
        y += 12;
    }

    Ok(())
}

pub fn draw_top_left(
    screen: &mut display::DirectWriteScreen,
    title: &str,
    content: &[&str],
) -> CtrResult<()> {
    if screen.get_is_top_screen() {
        draw(screen, title, content, 184, 6, 10)?;
    }

    Ok(())
}

pub fn draw_top_right(
    screen: &mut display::DirectWriteScreen,
    title: &str,
    content: &[&str],
) -> CtrResult<()> {
    if screen.get_is_top_screen() {
        draw(screen, title, content, 192, 200, 10)?;
    }

    Ok(())
}

pub fn draw_bottom(
    screen: &mut display::DirectWriteScreen,
    title: &str,
    content: &[&str],
) -> CtrResult<()> {
    draw(screen, title, content, 308, 6, 10)
}
