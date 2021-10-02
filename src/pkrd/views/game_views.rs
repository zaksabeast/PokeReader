use super::{party_view::PartyView, rng6::Rng6View, rng7::Rng7View, wild_view::WildView};
use crate::pkrd::{display, reader, rng};
use ctr::res::CtrResult;

#[derive(Default)]
pub struct Gen6Views {
    rng_view: Rng6View,
    party_view: PartyView,
    wild_view: WildView,
}

impl Gen6Views {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Gen6Views {
    pub fn run_views<GameReader: reader::Gen6Reader>(
        views: &mut Gen6Views,
        game: &GameReader,
        rng: &mut rng::Gen6Rng,
        screen: &mut display::DirectWriteScreen,
    ) -> CtrResult<()> {
        rng.update(game);

        if views.rng_view.get_is_active() {
            Rng6View::run_view(game, rng, screen)?;
        }

        if views.party_view.get_is_active() {
            let party_slot = views.party_view.get_slot();
            let pkx = game.get_party_pkm(party_slot);
            views.party_view.run_view(&pkx, screen)?;
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct Gen7Views {
    rng_view: Rng7View,
    party_view: PartyView,
    wild_view: WildView,
}

impl Gen7Views {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Gen7Views {
    pub fn run_views<GameReader: reader::Gen7Reader>(
        views: &mut Gen7Views,
        game: &GameReader,
        rng: &mut rng::Gen7Rng,
        screen: &mut display::DirectWriteScreen,
    ) -> CtrResult<()> {
        rng.update(game);

        if views.rng_view.get_is_active() {
            Rng7View::run_view(game, rng, screen)?;
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
            WildView::run_view(&pkx, screen)?;
        }

        Ok(())
    }
}
