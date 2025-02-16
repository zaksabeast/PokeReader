use super::{
    draw::{draw_header, draw_non_cfw, draw_pkx, draw_rng},
    hook::{measured_div, reset_rng_advance},
    reader::Gen2Reader,
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
enum CrystalView {
    MainMenu,
    Rng,
    Party,
    Wild,
    NonCfw,
}

impl MenuOptionValue for CrystalView {
    fn get_label(view: Self) -> &'static str {
        match view {
            Self::MainMenu => "Main Menu",
            Self::Rng => "RNG",
            Self::Party => "Party",
            Self::Wild => "Wild",
            Self::NonCfw => "Non-CFW",
        }
    }
}

struct PersistedState {
    frame: usize,
    show_view: ShowView,
    view: CrystalView,
    main_menu: Menu<4, CrystalView>,
    party_menu: SubMenu<1, 6>,
}

unsafe fn get_state() -> &'static mut PersistedState {
    static mut STATE: Lazy<PersistedState> = Lazy::new(|| PersistedState {
        frame: 0,
        show_view: ShowView::default(),
        view: CrystalView::MainMenu,
        party_menu: SubMenu::default(),
        main_menu: Menu::new([
            MenuOption::new(CrystalView::Rng),
            MenuOption::new(CrystalView::Party),
            MenuOption::new(CrystalView::Wild),
            MenuOption::new(CrystalView::NonCfw),
        ]),
    });
    Lazy::force_mut(&mut STATE)
}

pub fn run_frame() {
    pnp::set_print_max_len(22);

    let reader = Gen2Reader::crystal();

    // This is safe as long as this is guaranteed to run single threaded.
    // A lock hinders performance too much on a 3ds.
    let state = unsafe { get_state() };

    state.frame = match (measured_div(), reader.rng_state()) {
        (0x0101, 0x01ff) => {
            reset_rng_advance();
            0
        }
        _ => state.frame.wrapping_add(1),
    };

    if !state.show_view.check() {
        return;
    }

    let is_locked = state.main_menu.update_lock();
    state.view = state.main_menu.next_view(CrystalView::MainMenu, state.view);
    draw_header(CrystalView::MainMenu, state.view, is_locked);

    match state.view {
        CrystalView::Rng => draw_rng(&reader),
        CrystalView::Wild => draw_pkx(&reader.wild()),
        CrystalView::Party => {
            let slot = state.party_menu.update_and_draw(is_locked);
            draw_pkx(&reader.party((slot - 1) as u8));
        }
        CrystalView::NonCfw => draw_non_cfw(&reader, state.frame),
        CrystalView::MainMenu => {
            state.main_menu.update_view();
            state.main_menu.draw();
        }
    }
}
