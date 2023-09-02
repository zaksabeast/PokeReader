use super::{
    draw::{draw_header, draw_pkx, draw_rng},
    reader::TransporterReader,
    rng::TransporterRng,
};
use crate::{
    menu::{Menu, MenuOption, MenuOptionValue},
    pnp,
    utils::ShowView,
};
use once_cell::unsync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransporterView {
    MainMenu,
    Pokemon,
    Rng,
}

impl MenuOptionValue for TransporterView {
    fn get_label(view: Self) -> &'static str {
        match view {
            Self::MainMenu => "Main Menu",
            Self::Pokemon => "Slot 1 Pokemon",
            Self::Rng => "RNG",
        }
    }
}

struct PersistedState {
    rng: TransporterRng,
    show_view: ShowView,
    patched_init_seed_read: bool,
    view: TransporterView,
    main_menu: Menu<2, TransporterView>,
}

unsafe fn get_state() -> &'static mut PersistedState {
    static mut STATE: Lazy<PersistedState> = Lazy::new(|| PersistedState {
        rng: TransporterRng::default(),
        show_view: ShowView::default(),
        patched_init_seed_read: false,
        view: TransporterView::MainMenu,
        main_menu: Menu::new([
            MenuOption::new(TransporterView::Rng),
            MenuOption::new(TransporterView::Pokemon),
        ]),
    });
    Lazy::force_mut(&mut STATE)
}

pub fn run_frame() {
    pnp::set_print_max_len(24);

    let reader = TransporterReader::new();

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
    state.view = state
        .main_menu
        .next_view(TransporterView::MainMenu, state.view);

    draw_header(
        TransporterView::MainMenu,
        state.view,
        state.main_menu.is_locked(),
    );

    match state.view {
        TransporterView::Pokemon => draw_pkx(&reader.transported_pkm(0)),
        TransporterView::Rng => draw_rng(&state.rng),
        TransporterView::MainMenu => {
            state.main_menu.update_view();
            state.main_menu.draw();
        }
    }
}
