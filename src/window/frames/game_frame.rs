//! Game frame

use crate::core::error::CResult;
use crate::emulator::Emulator;
use crate::window::draw::{DrawContext, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::window::frame::Frame;

/// Game frame
pub struct GameFrame {
    frame: Frame,
}

impl GameFrame {
    /// Create new frame
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            frame: Frame::new(rectf!(x, y, SCREEN_WIDTH, SCREEN_HEIGHT), "GAME"),
        }
    }

    /// Render
    pub fn render(&mut self, emulator: &Emulator, ctx: &mut DrawContext) -> CResult {
        // Render !
        emulator.cpu.borrow_mut().peripherals.screen.render(
            self.frame.rect.x() as u32,
            self.frame.rect.y() as u32,
            ctx.canvas,
        )?;

        self.frame.render(ctx)?;
        Ok(())
    }
}
