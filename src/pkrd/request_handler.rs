use super::{context::PkrdServiceContext, display::Screen, frame_pause::handle_frame_pause, hook};
use crate::{log, pkrd::notification};
use core::{
    mem, slice,
    sync::atomic::{AtomicU32, Ordering},
};
use ctr::{
    ctr_method, hid,
    hid::InterfaceDevice,
    ipc::Handles,
    res::{CtrResult, GenericResultCode, ResultCode},
    svc,
    sysmodule::server::Service,
    Handle,
};
use no_std_io::{EndianRead, EndianWrite};
use num_enum::{FromPrimitive, IntoPrimitive};

static PKRD_HANDLE: AtomicU32 = AtomicU32::new(0);

/// Returns a pkrd:game session handle.
/// This is manually dropped to avoid closing the session handle.
pub fn get_pkrd_session_handle() -> mem::ManuallyDrop<Handle> {
    let raw_handle = PKRD_HANDLE.load(Ordering::Relaxed);
    let handle = raw_handle.into();
    mem::ManuallyDrop::new(handle)
}

#[derive(IntoPrimitive, FromPrimitive)]
#[repr(u16)]
pub enum PkrdGameCommand {
    #[num_enum(default)]
    InvalidCommand = 0,
    Setup = 1,
    RunGameHook = 2,
}

impl Service for PkrdGameCommand {
    const ID: usize = 0;
    const MAX_SESSION_COUNT: i32 = 8;
    const NAME: &'static str = "pkrd:game";
}

#[ctr_method(cmd = "PkrdGameCommand::Setup", normal = 0x1, translate = 0x0)]
fn setup(_context: &mut PkrdServiceContext, _session_index: usize, handles: Handles) -> CtrResult {
    PKRD_HANDLE.store(handles.into_handle().unwrap(), Ordering::Relaxed);
    Ok(())
}

#[derive(EndianRead, EndianWrite)]
struct GameHookParams {
    frame_buffer: u32,
    screen_id: u32,
    stride: u32,
    format: u32,
    heap_ptr: u32,
    heap_len: u32,
}

#[ctr_method(cmd = "PkrdGameCommand::RunGameHook", normal = 0x1, translate = 0x0)]
fn run_game_hook(
    context: &mut PkrdServiceContext,
    _session_index: usize,
    params: GameHookParams,
) -> CtrResult {
    let is_top_screen = params.screen_id == 0;

    if notification::is_new_game_launch() {
        // Get heap
        let heap_ptr = params.heap_ptr as *mut u8;
        let heap_len = params.heap_len as usize;

        // We're trusting our patch works and nothing else is using this command
        let physical_heap_ptr = svc::convert_pa_to_uncached_pa(heap_ptr)?;
        let heap = unsafe { slice::from_raw_parts_mut(physical_heap_ptr, heap_len) };

        // Since this is a physical address, it is static memory
        context.game = hook::get_hooked_process(heap);
    }

    let screen = &mut context.screen;

    if let Err(result_code) = screen.set_context(
        is_top_screen,
        params.frame_buffer,
        params.stride,
        params.format,
    ) {
        log::error(&alloc::format!(
            "Failed screen context {:x}",
            result_code.into_raw()
        ));
    }

    // The input needs to be scanned here so we can use it in the hook
    hid::Global::scan_input();

    let hook_result = context
        .game
        .as_mut()
        .ok_or_else::<ResultCode, fn() -> ResultCode>(|| GenericResultCode::InvalidValue.into())?
        .run_hook(screen);

    // The game ignores the result of this, and there's not much we can
    // do to handle the error aside from logging.
    if let Err(result_code) = hook_result {
        log::error(&alloc::format!(
            "Failed run_hook: {:x}",
            result_code.into_raw()
        ));
    }

    handle_frame_pause(context, is_top_screen);

    Ok(())
}
