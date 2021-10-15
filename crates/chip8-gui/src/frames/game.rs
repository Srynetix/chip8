//! Game frame.

use chip8_core::{
    drivers::{SCREEN_HEIGHT, SCREEN_WIDTH},
    emulator::Emulator,
};
use chip8_drivers::MQRenderDriver;
use macroquad::prelude::{draw_texture, Rect};

use crate::frame::Frame;

/// Game frame.
pub struct GameFrame {
    frame: Frame,
    driver: MQRenderDriver,
}

impl GameFrame {
    /// Create new frame.
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            frame: Frame::new(
                Rect::new(
                    x as f32,
                    y as f32,
                    SCREEN_WIDTH as f32,
                    SCREEN_HEIGHT as f32,
                ),
                "GAME",
            ),
            driver: MQRenderDriver::new(),
        }
    }

    /// Render.
    pub fn render(&mut self, emulator: &mut Emulator) {
        emulator
            .cpu
            .peripherals
            .screen
            .render_pixels(0, 0, 0, &mut self.driver)
            .unwrap();
        self.driver.texture.update(&self.driver.image);
        draw_texture(
            self.driver.texture,
            self.frame.rect.x,
            self.frame.rect.y,
            macroquad::color::WHITE,
        );

        self.frame.render();
    }
}
