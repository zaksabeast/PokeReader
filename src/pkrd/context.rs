use super::{
    display::{DirectWriteScreen, Screen},
    hook,
};
use alloc::boxed::Box;
use ctr::{
    pm_dbg,
    res::{CtrResult, GenericResultCode, ResultCode},
    sysmodule::server::ServiceContext,
};

pub struct PkrdServiceContext {
    pub screen: DirectWriteScreen,
    pub game: Option<Box<dyn hook::HookedProcess>>,
}

impl PkrdServiceContext {
    pub fn new() -> CtrResult<Self> {
        Ok(Self {
            screen: DirectWriteScreen::new(),
            game: None,
        })
    }

    fn initialize_game(&mut self) {
        self.game = hook::get_hooked_process();
    }

    pub fn get_or_initialize_game_and_screen(
        &mut self,
    ) -> CtrResult<(&mut Box<dyn hook::HookedProcess>, &mut DirectWriteScreen)> {
        let running_title_id = pm_dbg::get_current_app_info()?.program_info.program_id;

        match &self.game {
            None => self.initialize_game(),
            Some(game) => {
                if game.get_title() != running_title_id {
                    self.initialize_game()
                }
            }
        };

        let game = self
            .game
            .as_mut()
            .ok_or_else::<ResultCode, fn() -> ResultCode>(|| {
                GenericResultCode::InvalidValue.into()
            })?;

        Ok((game, &mut self.screen))
    }
}

impl ServiceContext for PkrdServiceContext {}
