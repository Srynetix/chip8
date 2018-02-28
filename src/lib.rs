//! CHIP-8 emulator

#![warn(missing_docs)]

extern crate chip8_core;
extern crate chip8_cpu;
extern crate chip8_graphics;

extern crate time;
extern crate clap;
#[macro_use]
extern crate log;
extern crate fern;
extern crate chrono;

pub mod device;
pub mod shell;
pub mod logger;

pub use device::{Device};
pub use shell::start_shell;