//! Home scene

use std::error::Error;

use sdl2::keyboard::{Keycode, Scancode};
use sdl2::EventPump;

use crate::window::draw::{
    clear_screen, draw_frame, draw_text, DrawContext, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use crate::window::scene::Scene;
use crate::window::scenemanager::SceneContext;

/// Home scene
pub struct HomeScene {}

impl Default for HomeScene {
    fn default() -> Self {
        Self {}
    }
}

impl HomeScene {
    /// Create new scene
    pub fn new() -> Self {
        Default::default()
    }
}

impl Scene for HomeScene {
    fn init(&mut self) {}
    fn destroy(&mut self) {}

    fn render(&mut self, ctx: &mut DrawContext) -> Result<(), Box<dyn Error>> {
        clear_screen(ctx.canvas);

        {
            // Draw logo
            let font = ctx.font_handler.get_or_create_font("default", 64)?;
            let txt = "CHIP-8";
            let sz = font.size_of(txt).unwrap();

            let x_pos = WINDOW_WIDTH / 2 - sz.0 / 2;
            let y_pos = WINDOW_HEIGHT / 4 - sz.1 / 2;
            let pad = 40;

            draw_frame(
                ctx.canvas,
                rectf!(x_pos - pad, y_pos - pad, sz.0 + pad * 2, sz.1 + pad * 2),
            )?;
            draw_text(
                ctx.canvas,
                ctx.texture_creator,
                font,
                "CHIP-8",
                x_pos,
                y_pos,
            )?;
        }

        {
            // Draw instructions
            let font = ctx.font_handler.get_or_create_font("default", 32)?;
            let txt1 = "F2 - LOAD GAME";
            let txt2 = "F10 - QUIT";
            let sz1 = font.size_of(txt1).unwrap();
            let sz2 = font.size_of(txt2).unwrap();

            let x_pos1 = WINDOW_WIDTH / 2 - sz1.0 / 2;
            let y_pos1 = WINDOW_HEIGHT - WINDOW_HEIGHT / 4 - sz1.1 / 2;
            let x_pos2 = WINDOW_WIDTH / 2 - sz2.0 / 2;
            let y_pos2 = y_pos1 + sz2.1 * 2;

            draw_text(ctx.canvas, ctx.texture_creator, font, txt1, x_pos1, y_pos1)?;
            draw_text(ctx.canvas, ctx.texture_creator, font, txt2, x_pos2, y_pos2)?;
        }

        Ok(())
    }

    fn input(&mut self, ctx: &mut SceneContext, event_pump: &mut EventPump) {
        let f2 = Scancode::from_keycode(Keycode::F2).unwrap();
        let f10 = Scancode::from_keycode(Keycode::F10).unwrap();

        if event_pump.keyboard_state().is_scancode_pressed(f2) {
            ctx.set_current_scene("debug");
        } else if event_pump.keyboard_state().is_scancode_pressed(f10) {
            ctx.quit();
        }
    }
}
