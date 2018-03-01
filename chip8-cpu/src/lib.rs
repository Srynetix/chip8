//! CHIP-8 core module

#![warn(missing_docs)]

extern crate chip8_core;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rand;

pub mod cpu;
pub mod memory;
pub mod registers;
pub mod stack;
pub mod timer;
pub mod opcodes;
pub mod video;
pub mod font;
pub mod input;
pub mod cartridge;

pub use cpu::CPU;
pub use cartridge::Cartridge;