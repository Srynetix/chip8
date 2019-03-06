//! Keyboard frame

use std::collections::HashMap;

use sdl2::pixels::Color;

use crate::core::error::CResult;
use crate::core::types::C8Byte;
use crate::emulator::Emulator;
use crate::window::draw::{draw_text_ex, DrawContext};
use crate::window::frame::Frame;

const KEY_SIZE: u32 = 32;
const KEY_MARGIN: u32 = 8;

/// Keyboard width
pub const KEYBOARD_WIDTH: u32 = (KEY_SIZE + KEY_MARGIN * 2) * 4;
/// Keyboard height
pub const KEYBOARD_HEIGHT: u32 = (KEY_SIZE + KEY_MARGIN * 2) * 5;

lazy_static! {
    static ref KEY_POSITIONS: HashMap<C8Byte, (u32, u32)> = {
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
    };
}

/// Keyboard frame
pub struct KeyboardFrame {
    frame: Frame,
}

impl KeyboardFrame {
    /// Create new frame
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            frame: Frame::new(rectf!(x, y, KEYBOARD_WIDTH, KEYBOARD_HEIGHT), "KEYBOARD"),
        }
    }

    fn render_wait_indicator(&self, emulator: &Emulator, ctx: &mut DrawContext) -> CResult {
        let font = ctx.font_handler.get_or_create_font("default", 16).unwrap();

        let grey_color = Color::RGB(127, 127, 127);
        let white_color = Color::RGB(255, 255, 255);

        let wait_x = KEY_SIZE + KEY_MARGIN * 2;
        let wait_y = (KEY_SIZE + KEY_MARGIN * 2) * 4;
        let wait_w = (KEY_SIZE + KEY_MARGIN * 2) * 2;
        let wait_h = KEY_SIZE + KEY_MARGIN * 2;

        let locked = emulator
            .cpu
            .borrow()
            .peripherals
            .input
            .data
            .lock
            .is_locked();
        let color = if locked { white_color } else { grey_color };

        let wait_sz = font.size_of("WAIT").unwrap();

        // Render wait
        draw_text_ex(
            ctx.canvas,
            ctx.texture_creator,
            font,
            "WAIT",
            self.frame.rect.x() as u32 + wait_x + wait_w / 2 - wait_sz.0 / 2,
            self.frame.rect.y() as u32 + wait_y + wait_h / 2 - wait_sz.1 / 2,
            color,
        )?;

        let old_color = ctx.canvas.draw_color();
        ctx.canvas.set_draw_color(color);
        ctx.canvas.draw_rect(rectf!(
            self.frame.rect.x() as u32 + wait_x,
            self.frame.rect.y() as u32 + wait_y,
            wait_w,
            wait_h
        ))?;
        ctx.canvas.set_draw_color(old_color);

        Ok(())
    }

    fn render_keyboard(&self, emulator: &Emulator, ctx: &mut DrawContext) -> CResult {
        let font = ctx.font_handler.get_or_create_font("default", 16).unwrap();

        let grey_color = Color::RGB(127, 127, 127);
        let white_color = Color::RGB(255, 255, 255);

        for (idx, v) in emulator
            .cpu
            .borrow()
            .peripherals
            .input
            .get_data()
            .iter()
            .enumerate()
        {
            let color = match v {
                0 => grey_color,
                _ => white_color,
            };

            let (x, y) = KEY_POSITIONS[&(idx as C8Byte)];
            let character = format!("{:X}", idx);
            let (font_width, font_height) = font.size_of(&character).unwrap();

            let c_x =
                x * (KEY_SIZE + KEY_MARGIN * 2) + (KEY_SIZE + KEY_MARGIN * 2) / 2 - font_width / 2;
            let c_y =
                y * (KEY_SIZE + KEY_MARGIN * 2) + (KEY_SIZE + KEY_MARGIN * 2) / 2 - font_height / 2;

            draw_text_ex(
                ctx.canvas,
                ctx.texture_creator,
                font,
                &character,
                self.frame.rect.x() as u32 + c_x,
                self.frame.rect.y() as u32 + c_y,
                color,
            )?;

            let old_color = ctx.canvas.draw_color();
            ctx.canvas.set_draw_color(color);
            ctx.canvas.draw_rect(rectf!(
                self.frame.rect.x() as u32 + x * (KEY_SIZE + KEY_MARGIN * 2),
                self.frame.rect.y() as u32 + y * (KEY_SIZE + KEY_MARGIN * 2),
                KEY_SIZE + KEY_MARGIN * 2,
                KEY_SIZE + KEY_MARGIN * 2
            ))?;
            ctx.canvas.set_draw_color(old_color);
        }

        Ok(())
    }

    /// Render
    pub fn render(&mut self, emulator: &Emulator, ctx: &mut DrawContext) -> CResult {
        // Render keyboard
        self.render_keyboard(emulator, ctx)?;

        // Render wait indicator
        self.render_wait_indicator(emulator, ctx)?;

        // Render !
        self.frame.render(ctx)
    }
}
