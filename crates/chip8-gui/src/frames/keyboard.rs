//! Keyboard frame.

use std::collections::HashMap;

use chip8_core::{core::types::C8Byte, emulator::Emulator};
use macroquad::prelude::Rect;
use once_cell::sync::Lazy;

use crate::{
    draw::{ui_draw_frame_ex, ui_draw_text_ex, ui_text_size},
    frame::Frame,
};

const KEY_SIZE: u32 = 32;
const KEY_MARGIN: u32 = 8;

/// Keyboard width.
pub const KEYBOARD_WIDTH: u32 = (KEY_SIZE + KEY_MARGIN * 2) * 4;
/// Keyboard height.
pub const KEYBOARD_HEIGHT: u32 = (KEY_SIZE + KEY_MARGIN * 2) * 5;

static KEY_POSITIONS: Lazy<HashMap<C8Byte, (u32, u32)>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert(0x0, (1, 3));
    m.insert(0x1, (0, 0));
    m.insert(0x2, (1, 0));
    m.insert(0x3, (2, 0));
    m.insert(0x4, (0, 1));
    m.insert(0x5, (1, 1));
    m.insert(0x6, (2, 1));
    m.insert(0x7, (0, 2));
    m.insert(0x8, (1, 2));
    m.insert(0x9, (2, 2));
    m.insert(0xA, (0, 3));
    m.insert(0xB, (2, 3));
    m.insert(0xC, (3, 0));
    m.insert(0xD, (3, 1));
    m.insert(0xE, (3, 2));
    m.insert(0xF, (3, 3));

    m
});

/// Keyboard frame.
pub struct KeyboardFrame {
    frame: Frame,
}

impl KeyboardFrame {
    /// Create new frame.
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            frame: Frame::new(
                Rect::new(
                    x as f32,
                    y as f32,
                    KEYBOARD_WIDTH as f32,
                    KEYBOARD_HEIGHT as f32,
                ),
                "KEYBOARD",
            ),
        }
    }

    fn render_wait_indicator(&self, emulator: &Emulator) {
        let font_size = 32;
        let grey_color = macroquad::color::GRAY;
        let white_color = macroquad::color::WHITE;

        let wait_x = KEY_SIZE + KEY_MARGIN * 2;
        let wait_y = (KEY_SIZE + KEY_MARGIN * 2) * 4;
        let wait_w = (KEY_SIZE + KEY_MARGIN * 2) * 2;
        let wait_h = KEY_SIZE + KEY_MARGIN * 2;

        let locked = emulator.cpu.peripherals.input.is_locked();
        let color = if locked { white_color } else { grey_color };

        let wait_sz = ui_text_size("WAIT", font_size);

        // Render wait
        ui_draw_text_ex(
            "WAIT",
            self.frame.rect.x + wait_x as f32 + wait_w as f32 / 2. - wait_sz.width / 2.,
            self.frame.rect.y + wait_y as f32 + wait_h as f32 / 2. - wait_sz.height / 2.,
            font_size,
            color,
        );

        ui_draw_frame_ex(
            Rect::new(
                self.frame.rect.x + wait_x as f32,
                self.frame.rect.y + wait_y as f32,
                wait_w as f32,
                wait_h as f32,
            ),
            color,
        );
    }

    fn render_keyboard(&self, emulator: &Emulator) {
        let font_size = 32;
        let grey_color = macroquad::color::GRAY;
        let white_color = macroquad::color::WHITE;

        for (idx, v) in emulator.cpu.peripherals.input.get_data().iter().enumerate() {
            let color = match v {
                0 => grey_color,
                _ => white_color,
            };

            let (x, y) = KEY_POSITIONS[&(idx as C8Byte)];
            let character = format!("{:X}", idx);
            let (font_width, font_height) = {
                let sz = ui_text_size(&character, font_size);
                (sz.width, sz.height)
            };

            let c_x = x * (KEY_SIZE + KEY_MARGIN * 2) + (KEY_SIZE + KEY_MARGIN * 2) / 2
                - font_width as u32 / 2;
            let c_y = y * (KEY_SIZE + KEY_MARGIN * 2) + (KEY_SIZE + KEY_MARGIN * 2) / 2
                - font_height as u32 / 2;

            ui_draw_text_ex(
                &character,
                self.frame.rect.x + c_x as f32,
                self.frame.rect.y + c_y as f32,
                font_size,
                color,
            );

            ui_draw_frame_ex(
                Rect::new(
                    self.frame.rect.x + x as f32 * (KEY_SIZE + KEY_MARGIN * 2) as f32,
                    self.frame.rect.y + y as f32 * (KEY_SIZE + KEY_MARGIN * 2) as f32,
                    (KEY_SIZE + KEY_MARGIN * 2) as f32,
                    (KEY_SIZE + KEY_MARGIN * 2) as f32,
                ),
                color,
            );
        }
    }

    /// Render.
    pub fn render(&mut self, emulator: &Emulator) {
        // Render keyboard.
        self.render_keyboard(emulator);

        // Render wait indicator.
        self.render_wait_indicator(emulator);

        // Render.
        self.frame.render();
    }
}
