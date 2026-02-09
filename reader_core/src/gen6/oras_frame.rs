use super::{
    draw::{
        draw_daycare, draw_dex_nav, draw_header, draw_mirage_spot, draw_pkx, draw_rng, draw_seed_rng, PkxType,
    },
    reader::Gen6Reader,
    rng::Gen6Rng,
};
use crate::{
    pnp,
    utils::{
        help_menu::HelpMenu,
        menu::{Menu, MenuOption},
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
    MirageSpot,
    SeedRng,
    HelpMenu,
}

struct PersistedState {
    rng: Gen6Rng,
    show_view: ShowView,
    view: OrasView,
    main_menu: Menu<OrasView>,
    party_menu: SubMenu,
    wild_menu: SubMenu,
    help_menu: HelpMenu,
}

const MENU: &'static [MenuOption<OrasView>] = &[
    MenuOption::new(OrasView::Rng, "RNG"),
    MenuOption::new(OrasView::Daycare1, "Daycare"),
    MenuOption::new(OrasView::Daycare2, "Daycare 2"),
    MenuOption::new(OrasView::Wild, "Wild"),
    MenuOption::new(OrasView::DexNav, "DexNav"),
    MenuOption::new(OrasView::Party, "Party"),
    MenuOption::new(OrasView::MirageSpot, "Mirage Spot"),
    MenuOption::new(OrasView::SeedRng, "Seed RNG"),
    MenuOption::new(OrasView::HelpMenu, "Help"),
];

unsafe fn get_state() -> &'static mut PersistedState {
    static mut STATE: Lazy<PersistedState> = Lazy::new(|| PersistedState {
        rng: Gen6Rng::default(),
        show_view: ShowView::default(),
        view: OrasView::MainMenu,
        party_menu: SubMenu::new(1, 6),
        wild_menu: SubMenu::new(1, 5),
        help_menu: HelpMenu::default(),
        main_menu: Menu::new(MENU),
    });
    Lazy::force_mut(&mut STATE)
}

pub fn run_oras_frame() {
    pnp::set_print_max_len(23);

    let reader = Gen6Reader::oras();

    // This is safe as long as this is guaranteed to run single threaded.
    // A lock hinders performance too much on a 3ds.
    let state = unsafe { get_state() };

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
        OrasView::Wild => {
            let slot = state.wild_menu.update_and_draw(is_locked);
            draw_pkx(&reader.wild_pkm((slot - 1) as u32), PkxType::Wild);
        }
        OrasView::DexNav => draw_dex_nav(&reader, &state.rng),
        OrasView::Party => {
            let slot = state.party_menu.update_and_draw(is_locked);
            draw_pkx(&reader.party_pkm((slot - 1) as u32), PkxType::Tame);
        }
        OrasView::SeedRng => draw_seed_rng(&reader, &state.rng),
        OrasView::MirageSpot => draw_mirage_spot(&reader),
        OrasView::HelpMenu => state.help_menu.update_and_draw(is_locked),
        OrasView::MainMenu => {
            state.main_menu.update_view();
            state.main_menu.draw();
        }
    }
}
