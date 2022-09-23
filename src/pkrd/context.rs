use super::{
    display::{DirectWriteScreen, Screen},
    hook,
};
use crate::PkrdGameCommand;
use alloc::boxed::Box;
use ctr::{match_ctr_route, res::CtrResult, sysmodule::server::ServiceRouter};

pub struct PkrdServiceContext {
    pub screen: DirectWriteScreen,
    pub game: Option<Box<dyn hook::HookedProcess>>,
    pub is_paused: bool,
}

impl PkrdServiceContext {
    pub fn new() -> CtrResult<Self> {
        Ok(Self {
            screen: DirectWriteScreen::new(),
            game: None,
            is_paused: false,
        })
    }
}

impl ServiceRouter for PkrdServiceContext {
    fn accept_session(&mut self, _session_index: usize) {}

    fn close_session(&mut self, _session_index: usize) {}

    fn handle_request(
        &mut self,
        service_id: usize,
        session_index: usize,
    ) -> CtrResult<ctr::ipc::WrittenCommand> {
        match_ctr_route!(
            PkrdServiceContext,
            service_id,
            session_index,
            PkrdGameCommand::Setup,
            PkrdGameCommand::RunGameHook
        )
    }
}
