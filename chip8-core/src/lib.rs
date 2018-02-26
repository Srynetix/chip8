//! CHIP-8 core module

#![warn(missing_docs)]

#[macro_use]
extern crate lazy_static;
extern crate rand;

pub mod cpu;
pub mod cartridge;

pub use cpu::CPU;
pub use cartridge::Cartridge;