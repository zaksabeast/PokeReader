use crate::pkrd::{display, reader, reader::pkm};
use alloc::string::ToString;
use ctr::{
    hid,
    hid::{Button, InterfaceDevice},
    res::CtrResult,
};

#[derive(Default)]
pub struct Views {
    show_rng_view: bool,
    show_party_view: bool,
    party_slot: pkm::PartySlot,
}

impl Views {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn run_gen6_views<GameReader: reader::Gen6Reader>(
    views: &mut Views,
    game: &GameReader,
    screen: &mut display::DirectWriteScreen,
) -> CtrResult<()> {
    if hid::Global::is_just_pressed(Button::Start | Button::Dup) {
        views.show_rng_view = !views.show_rng_view;
    }

    if views.show_rng_view {
        super::rng6::run_view(game, screen)?;
    }

    Ok(())
}

pub fn run_gen7_views<GameReader: reader::Gen7Reader>(
    views: &mut Views,
    game: &GameReader,
    screen: &mut display::DirectWriteScreen,
) -> CtrResult<()> {
    // TODO: Clean this and do it better

    if hid::Global::is_just_pressed(Button::Start | Button::Dup) {
        views.show_rng_view = !views.show_rng_view;
    }

    if hid::Global::is_just_pressed(Button::Start | Button::Dright) {
        views.show_party_view = !views.show_party_view;
    }

    if hid::Global::is_just_pressed(Button::Select | Button::Dright) {
        views.party_slot = views.party_slot.increment()
    }

    if hid::Global::is_just_pressed(Button::Select | Button::Dleft) {
        views.party_slot = views.party_slot.decrement()
    }

    if views.show_rng_view {
        super::rng7::run_view(game, screen)?;
    }

    if views.show_party_view {
        let pkx = game.get_party_pkm(views.party_slot);
        let title = &alloc::format!("Party {}", views.party_slot.to_string());
        super::pkx::run_view(title, &pkx, screen)?;
    }

    Ok(())
}
