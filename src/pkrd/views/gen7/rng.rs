use core::convert::TryInto;

use crate::pkrd::reader::RngSlot;
use crate::pkrd::{display, reader, rng, views::view};
use ctr::{res::CtrResult};

pub mod input {
    use super::*;
    use ctr::hid::{Button, Global, InterfaceDevice};

    pub fn toggle() -> bool {
        Global::is_just_pressed(Button::Start | Button::Dup)
    }

    fn increment() -> bool {
        Global::is_just_pressed(Button::Select | Button::Dright)
    }

    fn decrement() -> bool {
        Global::is_just_pressed(Button::Select | Button::Dleft)
    }

    pub fn next_rng_slot(mut slot: RngSlot) -> RngSlot {
        if increment() {
            slot.increment();
        }

        if decrement() {
            slot.decrement();
        }

        slot
    }
}

pub fn draw(
    screen: &mut display::DirectWriteScreen,
    game: &impl reader::Gen7Reader,
    rng: &rng::Gen7Rng,
    rng_slot: RngSlot,
) -> CtrResult<()> {
    if rng_slot.value() == 0 {
        draw_main(screen, game, rng)?;
    } else {
        draw_sos(screen, game)?;
    }

    Ok(())
}

pub fn draw_main(
    screen: &mut display::DirectWriteScreen,
    game: &impl reader::Gen7Reader,
    rng: &rng::Gen7Rng,
) -> CtrResult<()> {
    let init_seed = game.get_initial_seed();
    let sfmt_state = game.get_sfmt_state();
    let sfmt_state_bytes = sfmt_state.to_ne_bytes();
    let sfmt_state_parts: [u32; 2] = [u32::from_ne_bytes(sfmt_state_bytes[0..4].try_into().unwrap()), u32::from_ne_bytes(sfmt_state_bytes[4..8].try_into().unwrap())];
    let sfmt_advances = rng.get_sfmt_advances();
    let tid = game.get_tid();
    let tsv = game.get_tsv();

    view::draw_top_right(
        screen,
        "Main RNG View",
        &[
            &alloc::format!("Init seed: {:08X}", init_seed),
            &alloc::format!("Curr state[1]: {:08X}", sfmt_state_parts[1]),
            &alloc::format!("Curr state[0]: {:08X}", sfmt_state_parts[0]),
            &alloc::format!("Advances: {}", sfmt_advances),
            &alloc::format!(""),
            &alloc::format!("Gen7TID: {}", tid),
            &alloc::format!("TSV: {}", tsv),
        ],
    )
}

pub fn draw_sos(
    screen: &mut display::DirectWriteScreen,
    game: &impl reader::Gen7Reader,
) -> CtrResult<()> {
    let sos_seed = game.get_sos_seed();
    let sos_chain = game.get_sos_chain();

    view::draw_top_right(
        screen,
        "SOS RNG View",
        &[
            &alloc::format!("SOS Seed: {:08X}", sos_seed),
            &alloc::format!("SOS Chain Length: {}", sos_chain),
        ],
    )
}
