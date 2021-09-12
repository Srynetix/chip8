//! Drivers.

use super::errors::CResult;
use crate::{
    debugger::{Debugger, DebuggerContext},
    emulator::{Emulator, EmulatorContext},
    peripherals::{cartridge::Cartridge, input::InputState, screen::Color},
};

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
    fn run_emulator(
        &mut self,
        emulator: Emulator,
        emulator_ctx: EmulatorContext,
        cartridge: Cartridge,
    ) -> CResult;

    /// Run debugger.
    fn run_debugger(
        &mut self,
        debugger: Debugger,
        debugger_ctx: DebuggerContext,
        emulator: Emulator,
        emulator_ctx: EmulatorContext,
        cartridge: Cartridge,
    ) -> CResult;

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
    #[allow(clippy::too_many_arguments)]
    fn render_pixel(
        &mut self,
        origin_x: u32,
        origin_y: u32,
        x: usize,
        y: usize,
        scale: usize,
        color: Color,
        frame_width: usize,
    ) -> CResult;
}

/// Audio interface.
pub trait AudioInterface {
    /// Play beep.
    fn play_beep(&mut self);
}

/// Drivers.
#[derive(Default)]
pub struct Drivers {
    /// Audio driver
    pub audio: Option<Box<dyn AudioInterface>>,
}

impl Drivers {
    /// Creates new drivers.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set audio driver.
    pub fn set_audio_driver(&mut self, audio_driver: Box<dyn AudioInterface>) {
        self.audio = Some(audio_driver);
    }
}
