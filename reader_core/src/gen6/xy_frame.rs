use super::{
    draw::{draw_daycare, draw_header, draw_pkx, draw_radar, draw_rng},
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

pub fn init_xy() {
    Gen6Reader::xy().patch_inital_seed_read();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyView {
    MainMenu,
    Rng,
    Daycare,
    Wild,
    Radar,
    Party,
}

impl MenuOptionValue for XyView {
    fn get_label(view: Self) -> &'static str {
        match view {
            Self::MainMenu => "Main Menu",
            Self::Rng => "RNG",
            Self::Daycare => "Daycare",
            Self::Wild => "Wild",
            Self::Radar => "Radar",
            Self::Party => "Party",
        }
    }
}

struct PersistedState {
    rng: Gen6Rng,
    show_view: ShowView,
    view: XyView,
    main_menu: Menu<5, XyView>,
    party_menu: SubMenu<1, 6>,
}

unsafe fn get_state() -> &'static mut PersistedState {
    static mut STATE: Lazy<PersistedState> = Lazy::new(|| PersistedState {
        rng: Gen6Rng::default(),
        show_view: ShowView::default(),
        view: XyView::MainMenu,
        party_menu: SubMenu::default(),
        main_menu: Menu::new([
            MenuOption::new(XyView::Rng),
            MenuOption::new(XyView::Daycare),
            MenuOption::new(XyView::Wild),
            MenuOption::new(XyView::Radar),
            MenuOption::new(XyView::Party),
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

    state.rng.update(&reader);

    if !state.show_view.check() {
        return;
    }

    let is_locked = state.main_menu.update_lock();
    state.view = state.main_menu.next_view(XyView::MainMenu, state.view);
    draw_header(XyView::MainMenu, state.view, is_locked);

    match state.view {
        XyView::Rng => draw_rng(&reader, &state.rng),
        XyView::Daycare => draw_daycare(&reader.daycare1()),
        XyView::Wild => draw_pkx(&reader.wild_pkm()),
        XyView::Radar => draw_radar(&reader, &state.rng),
        XyView::Party => {
            let slot = state.party_menu.update_and_draw(is_locked);
            draw_pkx(&reader.party_pkm((slot - 1) as u32));
        }
        XyView::MainMenu => {
            state.main_menu.update_view();
            state.main_menu.draw();
        }
    }
}
