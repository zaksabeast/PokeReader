// Thanks to https://gitlab.com/thepixellizeross/ctrpluginframework/-/blob/develop/Library/source/plgldr.c

use super::ipc::{Handle, get_thread_command_buffer, ipc_make_header};
use super::svc::{
    ArbitrationType, arbitrate_address, connect_to_port, create_thread, exit_thread, send_sync_request,
    sleep_thread,
};
use crate::ctr::res::CtrResult;
use alloc::boxed::Box;
use num_enum::FromPrimitive;

#[repr(C)]
pub struct PluginHeader {
    pub magic: u32,
    pub version: u32,
    pub heap_va: u32,
    pub heap_size: u32,
    pub exe_size: u32,
    pub is_default_plugin: u32,
    pub plgldr_event: *mut i32,
    pub plgldr_reply: *mut i32,
    pub notify_home_event: u8,
    pub padding: [u8; 7],
    pub wait_for_reply_timeout: u64,
    pub reserved: [u32; 20],
    pub config: [u32; 32],
}

const WAIT_FOR_REPLY_TIMEOUT_NS: u64 = 1_000_000_000;

static mut PLGLDR_HANDLE: Handle = Handle(0);
static mut PLGLDR_ARBITER: Handle = Handle(0);
static mut PLG_EVENT: *mut i32 = core::ptr::null_mut();
static mut PLG_REPLY: *mut i32 = core::ptr::null_mut();

pub fn get_header() -> &'static mut PluginHeader {
    unsafe { &mut *(0x07000000 as *mut PluginHeader) }
}

fn get_arbiter(service: Handle) -> CtrResult<Handle> {
    let cmd_buf = get_thread_command_buffer();
    cmd_buf[0] = ipc_make_header(9, 0, 0);

    match send_sync_request(service) {
        Ok(_) | Err(-0x1f1fe40c) => Ok(()), // Succeeded or not implemented (citra)
        Err(err) => Err(err),
    }?;

    Ok(Handle(cmd_buf[3]))
}

pub fn init() -> CtrResult<()> {
    let handle = connect_to_port("plg:ldr")?;
    unsafe { PLGLDR_HANDLE = handle };

    let handle = get_arbiter(handle)?;
    unsafe { PLGLDR_ARBITER = handle };

    let header = get_header();
    unsafe {
        PLG_EVENT = header.plgldr_event;
        PLG_REPLY = header.plgldr_reply;
        header.wait_for_reply_timeout = WAIT_FOR_REPLY_TIMEOUT_NS;
    };

    Ok(())
}

#[derive(Default, FromPrimitive)]
#[repr(i32)]
enum PlgEvent {
    #[default]
    Wait = -1,
    Ok = 0,
    SleepEntry = 1,
    SleepExit = 2,
    AboutToSwap = 3,
    AboutToExit = 4,
    HomeEnter = 5,
    HomeExit = 6,
}

fn fetch_event() -> PlgEvent {
    if unsafe { PLG_EVENT.is_null() } {
        return PlgEvent::Wait;
    }
    let event = unsafe { *PLG_EVENT };
    event.into()
}

fn send_event(event: PlgEvent) {
    if unsafe { PLG_EVENT.is_null() } {
        return;
    }
    unsafe { *PLG_EVENT = event as i32 };
}

fn send_reply(reply: PlgEvent) {
    if unsafe { PLG_REPLY.is_null() } {
        return;
    }
    unsafe { *PLG_REPLY = reply as i32 };
}

fn handle_event() {
    use PlgEvent::*;

    let event = fetch_event();
    send_reply(Ok);

    if matches!(event, Wait | Ok | SleepEntry | SleepExit) {
        send_event(Ok);
        return;
    }

    let plg_reply = unsafe { PLG_REPLY } as u32;
    let arbiter = unsafe { PLGLDR_ARBITER };

    let _ = arbitrate_address(arbiter, plg_reply, ArbitrationType::Signal, 1, 0);

    match event {
        AboutToSwap => {
            let plg_event = unsafe { PLG_EVENT } as u32;
            send_event(Wait);
            let _ = arbitrate_address(arbiter, plg_event, ArbitrationType::WaitIfLessThan, Ok as i32, 0);
        }
        AboutToExit => exit_thread(),
        _ => {
            send_event(Ok);
        }
    }
}

extern "C" fn handle_events() {
    loop {
        handle_event();
        sleep_thread(250000000);
    }
}

#[repr(C, align(8))]
pub struct ThreadStack {
    bytes: [u8; 0x1000],
}

impl ThreadStack {
    pub fn new() -> Box<Self> {
        Box::new(Self { bytes: [0; 0x1000] })
    }
}

pub fn start_ack_events_thread() {
    let mut stack = ThreadStack::new();

    let stack_top = unsafe { stack.bytes.as_mut_ptr().add(stack.bytes.len()) };

    let _ = create_thread(handle_events, stack_top, 0x31, -2);

    Box::leak(stack);
}
