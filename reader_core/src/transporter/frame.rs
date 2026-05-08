use super::{
    draw::{draw_header, draw_pkx, draw_rng, PkxType},
    reader::TransporterReader,
    rng::TransporterRng,
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
enum TransporterView {
    MainMenu,
    Pokemon,
    Rng,
    HelpMenu,
}

struct PersistedState {
    rng: TransporterRng,
    show_view: ShowView,
    view: TransporterView,
    main_menu: Menu<TransporterView>,
    help_menu: HelpMenu,
    pokemon_menu: SubMenu,
}

const MENU: &[MenuOption<TransporterView>] = &[
    MenuOption::new(TransporterView::Rng, "RNG"),
    MenuOption::new(TransporterView::Pokemon, "Pokemon"),
    MenuOption::new(TransporterView::HelpMenu, "Help"),
];

unsafe fn get_state() -> &'static mut PersistedState {
    static mut STATE: Lazy<PersistedState> = Lazy::new(|| PersistedState {
        rng: TransporterRng::default(),
        show_view: ShowView::default(),
        view: TransporterView::MainMenu,
        pokemon_menu: SubMenu::new(1, 30),
        help_menu: HelpMenu::default(),
        main_menu: Menu::new(MENU),
    });
    Lazy::force_mut(&mut STATE)
}

pub fn run_frame() {
    pnp::set_print_max_len(24);

    let reader = TransporterReader::new();

    // This is safe as long as this is guaranteed to run single threaded.
    // A lock hinders performance too much on a 3ds.
    let state = unsafe { get_state() };

    state.rng.update(&reader);

    if !state.show_view.check() {
        return;
    }

    let is_locked = state.main_menu.update_lock();
    state.view = state.main_menu.next_view(TransporterView::MainMenu, state.view);
    draw_header(TransporterView::MainMenu, state.view, is_locked);

    match state.view {
        TransporterView::Rng => draw_rng(&state.rng),
        TransporterView::Pokemon => {
            let slot = state.pokemon_menu.update_and_draw(is_locked);
            draw_pkx(&reader.transported_pkm((slot - 1) as u32), PkxType::Tame);
        }
        TransporterView::HelpMenu => state.help_menu.update_and_draw(is_locked),
        TransporterView::MainMenu => {
            state.main_menu.update_view();
            state.main_menu.draw();
        }
    }
}
