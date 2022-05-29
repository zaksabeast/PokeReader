use super::help as help_view;
use super::rng as rng_view;
use crate::pkrd::views::gen6::daycare;
use crate::{
    pkrd::{display, reader, reader::DaycareSlot, rng, views::pkm},
    utils::party_slot::PartySlot,
};
use ctr::res::CtrResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TopLeftGen6View {
    None,
    PartyView,
    WildView,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TopRightGen6View {
    None,
    RngView,
    DaycareView,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BottomGen6View {
    None,
    HelpView,
}

pub struct Gen6Views {
    left_view: TopLeftGen6View,
    right_view: TopRightGen6View,
    entire_view: BottomGen6View,
    party_slot: PartySlot,
    daycare_slot: DaycareSlot,
}

impl Default for Gen6Views {
    fn default() -> Self {
        Self {
            left_view: TopLeftGen6View::None,
            right_view: TopRightGen6View::None,
            entire_view: BottomGen6View::None,
            party_slot: PartySlot::default(),
            daycare_slot: DaycareSlot::default(),
        }
    }
}

impl Gen6Views {
    fn update_views(&mut self) {
        self.right_view = match self.right_view {
            TopRightGen6View::RngView if rng_view::input::toggle() => TopRightGen6View::None,
            TopRightGen6View::DaycareView if daycare::input::toggle() => TopRightGen6View::None,
            _ if rng_view::input::toggle() => TopRightGen6View::RngView,
            _ if daycare::input::toggle() => TopRightGen6View::DaycareView,
            view => view,
        };

        if self.right_view == TopRightGen6View::DaycareView {
            self.daycare_slot = daycare::input::next_daycare_slot(self.daycare_slot);
        }

        self.left_view = match self.left_view {
            TopLeftGen6View::PartyView if pkm::party::input::toggle() => TopLeftGen6View::None,
            TopLeftGen6View::WildView if pkm::wild::input::toggle() => TopLeftGen6View::None,
            _ if pkm::party::input::toggle() => TopLeftGen6View::PartyView,
            _ if pkm::wild::input::toggle() => TopLeftGen6View::WildView,
            view => view,
        };

        if self.left_view == TopLeftGen6View::PartyView {
            self.party_slot = pkm::party::input::next_party_slot(self.party_slot);
        }

        self.entire_view = match self.entire_view {
            BottomGen6View::HelpView if help_view::input::toggle() => BottomGen6View::None,
            _ if help_view::input::toggle() => BottomGen6View::HelpView,
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
            TopLeftGen6View::PartyView => {
                let pkx = game.get_party_pkm(self.party_slot);
                pkm::party::draw(screen, &pkx, self.party_slot)?;
            }
            TopLeftGen6View::WildView => {
                let pkx = game.get_wild_pkm();
                pkm::wild::draw(screen, pkx)?;
            }
            TopLeftGen6View::None => {}
        }

        match self.right_view {
            TopRightGen6View::RngView => rng_view::draw(screen, game, rng)?,
            TopRightGen6View::DaycareView => {
                let daycare = game.get_daycare(self.daycare_slot);
                daycare::draw(screen, &daycare)?;
            }
            TopRightGen6View::None => {}
        }

        match self.entire_view {
            BottomGen6View::HelpView => help_view::draw(screen)?,
            BottomGen6View::None => {}
        }

        Ok(())
    }
}
