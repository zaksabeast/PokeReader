use crate::ctr::get_current_process_id;
use crate::ctr::res::{CtrResult, parse_res};

#[allow(non_camel_case_types, dead_code)]
#[derive(Default)]
#[repr(C, packed)]
struct FS_ProgramInfo {
    title_id: u64,
    media_type: u8,
    padding: [u8; 7],
}

#[allow(non_camel_case_types, dead_code)]
#[derive(Default)]
#[repr(C, packed)]
struct FS_ProductInfo {
    produce_code: [u8; 0x10],
    company_code: [u8; 0x2],
    revision_version: u16,
}

#[cfg(target_os = "horizon")]
extern "C" {
    fn fsInit() -> i32;
    fn fsExit();
    fn FSUSER_GetProgramLaunchInfo(info: *mut FS_ProgramInfo, process_id: u32) -> i32;
    fn FSUSER_GetProductInfo(info: *mut FS_ProductInfo, process_id: u32) -> i32;
}

#[cfg(not(target_os = "horizon"))]
#[allow(non_snake_case)]
mod stubs {
    use super::{FS_ProductInfo, FS_ProgramInfo};

    pub unsafe fn fsInit() -> i32 {
        0
    }
    pub unsafe fn fsExit() {}
    pub unsafe fn FSUSER_GetProgramLaunchInfo(_info: *mut FS_ProgramInfo, _process_id: u32) -> i32 {
        0
    }
    pub unsafe fn FSUSER_GetProductInfo(_info: *mut FS_ProductInfo, _process_id: u32) -> i32 {
        0
    }
}

#[cfg(not(target_os = "horizon"))]
use stubs::*;

pub struct ProgramInfo {
    pub title_id: u64,
    pub revision_version: u16,
}

pub fn get_program_info() -> CtrResult<ProgramInfo> {
    unsafe {
        fsInit();
        let result = get_program_info_impl();
        fsExit();
        result
    }
}

unsafe fn get_program_info_impl() -> CtrResult<ProgramInfo> {
    let process_id = get_current_process_id()?;

    let mut program_info = FS_ProgramInfo::default();
    let res = FSUSER_GetProgramLaunchInfo(&mut program_info as *mut FS_ProgramInfo, process_id);
    parse_res(res)?;

    let mut product_info = FS_ProductInfo::default();
    // Citra can't reliably report the product info, and its version is detected separately,
    // so a failure here leaves the revision version as 0 instead of being fatal.
    let _ = FSUSER_GetProductInfo(&mut product_info as *mut FS_ProductInfo, process_id);

    Ok(ProgramInfo {
        title_id: program_info.title_id,
        revision_version: product_info.revision_version,
    })
}
