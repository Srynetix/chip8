//! CHIP-8 graphics module

#![warn(missing_docs)]

extern crate chip8_core;
extern crate chip8_cpu;
extern crate sdl2;

pub mod renderer;

pub use renderer::Renderer;