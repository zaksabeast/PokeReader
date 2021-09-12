//! Everything related to the sysmodule.
//!
//! The sysmodule hooks into a supported game when it receives a notification
//! that a game has launched.  After hooking, the game will run a command
//! in the sysmodule every video frame.  The command will read game specific
//! values and show them to the user.
//!
//! # Reading a new value from a game
//! 1. Update a trait in the [reader] module to read the new value.
//! 1. Update anything that implements the trait with new offsets or methods if necessary.
//! 1. Update the [views] module using the trait to show the new value.

/// Sysmodule context, which persists data between frames.
pub mod context;
/// Sysmodule notification handlers.
pub mod notification;
/// Sysmodule request handler.
pub mod request_handler;

/// Everything related to drawing on the screen.
mod display;
// Handles game pausing and frame advancing like an emulator to easily get frame accurate button presses.
mod frame_pause;
/// Orchestrates game related tasks such as hooking, reading, and showing views to the user.
mod game;
/// Tools for pkrd to hook into games.
mod hook;
/// Tools related to reading the game, such as Pokemon.
mod reader;
/// RNG utils to help keep track of the current state.
mod rng;
/// The views shown to the user, such as Pokemon and RNG info.
mod views;
