//! Drivers.

mod winit_driver;
mod pixels_driver;

use crate::{Emulator, core::error::CResult, debugger::{Debugger, DebuggerContext}, emulator::EmulatorContext, peripherals::{cartridge::Cartridge, input::InputState, screen::Color}};

/// Window interface.
pub trait WindowInterface {
    /// Run emulator.
    fn run_emulator(&mut self, emulator: Emulator, emulator_ctx: EmulatorContext, cartridge: Cartridge) -> CResult;

    /// Run debugger.
    fn run_debugger(&mut self, debugger: Debugger, debugger_ctx: DebuggerContext, emulator: Emulator, emulator_ctx: EmulatorContext, cartridge: Cartridge) -> CResult;

    /// Run GUI
    fn run_gui(&mut self) -> CResult;
}

/// Input interface.
pub trait InputInterface {
    /// Update input state.
    fn update_input_state(&mut self, state: &mut InputState);
}

/// Render interface.
pub trait RenderInterface {
    /// Render pixel.
    fn render_pixel(&mut self, origin_x: u32, origin_y: u32, x: usize, y: usize, scale: usize, color: Color, frame_width: usize) -> CResult;
}

pub use winit_driver::{WinitWindowDriver, WinitInputDriver};
pub use pixels_driver::PixelsRenderDriver;
