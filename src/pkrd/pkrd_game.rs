use super::{context::PkrdServiceContext, game_hook};
use alloc::format;
use core::sync::atomic::{AtomicU32, Ordering};
use ctr::{
    ipc::{ThreadCommandBuilder, ThreadCommandParser},
    log,
    res::GenericResultCode,
    sysmodule::server::RequestHandlerResult,
    DebugProcess,
};
use num_enum::IntoPrimitive;

static PKRD_HANDLE: AtomicU32 = AtomicU32::new(0);

pub fn get_raw_pkrd_handle() -> u32 {
    PKRD_HANDLE.load(Ordering::Relaxed)
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
    mut command_parser: ThreadCommandParser,
    _session_index: usize,
) -> RequestHandlerResult {
    let command_id = command_parser.get_command_id();

    log(&format!(
        "[CMD] pkrd:game[{:x}][{:x}]",
        command_id,
        command_parser.get_header()
    ));

    match command_id.into() {
        PkrdGameCommand::Setup => {
            command_parser.pop();
            let raw_handle = command_parser.pop();
            PKRD_HANDLE.store(raw_handle, Ordering::Relaxed);

            let mut command = ThreadCommandBuilder::new(command_id);
            command.push(GenericResultCode::Success);
            Ok(command.build())
        }
        PkrdGameCommand::RunGameHook => {
            let stack_pointer = command_parser.pop();

            // The game debug session needs to start and end in this scope so rosalina can obtain one too
            let game = DebugProcess::new(0x0004000000055E00)?;

            let [screen_id, _swap, frame_buffer_a, _frame_buffer_b, stride, format] =
                game.read::<[u32; 6]>(stack_pointer - 16)?;
            let is_top_screen = screen_id == 0;

            context
                .screen
                .set_context(is_top_screen, frame_buffer_a, stride, format)?;

            #[allow(unused_must_use)]
            {
                // Ignore the result since the game ignores it anyways
                // and we don't want an error to prevent the game from running
                game_hook::run_hook(&mut context.screen);
            }

            // Eat events after writing to the screen
            game.eat_events()?;

            let mut command = ThreadCommandBuilder::new(command_id);
            command.push(GenericResultCode::Success);
            Ok(command.build())
        }
        _ => {
            let mut command = ThreadCommandBuilder::new(PkrdGameCommand::InvalidCommand);
            command.push(GenericResultCode::InvalidCommand);
            Ok(command.build())
        }
    }
}
