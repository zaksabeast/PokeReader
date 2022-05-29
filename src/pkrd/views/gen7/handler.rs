use super::{daycare, help as help_view, rng as rng_view};
use crate::pkrd::reader::{RngSlot, WildSlot};
use crate::{
    pkrd::{display, reader, rng, views::pkm},
    utils::party_slot::PartySlot,
};
use ctr::res::CtrResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TopLeftGen7View {
    None,
    PartyView,
    WildView,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TopRightGen7View {
    None,
    RngView,
    DaycareView,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BottomGen7View {
    None,
    HelpView,
}

pub struct Gen7Views {
    left_view: TopLeftGen7View,
    right_view: TopRightGen7View,
    entire_view: BottomGen7View,
    party_slot: PartySlot,
    wild_slot: WildSlot,
    rng_slot: RngSlot,
}

impl Default for Gen7Views {
    fn default() -> Self {
        Self {
            left_view: TopLeftGen7View::None,
            right_view: TopRightGen7View::None,
            entire_view: BottomGen7View::None,
            party_slot: PartySlot::default(),
            wild_slot: WildSlot::default(),
            rng_slot: RngSlot::default(),
        }
    }
}

impl Gen7Views {
    fn update_views(&mut self) {
        self.right_view = match self.right_view {
            TopRightGen7View::RngView if rng_view::input::toggle() => TopRightGen7View::None,
            TopRightGen7View::DaycareView if daycare::input::toggle() => TopRightGen7View::None,
            _ if rng_view::input::toggle() => TopRightGen7View::RngView,
            _ if daycare::input::toggle() => TopRightGen7View::DaycareView,
            view => view,
        };

        self.left_view = match self.left_view {
            TopLeftGen7View::WildView if pkm::wild::input::toggle() => TopLeftGen7View::None,
            TopLeftGen7View::PartyView if pkm::party::input::toggle() => TopLeftGen7View::None,
            _ if pkm::wild::input::toggle() => TopLeftGen7View::WildView,
            _ if pkm::party::input::toggle() => TopLeftGen7View::PartyView,
            view => view,
        };

        if self.left_view == TopLeftGen7View::PartyView {
            self.party_slot = pkm::party::input::next_party_slot(self.party_slot);
        }

        if self.left_view == TopLeftGen7View::WildView {
            self.wild_slot = pkm::wild::input::next_wild_slot(self.wild_slot);
        }

        if self.right_view == TopRightGen7View::RngView {
            self.rng_slot = rng_view::input::next_rng_slot(self.rng_slot);
        }

        self.entire_view = match self.entire_view {
            BottomGen7View::HelpView if help_view::input::toggle() => BottomGen7View::None,
            _ if help_view::input::toggle() => BottomGen7View::HelpView,
            view => view,
        }
    }

    pub fn run_views<GameReader: reader::Gen7Reader>(
        &mut self,
        screen: &mut display::DirectWriteScreen,
        game: &GameReader,
        rng: &mut rng::Gen7Rng,
    ) -> CtrResult<()> {
        rng.update(game);
        self.update_views();

        match self.left_view {
            TopLeftGen7View::PartyView => {
                let pkx = game.get_party_pkm(self.party_slot);
                pkm::party::draw(screen, &pkx, self.party_slot)?;
            }
            TopLeftGen7View::WildView => {
                let wild = game.get_wild(self.wild_slot);
                pkm::wild::draw(screen, wild)?
            }
            TopLeftGen7View::None => {}
        }

        match self.right_view {
            TopRightGen7View::RngView => rng_view::draw(screen, game, rng, self.rng_slot)?,
            TopRightGen7View::DaycareView => daycare::draw(screen, game)?,
            TopRightGen7View::None => {}
        }

        match self.entire_view {
            BottomGen7View::HelpView => help_view::draw(screen)?,
            BottomGen7View::None => {}
        }

        Ok(())
    }
}
