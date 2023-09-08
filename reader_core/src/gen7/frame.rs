use super::{
    draw::{draw_citra_info, draw_daycare, draw_header, draw_pkx, draw_rng, draw_sos},
    reader::Gen7Reader,
};
use crate::{
    pnp,
    rng::{RngWrapper, Sfmt},
    utils::{
        menu::{Menu, MenuOption, MenuOptionValue},
        sub_menu::SubMenu,
        ShowView,
    },
};
use once_cell::unsync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gen7View {
    MainMenu,
    Rng,
    Daycare,
    WildPokemon,
    Sos,
    Party,
    Box,
    Pelago,
    Citra,
}

impl MenuOptionValue for Gen7View {
    fn get_label(view: Self) -> &'static str {
        match view {
            Self::MainMenu => "Main Menu",
            Self::Rng => "RNG",
            Self::Daycare => "Daycare",
            Self::WildPokemon => "Wild",
            Self::Sos => "SOS",
            Self::Party => "Party",
            Self::Box => "Box",
            Self::Pelago => "Pelago",
            Self::Citra => "Citra",
        }
    }
}

struct PersistedState {
    sfmt: RngWrapper<Sfmt>,
    show_view: ShowView,
    view: Gen7View,
    main_menu: Menu<8, Gen7View>,
    party_menu: SubMenu<1, 6>,
    pelago_menu: SubMenu<1, 3>,
}

unsafe fn get_state() -> &'static mut PersistedState {
    static mut STATE: Lazy<PersistedState> = Lazy::new(|| PersistedState {
        sfmt: RngWrapper::default(),
        show_view: ShowView::default(),
        view: Gen7View::MainMenu,
        party_menu: SubMenu::default(),
        pelago_menu: SubMenu::default(),
        main_menu: Menu::new([
            MenuOption::new(Gen7View::Rng),
            MenuOption::new(Gen7View::Daycare),
            MenuOption::new(Gen7View::WildPokemon),
            MenuOption::new(Gen7View::Sos),
            MenuOption::new(Gen7View::Party),
            MenuOption::new(Gen7View::Box),
            MenuOption::new(Gen7View::Pelago),
            MenuOption::new(Gen7View::Citra),
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

    let is_locked = state.main_menu.update_lock();
    state.view = state.main_menu.next_view(Gen7View::MainMenu, state.view);
    draw_header(Gen7View::MainMenu, state.view, is_locked);

    match state.view {
        Gen7View::Rng => draw_rng(&reader, &state.sfmt),
        Gen7View::Daycare => draw_daycare(&reader),
        Gen7View::WildPokemon => draw_pkx(&reader.wild_pkm()),
        Gen7View::Sos => draw_sos(&reader),
        Gen7View::Box => draw_pkx(&reader.box_pkm()),
        Gen7View::Citra => draw_citra_info(&reader),
        Gen7View::Party => {
            let slot = state.party_menu.update_and_draw(is_locked);
            draw_pkx(&reader.party_pkm((slot - 1) as u32));
        }
        Gen7View::Pelago => {
            let slot = state.pelago_menu.update_and_draw(is_locked);
            draw_pkx(&reader.pelago_pkm((slot - 1) as u32))
        }
        Gen7View::MainMenu => {
            state.main_menu.update_view();
            state.main_menu.draw();
        }
    }
}

pub fn run_sm_frame() {
    let reader = Gen7Reader::sm();
    run_frame(reader)
}

pub fn run_usum_frame() {
    let reader = Gen7Reader::usum();
    run_frame(reader)
}
