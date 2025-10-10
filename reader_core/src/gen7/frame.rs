use super::{
    draw::{PkxType, draw_citra_info, draw_daycare, draw_header, draw_pkx, draw_rng, draw_sos},
    reader::Gen7Reader,
};
use crate::{
    pnp,
    rng::{RngWrapper, Sfmt},
    utils::{
        ShowView,
        help_menu::HelpMenu,
        menu::{Menu, MenuOption, MenuOptionValue},
        sub_menu::SubMenu,
        sub_menu_capture::SubMenuCapture,
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
    HelpMenu,
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
            Self::HelpMenu => "Help",
        }
    }
}

struct PersistedState {
    sfmt: RngWrapper<Sfmt>,
    show_view: ShowView,
    view: Gen7View,
    main_menu: Menu<9, Gen7View>,
    help_menu: HelpMenu,
    wild_menu: SubMenu<1, 4>,
    party_menu: SubMenu<1, 6>,
    sos_menu: SubMenuCapture<1, 4>,
    pelago_menu: SubMenu<1, 3>,
}

unsafe fn get_state() -> &'static mut PersistedState {
    static mut STATE: Lazy<PersistedState> = Lazy::new(|| PersistedState {
        sfmt: RngWrapper::default(),
        show_view: ShowView::default(),
        view: Gen7View::MainMenu,
        party_menu: SubMenu::default(),
        pelago_menu: SubMenu::default(),
        wild_menu: SubMenu::default(),
        sos_menu: SubMenuCapture::default(),
        help_menu: HelpMenu::new(|| {
            pnp::println!("SOS Controls:");
            pnp::println!("[X] + [Right]:");
            pnp::println!("   Set Caller slot to");
            pnp::println!("   the current ally.");
            pnp::println!("   Use this when you");
            pnp::println!("   faint the caller.");
            pnp::println!("");
            pnp::println!("[X] + [Up]/[Down]:");
            pnp::println!("   Manually change");
            pnp::println!("   the caller slot.");
            pnp::println!("   (Not recommended)");
            pnp::println!("");
        }),
        main_menu: Menu::new([
            MenuOption::new(Gen7View::Rng),
            MenuOption::new(Gen7View::Daycare),
            MenuOption::new(Gen7View::WildPokemon),
            MenuOption::new(Gen7View::Sos),
            MenuOption::new(Gen7View::Party),
            MenuOption::new(Gen7View::Box),
            MenuOption::new(Gen7View::Pelago),
            MenuOption::new(Gen7View::Citra),
            MenuOption::new(Gen7View::HelpMenu),
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
        Gen7View::WildPokemon => {
            let slot = state.wild_menu.update_and_draw(is_locked);
            draw_pkx(&reader.wild_pkm((slot - 1) as u32), PkxType::Wild);
        }
        Gen7View::Sos => {
            let prev_caller_slot = state.sos_menu.counter_value();
            let prev_correction_value = state.sos_menu.captured_value();
            let caller_slot = state.sos_menu.update_headless(
                is_locked,
                reader.sos_chain() as u32,
                reader.ally_slot(prev_caller_slot as u32, prev_correction_value) as usize + 1,
            );
            let correction_value = state.sos_menu.captured_value();
            draw_sos(&reader, caller_slot as u32, correction_value);
        }
        Gen7View::Box => draw_pkx(&reader.box_pkm(), PkxType::Tame),
        Gen7View::Citra => draw_citra_info(&reader),
        Gen7View::Party => {
            let slot = state.party_menu.update_and_draw(is_locked);
            draw_pkx(&reader.party_pkm((slot - 1) as u32), PkxType::Tame);
        }
        Gen7View::Pelago => {
            let slot = state.pelago_menu.update_and_draw(is_locked);
            draw_pkx(&reader.pelago_pkm((slot - 1) as u32), PkxType::Wild)
        }
        Gen7View::HelpMenu => state.help_menu.update_and_draw(is_locked),
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
