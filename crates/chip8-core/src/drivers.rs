//! Drivers.

use crate::{emulator::Emulator, debugger::{Debugger, DebuggerContext}, emulator::EmulatorContext, peripherals::{cartridge::Cartridge, input::InputState, screen::Color}};
use super::errors::CResult;

/// Screen width.
pub const SCREEN_WIDTH: u32 = 640;
/// Screen height.
pub const SCREEN_HEIGHT: u32 = 320;
/// Window width.
pub const WINDOW_WIDTH: u32 = 960;
/// Window height.
pub const WINDOW_HEIGHT: u32 = 720;
/// Window title.
pub const WINDOW_TITLE: &str = "CHIP-8 Emulator GUI";

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
