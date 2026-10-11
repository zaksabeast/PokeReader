#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Handle(pub u32);

impl Handle {
    pub const CURRENT_PROCESS: Handle = Handle(0xFFFF8001);
}

#[cfg(target_os = "horizon")]
#[allow(dead_code)]
#[repr(C)]
struct ThreadLocalStorage {
    storage: [u32; 32],
    cmd_buf: [u32; 64],
    static_bufs: [u32; 32],
}

#[cfg(target_os = "horizon")]
fn get_thread_local_storage() -> *mut ThreadLocalStorage {
    let mut ret: *mut u32 = core::ptr::null_mut();
    unsafe {
        core::arch::asm!("mrc p15, 0, {data}, c13, c0, 3", data = out(reg) ret);
    }
    ret as *mut ThreadLocalStorage
}

#[cfg(target_os = "horizon")]
pub fn get_thread_command_buffer() -> &'static mut [u32; 64] {
    let tls = get_thread_local_storage();
    unsafe { &mut (*tls).cmd_buf }
}

pub const fn ipc_make_header(cmd_id: u16, normal_params: u32, translate_params: u32) -> u32 {
    ((cmd_id as u32) << 16) | ((normal_params & 0x3f) << 6) | (translate_params & 0x3f)
}
