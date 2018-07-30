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

mod breakpoints;
mod cartridge;
mod cpu;
mod debugger;
mod font;
mod input;
mod logger;
mod memory;
mod opcodes;
mod peripherals;
mod registers;
mod savestate;
mod screen;
mod shell;
mod stack;
mod timer;
mod types;

pub use cartridge::Cartridge;
pub use cpu::CPU;
pub use shell::start_shell;
