//! Debug scene

use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::EventPump;

use crate::cartridge::Cartridge;
use crate::error::CResult;
use crate::window::draw::{
    clear_screen, draw_text, DrawContext, SCREEN_HEIGHT, SCREEN_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use crate::window::frame::Frame;
use crate::window::scene::Scene;
use crate::window::scenemanager::SceneContext;

/// Debug scene
pub struct DebugScene {
    game_frame: GameFrame,
    info_frame: InfoFrame,
    console_frame: ConsoleFrame,
}

impl Default for DebugScene {
    fn default() -> Self {
        Self {
            game_frame: GameFrame::new(rectf!(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT)),
            info_frame: InfoFrame::new(rectf!(
                SCREEN_WIDTH,
                0,
                WINDOW_WIDTH - SCREEN_WIDTH,
                SCREEN_HEIGHT
            )),
            console_frame: ConsoleFrame::new(rectf!(
                0,
                SCREEN_HEIGHT,
                WINDOW_WIDTH,
                WINDOW_HEIGHT - SCREEN_HEIGHT
            )),
        }
    }
}

impl DebugScene {
    /// Create new scene
    pub fn new() -> Self {
        Default::default()
    }

    /// Load cartridge dump
    pub fn load_cartridge_dump(&mut self, cartridge: &Cartridge) {
        let (_code, assembly, verbose) = cartridge.disassemble();
        let mut ptr_value = 0x200;
        for i in 0..assembly.len() {
            let line = format!(
                "{:04X}| {:3} {:20} ; {}",
                ptr_value, "", assembly[i], verbose[i]
            );
            self.console_frame.print_text(&line);
            ptr_value += 2;
        }
    }
}

impl Scene for DebugScene {
    fn init(&mut self, _ctx: &mut SceneContext) {}
    fn destroy(&mut self, _ctx: &mut SceneContext) {}

    fn render(&mut self, ctx: &mut DrawContext) -> CResult {
        clear_screen(ctx.canvas);

        self.game_frame.render(ctx)?;
        self.info_frame.render(ctx)?;
        self.console_frame.render(ctx)?;

        Ok(())
    }

    fn keydown(&mut self, _ctx: &mut SceneContext, _kc: Keycode) {}
    fn keyup(&mut self, _ctx: &mut SceneContext, _kc: Keycode) {}
    fn input(&mut self, _ctx: &mut SceneContext, _event_pump: &mut EventPump) {}
}

struct GameFrame {
    frame: Frame,
}

impl GameFrame {
    pub fn new(rect: Rect) -> Self {
        Self {
            frame: Frame::new(rect, "GAME"),
        }
    }

    pub fn render(&self, ctx: &mut DrawContext) -> CResult {
        self.frame.render(ctx)?;
        Ok(())
    }
}

struct InfoFrame {
    frame: Frame,
}

impl InfoFrame {
    pub fn new(rect: Rect) -> Self {
        Self {
            frame: Frame::new(rect, "INFO"),
        }
    }

    pub fn render(&self, ctx: &mut DrawContext) -> CResult {
        self.frame.render(ctx)?;
        Ok(())
    }
}

struct ConsoleFrame {
    frame: Frame,
    buffer: Vec<String>,
}

impl ConsoleFrame {
    pub fn new(rect: Rect) -> Self {
        Self {
            frame: Frame::new(rect, "DEBUG"),
            buffer: vec![],
        }
    }

    pub fn print_text(&mut self, text: &str) {
        self.buffer.push(String::from(text))
    }

    pub fn render(&self, ctx: &mut DrawContext) -> CResult {
        let font = ctx.font_handler.get_font("default", 8).unwrap();
        let mut cursor_y = self.frame.rect.y() + 4;
        let char_height = font.height() + 4;

        for b in self.buffer.iter() {
            draw_text(
                ctx.canvas,
                ctx.texture_creator,
                font,
                b,
                (self.frame.rect.x() + 4) as u32,
                cursor_y as u32,
            )?;
            cursor_y += char_height;
        }

        self.frame.render(ctx)?;

        Ok(())
    }
}
