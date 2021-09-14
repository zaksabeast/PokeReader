use super::{party_view::PartyView, wild_view::WildView};
use crate::pkrd::{display, reader};
use ctr::{
    hid,
    hid::{Button, InterfaceDevice},
    res::CtrResult,
};

#[derive(Default)]
pub struct Views {
    show_rng_view: bool,
    party_view: PartyView,
    wild_view: WildView,
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

    if views.party_view.get_is_active() {
        let party_slot = views.party_view.get_slot();
        let pkx = game.get_party_pkm(party_slot);
        views.party_view.run_view(&pkx, screen)?;
    }

    Ok(())
}

pub fn run_gen7_views<GameReader: reader::Gen7Reader>(
    views: &mut Views,
    game: &GameReader,
    screen: &mut display::DirectWriteScreen,
) -> CtrResult<()> {
    if hid::Global::is_just_pressed(Button::Start | Button::Dup) {
        views.show_rng_view = !views.show_rng_view;
    }

    if views.show_rng_view {
        super::rng7::run_view(game, screen)?;
    }

    if views.party_view.get_is_active() {
        views.wild_view.set_is_active(false);

        let party_slot = views.party_view.get_slot();
        let pkx = game.get_party_pkm(party_slot);
        views.party_view.run_view(&pkx, screen)?;
    }

    if views.wild_view.get_is_active() {
        views.party_view.set_is_active(false);

        let pkx = game.get_wild_pkm();
        views.wild_view.run_view(&pkx, screen)?;
    }

    Ok(())
}
