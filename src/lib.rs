//! CHIP-8 emulator

#![warn(missing_docs)]

extern crate chip8_core;
extern crate chip8_graphics;

pub mod device;

pub use device::{Device, DeviceBuilder};
pub use chip8_core::Cartridge;