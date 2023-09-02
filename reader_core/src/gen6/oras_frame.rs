use super::{
    draw::{draw_daycare, draw_dex_nav, draw_header, draw_pkx, draw_rng},
    reader::Gen6Reader,
    rng::Gen6Rng,
};
use crate::{
    menu::{Menu, MenuOption, MenuOptionValue},
    pnp,
    utils::ShowView,
};
use once_cell::unsync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OrasView {
    MainMenu,
    Rng,
    Daycare1,
    Daycare2,
    Wild,
    DexNav,
    PartySlot1,
    PartySlot2,
    PartySlot3,
    PartySlot4,
    PartySlot5,
    PartySlot6,
}

impl MenuOptionValue for OrasView {
    fn get_label(view: Self) -> &'static str {
        match view {
            Self::MainMenu => "Main Menu",
            Self::Rng => "RNG",
            Self::Daycare1 => "Daycare",
            Self::Daycare2 => "Daycare 2",
            Self::Wild => "Wild",
            Self::DexNav => "DexNav",
            Self::PartySlot1 => "Party 1",
            Self::PartySlot2 => "Party 2",
            Self::PartySlot3 => "Party 3",
            Self::PartySlot4 => "Party 4",
            Self::PartySlot5 => "Party 5",
            Self::PartySlot6 => "Party 6",
        }
    }
}

struct PersistedState {
    rng: Gen6Rng,
    show_view: ShowView,
    view: OrasView,
    main_menu: Menu<11, OrasView>,
    patched_init_seed_read: bool,
}

unsafe fn get_state() -> &'static mut PersistedState {
    static mut STATE: Lazy<PersistedState> = Lazy::new(|| PersistedState {
        rng: Gen6Rng::default(),
        show_view: ShowView::default(),
        view: OrasView::MainMenu,
        patched_init_seed_read: false,
        main_menu: Menu::new([
            MenuOption::new(OrasView::Rng),
            MenuOption::new(OrasView::Daycare1),
            MenuOption::new(OrasView::Daycare2),
            MenuOption::new(OrasView::Wild),
            MenuOption::new(OrasView::DexNav),
            MenuOption::new(OrasView::PartySlot1),
            MenuOption::new(OrasView::PartySlot2),
            MenuOption::new(OrasView::PartySlot3),
            MenuOption::new(OrasView::PartySlot4),
            MenuOption::new(OrasView::PartySlot5),
            MenuOption::new(OrasView::PartySlot6),
        ]),
    });
    Lazy::force_mut(&mut STATE)
}

pub fn run_oras_frame() {
    pnp::set_print_max_len(23);

    let reader = Gen6Reader::oras();

    // This is safe as long as this is guaranteed to run single threaded.
    // A lock hinders performance too much on a 3ds.
    let state = unsafe { get_state() };

    if !state.patched_init_seed_read {
        reader.patch_inital_seed_read();
        state.patched_init_seed_read = true;
    }

    state.rng.update(&reader);

    if !state.show_view.check() {
        return;
    }

    state.main_menu.update_lock();
    state.view = state.main_menu.next_view(OrasView::MainMenu, state.view);

    draw_header(OrasView::MainMenu, state.view, state.main_menu.is_locked());

    match state.view {
        OrasView::Rng => draw_rng(&reader, &state.rng),
        OrasView::Daycare1 => draw_daycare(&reader.daycare1()),
        OrasView::Daycare2 => draw_daycare(&reader.daycare2()),
        OrasView::Wild => draw_pkx(&reader.wild_pkm()),
        OrasView::DexNav => draw_dex_nav(&reader, &state.rng),
        OrasView::PartySlot1 => draw_pkx(&reader.party_pkm(0)),
        OrasView::PartySlot2 => draw_pkx(&reader.party_pkm(1)),
        OrasView::PartySlot3 => draw_pkx(&reader.party_pkm(2)),
        OrasView::PartySlot4 => draw_pkx(&reader.party_pkm(3)),
        OrasView::PartySlot5 => draw_pkx(&reader.party_pkm(4)),
        OrasView::PartySlot6 => draw_pkx(&reader.party_pkm(5)),
        OrasView::MainMenu => {
            state.main_menu.update_view();
            state.main_menu.draw();
        }
    }
}
