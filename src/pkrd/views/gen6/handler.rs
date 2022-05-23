use super::help as help_view;
use super::rng as rng_view;
use crate::pkrd::views::gen6::daycare;
use crate::{
    pkrd::{display, reader, reader::DaycareSlot, rng, views::pkm},
    utils::party_slot::PartySlot,
};
use ctr::res::CtrResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LeftGen6View {
    None,
    PartyView,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RightGen6View {
    None,
    RngView,
    DaycareView,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EntireGen6View {
    None,
    HelpView,
}

pub struct Gen6Views {
    left_view: LeftGen6View,
    right_view: RightGen6View,
    entire_view: EntireGen6View,
    party_slot: PartySlot,
    daycare_slot: DaycareSlot,
}

impl Default for Gen6Views {
    fn default() -> Self {
        Self {
            left_view: LeftGen6View::None,
            right_view: RightGen6View::None,
            entire_view: EntireGen6View::None,
            party_slot: PartySlot::default(),
            daycare_slot: DaycareSlot::default(),
        }
    }
}

impl Gen6Views {
    fn update_views(&mut self) {
        self.right_view = match self.right_view {
            RightGen6View::RngView if rng_view::input::toggle() => RightGen6View::None,
            RightGen6View::DaycareView if daycare::input::toggle() => RightGen6View::None,
            _ if rng_view::input::toggle() => RightGen6View::RngView,
            _ if daycare::input::toggle() => RightGen6View::DaycareView,
            view => view,
        };

        if self.right_view == RightGen6View::DaycareView {
            self.daycare_slot = daycare::input::next_daycare_slot(self.daycare_slot);
        }

        self.left_view = match self.left_view {
            LeftGen6View::PartyView if pkm::party::input::toggle() => LeftGen6View::None,
            _ if pkm::party::input::toggle() => LeftGen6View::PartyView,
            view => view,
        };

        if self.left_view == LeftGen6View::PartyView {
            self.party_slot = pkm::party::input::next_party_slot(self.party_slot);
        }

        self.entire_view = match self.entire_view {
            EntireGen6View::HelpView if help_view::input::toggle() => EntireGen6View::None,
            _ if help_view::input::toggle() => EntireGen6View::HelpView,
            view => view,
        };
    }

    pub fn run_views<GameReader: reader::Gen6Reader>(
        &mut self,
        screen: &mut display::DirectWriteScreen,
        game: &GameReader,
        rng: &mut rng::Gen6Rng,
    ) -> CtrResult<()> {
        rng.update(game);
        self.update_views();

        match self.left_view {
            LeftGen6View::PartyView => {
                let pkx = game.get_party_pkm(self.party_slot - 1);
                pkm::party::draw(screen, &pkx, self.party_slot)?;
            }
            LeftGen6View::None => {}
        }

        match self.right_view {
            RightGen6View::RngView => rng_view::draw(screen, game, rng)?,
            RightGen6View::DaycareView => {
                let daycare = game.get_daycare(self.daycare_slot);
                daycare::draw(screen, &daycare)?;
            }
            RightGen6View::None => {}
        }

        match self.entire_view {
            EntireGen6View::HelpView => help_view::draw(screen)?,
            EntireGen6View::None => {}
        }

        Ok(())
    }
}
