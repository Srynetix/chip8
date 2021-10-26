//! Memory frame.

use chip8_core::{emulator::Emulator, peripherals::memory::CHUNK_SIZE};
use macroquad::prelude::Rect;

use crate::{
    draw::{ui_draw_fill_rect, ui_draw_text},
    frame::Frame,
};

/// Memory frame.
pub struct MemoryFrame {
    frame: Frame,
}

impl MemoryFrame {
    /// Create new frame.
    pub fn new(rect: Rect) -> Self {
        Self {
            frame: Frame::new(rect, "MMRY"),
        }
    }

    /// Render.
    pub fn render(&self, emulator: &Emulator) {
        let font_size = 6;
        let mut output = String::new();

        // Draw background.
        ui_draw_fill_rect(self.frame.rect, macroquad::color::BLACK);

        for (idx, chunk) in emulator
            .cpu
            .peripherals
            .memory
            .get_data()
            .chunks(CHUNK_SIZE)
            .enumerate()
        {
            output.push_str(&format!(
                "{:04X}-{:04X}|",
                idx * CHUNK_SIZE,
                (idx + 1) * CHUNK_SIZE
            ));

            for chunk_value in chunk.iter() {
                output.push_str(&format!("{:02X}", chunk_value));
            }

            output.push('\n');
        }

        output.push_str(&format!(
            "PC: {:04X}",
            emulator.cpu.peripherals.memory.get_pointer()
        ));

        ui_draw_text(
            &output,
            self.frame.rect.x + 4.,
            self.frame.rect.y + font_size as f32 + 4.,
            font_size,
        );

        self.frame.render();
    }
}
