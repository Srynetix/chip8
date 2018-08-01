//! CHIP-8 core module

#![warn(missing_docs)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

extern crate bincode;
extern crate chrono;
extern crate clap;
extern crate fern;
extern crate rand;
extern crate rustyline;
extern crate sdl2;
extern crate time;

#[macro_use]
mod macros;

pub mod breakpoints;
pub mod cartridge;
pub mod cpu;
pub mod debugger;
pub mod emulator;
pub mod font;
pub mod input;
pub mod logger;
pub mod memory;
pub mod opcodes;
pub mod peripherals;
pub mod registers;
pub mod savestate;
pub mod screen;
pub mod shell;
pub mod stack;
pub mod timer;
pub mod types;

pub use shell::start_shell;
