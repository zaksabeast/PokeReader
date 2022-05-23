use super::{daycare, help as help_view, rng as rng_view};
use crate::{
    pkrd::{display, reader, rng, views::pkm},
    utils::party_slot::PartySlot,
};
use ctr::res::CtrResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LeftGen7View {
    None,
    PartyView,
    WildView,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RightGen7View {
    None,
    RngView,
    DaycareView,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EntireGen7View {
    None,
    HelpView,
}

pub struct Gen7Views {
    left_view: LeftGen7View,
    right_view: RightGen7View,
    entire_view: EntireGen7View,
    party_slot: PartySlot,
}

impl Default for Gen7Views {
    fn default() -> Self {
        Self {
            left_view: LeftGen7View::None,
            right_view: RightGen7View::None,
            entire_view: EntireGen7View::None,
            party_slot: PartySlot::default(),
        }
    }
}

impl Gen7Views {
    fn update_views(&mut self) {
        self.right_view = match self.right_view {
            RightGen7View::RngView if rng_view::input::toggle() => RightGen7View::None,
            RightGen7View::DaycareView if daycare::input::toggle() => RightGen7View::None,
            _ if rng_view::input::toggle() => RightGen7View::RngView,
            _ if daycare::input::toggle() => RightGen7View::DaycareView,
            view => view,
        };

        self.left_view = match self.left_view {
            LeftGen7View::WildView if pkm::wild::input::toggle() => LeftGen7View::None,
            LeftGen7View::PartyView if pkm::party::input::toggle() => LeftGen7View::None,
            _ if pkm::wild::input::toggle() => LeftGen7View::WildView,
            _ if pkm::party::input::toggle() => LeftGen7View::PartyView,
            view => view,
        };

        if self.left_view == LeftGen7View::PartyView {
            self.party_slot = pkm::party::input::next_party_slot(self.party_slot);
        }

        self.entire_view = match self.entire_view {
            EntireGen7View::HelpView if help_view::input::toggle() => EntireGen7View::None,
            _ if help_view::input::toggle() => EntireGen7View::HelpView,
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
            LeftGen7View::PartyView => {
                let pkx = game.get_party_pkm(self.party_slot);
                pkm::party::draw(screen, &pkx, self.party_slot)?;
            }
            LeftGen7View::WildView => {
                let pkx = game.get_wild_pkm();
                pkm::wild::draw(screen, &pkx)?
            }
            LeftGen7View::None => {}
        }

        match self.right_view {
            RightGen7View::RngView => rng_view::draw(screen, game, rng)?,
            RightGen7View::DaycareView => daycare::draw(screen, game)?,
            RightGen7View::None => {}
        }

        match self.entire_view {
            EntireGen7View::HelpView => help_view::draw(screen)?,
            EntireGen7View::None => {}
        }

        Ok(())
    }
}
