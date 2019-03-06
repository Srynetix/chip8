//! CHIP-8 core module

#![warn(missing_docs)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

#[macro_use]
pub mod core;
pub mod debugger;
pub mod emulator;
pub mod peripherals;
pub mod shell;
pub mod window;

pub use crate::emulator::Emulator;
pub use crate::shell::start_shell;
