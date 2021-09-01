use ctr::{res::CtrResult, sysmodule::server::ServiceContext};

use super::display::Screen;

pub struct PkrdServiceContext {
    pub screen: Screen,
}

impl PkrdServiceContext {
    pub fn new() -> CtrResult<Self> {
        Ok(Self {
            screen: Screen::new(),
        })
    }
}

impl ServiceContext for PkrdServiceContext {}
