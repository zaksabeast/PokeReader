use super::{context::PkrdServiceContext, display::Screen, frame_pause::handle_frame_pause};
use crate::log;
use alloc::format;
use core::{
    mem, slice,
    sync::atomic::{AtomicU32, Ordering},
};
use ctr::{hid, hid::InterfaceDevice, ipc, res::GenericResultCode, svc, sysmodule::server, Handle};
use num_enum::IntoPrimitive;

static PKRD_HANDLE: AtomicU32 = AtomicU32::new(0);

/// Returns a pkrd:game session handle.
/// This is manually dropped to avoid closing the session handle.
pub fn get_pkrd_session_handle() -> mem::ManuallyDrop<Handle> {
    let raw_handle = PKRD_HANDLE.load(Ordering::Relaxed);
    let handle = raw_handle.into();
    mem::ManuallyDrop::new(handle)
}

#[derive(IntoPrimitive)]
#[repr(u16)]
enum PkrdGameCommand {
    InvalidCommand = 0,
    Setup = 1,
    RunGameHook = 2,
}

impl From<u16> for PkrdGameCommand {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::Setup,
            2 => Self::RunGameHook,
            _ => Self::InvalidCommand,
        }
    }
}

pub fn handle_pkrd_game_request(
    context: &mut PkrdServiceContext,
    mut command_parser: ipc::ThreadCommandParser,
    _session_index: usize,
) -> server::RequestHandlerResult {
    let command_id = command_parser.get_command_id();

    log::debug(&format!(
        "[CMD] pkrd:game[{:x}][{:x}]",
        command_id,
        command_parser.get_header()
    ));

    match command_id.into() {
        PkrdGameCommand::Setup => {
            command_parser.pop();
            let raw_handle = command_parser.pop();
            PKRD_HANDLE.store(raw_handle, Ordering::Relaxed);

            let mut command = ipc::ThreadCommandBuilder::new(command_id);
            command.push(GenericResultCode::Success);
            Ok(command.build())
        }
        PkrdGameCommand::RunGameHook => {
            // Check to make sure we're getting what we're expecting
            command_parser.validate_header(PkrdGameCommand::RunGameHook, 6, 0)?;

            // Get screen props
            let frame_buffer = command_parser.pop();
            let screen_id = command_parser.pop();
            let stride = command_parser.pop();
            let format = command_parser.pop();
            let is_top_screen = screen_id == 0;

            // Get heap
            let heap_ptr = command_parser.pop() as *mut u8;
            let heap_len = command_parser.pop_usize();

            // We're trusting our patch works and nothing else is using this command
            let physical_heap_ptr = svc::convert_pa_to_uncached_pa(heap_ptr)?;
            let heap = unsafe { slice::from_raw_parts_mut(physical_heap_ptr, heap_len) };

            let (game, screen) = context.get_or_initialize_game_and_screen()?;

            if let Err(result_code) =
                screen.set_context(is_top_screen, frame_buffer, stride, format)
            {
                log::error(&alloc::format!("Failed screen context {:x}", result_code));
            }

            // The input needs to be scanned here so we can use it in the hook
            hid::Global::scan_input();

            // The game ignores the result of this, and there's not much we can
            // do to handle the error aside from logging.
            if let Err(result_code) = game.run_hook(heap, screen) {
                log::error(&alloc::format!("Failed run_hook: {:x}", result_code));
            }

            handle_frame_pause(context, is_top_screen);

            let mut command = ipc::ThreadCommandBuilder::new(command_id);
            command.push(GenericResultCode::Success);
            Ok(command.build())
        }
        _ => {
            let mut command = ipc::ThreadCommandBuilder::new(PkrdGameCommand::InvalidCommand);
            command.push(GenericResultCode::InvalidCommand);
            Ok(command.build())
        }
    }
}
