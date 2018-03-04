//! CHIP-8 device modules

#![warn(missing_docs)]

extern crate chip8_core;
extern crate chip8_cpu;

extern crate sdl2;
extern crate time;
extern crate clap;
#[macro_use]
extern crate log;
extern crate fern;
extern crate chrono;
extern crate crossbeam;

pub mod shell;
pub mod logger;

pub use shell::start_shell;