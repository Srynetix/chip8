//! Game frame.

use crate::core::error::CResult;
use crate::emulator::Emulator;
use crate::window::draw::{DrawContext, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::window::frame::Frame;

/// Game frame.
pub struct GameFrame {
    frame: Frame,
}

impl GameFrame {
    /// Create new frame.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate.
    /// * `y` - Y coordinate.
    ///
    /// # Returns
    ///
    /// * Game frame instance.
    ///
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            frame: Frame::new(rectf!(x, y, SCREEN_WIDTH, SCREEN_HEIGHT), "GAME"),
        }
    }

    /// Render.
    ///
    /// # Arguments
    ///
    /// * `emulator` - Emulator.
    /// * `ctx` - Draw context.
    ///
    /// # Returns
    ///
    /// * Result.
    ///
    pub fn render(&mut self, emulator: &mut Emulator, ctx: &mut DrawContext) -> CResult {
        emulator.cpu.peripherals.screen.render(
            self.frame.rect.x() as u32,
            self.frame.rect.y() as u32,
            ctx.canvas,
        )?;

        self.frame.render(ctx)?;
        Ok(())
    }
}
