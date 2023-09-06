use super::{
    draw::{draw_daycare, draw_dex_nav, draw_header, draw_pkx, draw_rng},
    reader::Gen6Reader,
    rng::Gen6Rng,
};
use crate::{
    pnp,
    utils::{
        menu::{Menu, MenuOption, MenuOptionValue},
        sub_menu::SubMenu,
        ShowView,
    },
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
    Party,
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
            Self::Party => "Party",
        }
    }
}

struct PersistedState {
    rng: Gen6Rng,
    show_view: ShowView,
    view: OrasView,
    main_menu: Menu<6, OrasView>,
    party_menu: SubMenu<1, 6>,
    patched_init_seed_read: bool,
}

unsafe fn get_state() -> &'static mut PersistedState {
    static mut STATE: Lazy<PersistedState> = Lazy::new(|| PersistedState {
        rng: Gen6Rng::default(),
        show_view: ShowView::default(),
        view: OrasView::MainMenu,
        patched_init_seed_read: false,
        party_menu: SubMenu::default(),
        main_menu: Menu::new([
            MenuOption::new(OrasView::Rng),
            MenuOption::new(OrasView::Daycare1),
            MenuOption::new(OrasView::Daycare2),
            MenuOption::new(OrasView::Wild),
            MenuOption::new(OrasView::DexNav),
            MenuOption::new(OrasView::Party),
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

    let is_locked = state.main_menu.update_lock();
    state.view = state.main_menu.next_view(OrasView::MainMenu, state.view);
    draw_header(OrasView::MainMenu, state.view, is_locked);

    match state.view {
        OrasView::Rng => draw_rng(&reader, &state.rng),
        OrasView::Daycare1 => draw_daycare(&reader.daycare1()),
        OrasView::Daycare2 => draw_daycare(&reader.daycare2()),
        OrasView::Wild => draw_pkx(&reader.wild_pkm()),
        OrasView::DexNav => draw_dex_nav(&reader, &state.rng),
        OrasView::Party => {
            let slot = state.party_menu.update_and_draw(is_locked);
            draw_pkx(&reader.party_pkm((slot - 1) as u32));
        }
        OrasView::MainMenu => {
            state.main_menu.update_view();
            state.main_menu.draw();
        }
    }
}
