use super::{
    draw::{draw_daycare, draw_header, draw_pkx, draw_rng, draw_sos},
    reader::Gen7Reader,
};
use crate::{
    menu::{Menu, MenuOption, MenuOptionValue},
    rng::{RngWrapper, Sfmt},
    utils::ShowView,
};
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gen7View {
    MainMenu,
    Rng,
    Daycare,
    WildPokemon,
    Sos,
    PartySlot1,
    PartySlot2,
    PartySlot3,
    PartySlot4,
    PartySlot5,
    PartySlot6,
    Box,
    PelagoSlot1,
    PelagoSlot2,
    PelagoSlot3,
}

impl MenuOptionValue for Gen7View {
    fn get_label(view: Self) -> &'static str {
        match view {
            Self::MainMenu => "Main Menu",
            Self::Rng => "RNG",
            Self::Daycare => "Daycare",
            Self::WildPokemon => "Wild",
            Self::Sos => "SOS",
            Self::PartySlot1 => "Party 1",
            Self::PartySlot2 => "Party 2",
            Self::PartySlot3 => "Party 3",
            Self::PartySlot4 => "Party 4",
            Self::PartySlot5 => "Party 5",
            Self::PartySlot6 => "Party 6",
            Self::Box => "Box",
            Self::PelagoSlot1 => "Pelago 1",
            Self::PelagoSlot2 => "Pelago 2",
            Self::PelagoSlot3 => "Pelago 3",
        }
    }
}

struct PersistedState {
    sfmt: RngWrapper<Sfmt>,
    show_view: ShowView,
    view: Gen7View,
    main_menu: Menu<14, Gen7View>,
}

unsafe fn get_state() -> &'static mut PersistedState {
    static mut STATE: Lazy<PersistedState> = Lazy::new(|| PersistedState {
        sfmt: RngWrapper::default(),
        show_view: ShowView::default(),
        view: Gen7View::MainMenu,
        main_menu: Menu::new([
            MenuOption::new(Gen7View::Rng),
            MenuOption::new(Gen7View::Daycare),
            MenuOption::new(Gen7View::WildPokemon),
            MenuOption::new(Gen7View::Sos),
            MenuOption::new(Gen7View::PartySlot1),
            MenuOption::new(Gen7View::PartySlot2),
            MenuOption::new(Gen7View::PartySlot3),
            MenuOption::new(Gen7View::PartySlot4),
            MenuOption::new(Gen7View::PartySlot5),
            MenuOption::new(Gen7View::PartySlot6),
            MenuOption::new(Gen7View::Box),
            MenuOption::new(Gen7View::PelagoSlot1),
            MenuOption::new(Gen7View::PelagoSlot2),
            MenuOption::new(Gen7View::PelagoSlot3),
        ]),
    });
    Lazy::force_mut(&mut STATE)
}

fn run_frame(reader: Gen7Reader) {
    pnp::set_print_max_len(22);

    let init_seed: u32 = reader.init_seed();
    let sfmt_state: u64 = reader.sfmt_state();

    // This is safe as long as this is guaranteed to run single threaded.
    // A lock hinders performance too much on a 3ds.
    let state = unsafe { get_state() };

    state.sfmt.reinit_if_needed(init_seed);
    state.sfmt.update_advances(sfmt_state);

    if !state.show_view.check() {
        return;
    }

    state.view = state.main_menu.next_view(Gen7View::MainMenu, state.view);

    draw_header(Gen7View::MainMenu, state.view);

    match state.view {
        Gen7View::Rng => draw_rng(&reader, &state.sfmt),
        Gen7View::Daycare => draw_daycare(&reader),
        Gen7View::WildPokemon => draw_pkx(&reader.wild_pkm()),
        Gen7View::Sos => draw_sos(&reader),
        Gen7View::PartySlot1 => draw_pkx(&reader.party_pkm(0)),
        Gen7View::PartySlot2 => draw_pkx(&reader.party_pkm(1)),
        Gen7View::PartySlot3 => draw_pkx(&reader.party_pkm(2)),
        Gen7View::PartySlot4 => draw_pkx(&reader.party_pkm(3)),
        Gen7View::PartySlot5 => draw_pkx(&reader.party_pkm(4)),
        Gen7View::PartySlot6 => draw_pkx(&reader.party_pkm(5)),
        Gen7View::Box => draw_pkx(&reader.box_pkm()),
        Gen7View::PelagoSlot1 => draw_pkx(&reader.pelago_pkm(0)),
        Gen7View::PelagoSlot2 => draw_pkx(&reader.pelago_pkm(1)),
        Gen7View::PelagoSlot3 => draw_pkx(&reader.pelago_pkm(2)),
        Gen7View::MainMenu => {
            state.main_menu.update_view();
            state.main_menu.draw();
        }
    }
}

pub fn run_sm_frame() {
    run_frame(Gen7Reader::sm())
}

pub fn run_usum_frame() {
    run_frame(Gen7Reader::usum())
}
