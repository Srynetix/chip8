//! CHIP-8 core module

#![warn(missing_docs)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

extern crate rand;
extern crate sdl2;
extern crate time;
extern crate clap;
extern crate fern;
extern crate chrono;
extern crate bincode;

#[macro_use]
mod macros;

mod cpu;
mod memory;
mod registers;
mod stack;
mod timer;
mod opcodes;
mod screen;
mod font;
mod input;
mod cartridge;
mod peripherals;
mod breakpoints;
mod debugger;
mod shell;
mod logger;
mod types;
mod savestate;

pub use cpu::CPU;
pub use cartridge::Cartridge;
pub use shell::start_shell;