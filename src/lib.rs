//! CHIP-8.

#![warn(missing_docs)]

pub mod core;
pub mod debugger;
pub mod emulator;
pub mod peripherals;
pub mod shell;
pub mod window;

pub use crate::emulator::Emulator;
pub use crate::shell::start_shell;
pub use crate::shell::start_shell_using_args;
