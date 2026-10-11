use crate::ctr::res::{CtrResult, parse_res};

#[cfg(target_os = "horizon")]
extern "C" {
    fn srvInit() -> i32;
}

#[cfg(not(target_os = "horizon"))]
#[allow(non_snake_case)]
unsafe fn srvInit() -> i32 {
    0
}

pub fn init() -> CtrResult<()> {
    let res = unsafe { srvInit() };
    parse_res(res)
}
