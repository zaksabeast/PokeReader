use super::{
    draw::{draw_daycare, draw_header, draw_pkx, draw_radar, draw_rng},
    reader::Gen6Reader,
    rng::Gen6Rng,
};
use crate::{
    menu::{Menu, MenuOption, MenuOptionValue},
    utils::ShowView,
};
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyView {
    MainMenu,
    Rng,
    Daycare,
    Wild,
    Radar,
    PartySlot1,
    PartySlot2,
    PartySlot3,
    PartySlot4,
    PartySlot5,
    PartySlot6,
}

impl MenuOptionValue for XyView {
    fn get_label(view: Self) -> &'static str {
        match view {
            Self::MainMenu => "Main Menu",
            Self::Rng => "RNG",
            Self::Daycare => "Daycare",
            Self::Wild => "Wild",
            Self::Radar => "Radar",
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
    view: XyView,
    main_menu: Menu<10, XyView>,
    patched_init_seed_read: bool,
}

unsafe fn get_state() -> &'static mut PersistedState {
    static mut STATE: Lazy<PersistedState> = Lazy::new(|| PersistedState {
        rng: Gen6Rng::default(),
        show_view: ShowView::default(),
        view: XyView::MainMenu,
        patched_init_seed_read: false,
        main_menu: Menu::new([
            MenuOption::new(XyView::Rng),
            MenuOption::new(XyView::Daycare),
            MenuOption::new(XyView::Wild),
            MenuOption::new(XyView::Radar),
            MenuOption::new(XyView::PartySlot1),
            MenuOption::new(XyView::PartySlot2),
            MenuOption::new(XyView::PartySlot3),
            MenuOption::new(XyView::PartySlot4),
            MenuOption::new(XyView::PartySlot5),
            MenuOption::new(XyView::PartySlot6),
        ]),
    });
    Lazy::force_mut(&mut STATE)
}

pub fn run_xy_frame() {
    pnp::set_print_max_len(23);

    let reader = Gen6Reader::xy();

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
    state.view = state.main_menu.next_view(XyView::MainMenu, state.view);

    draw_header(XyView::MainMenu, state.view, state.main_menu.is_locked());

    match state.view {
        XyView::Rng => draw_rng(&reader, &state.rng),
        XyView::Daycare => draw_daycare(&reader.daycare1()),
        XyView::Wild => draw_pkx(&reader.wild_pkm()),
        XyView::Radar => draw_radar(&reader, &state.rng),
        XyView::PartySlot1 => draw_pkx(&reader.party_pkm(0)),
        XyView::PartySlot2 => draw_pkx(&reader.party_pkm(1)),
        XyView::PartySlot3 => draw_pkx(&reader.party_pkm(2)),
        XyView::PartySlot4 => draw_pkx(&reader.party_pkm(3)),
        XyView::PartySlot5 => draw_pkx(&reader.party_pkm(4)),
        XyView::PartySlot6 => draw_pkx(&reader.party_pkm(5)),
        XyView::MainMenu => {
            state.main_menu.update_view();
            state.main_menu.draw();
        }
    }
}
