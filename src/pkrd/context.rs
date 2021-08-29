use ctr::{res::CtrResult, sysmodule::server::ServiceContext};

pub struct PkrdServiceContext {}

impl PkrdServiceContext {
    pub fn new() -> CtrResult<Self> {
        Ok(Self {})
    }
}

impl ServiceContext for PkrdServiceContext {}
