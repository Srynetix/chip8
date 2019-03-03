//! Draw utils

use std::error::Error;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, TextureQuery, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

use super::font::FontHandler;

macro_rules! rectf(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

/// Screen width
pub const SCREEN_WIDTH: u32 = 640;
/// Screen height
pub const SCREEN_HEIGHT: u32 = 480;
/// Window width
pub const WINDOW_WIDTH: u32 = 960;
/// Window height
pub const WINDOW_HEIGHT: u32 = 720;

/// Draw context
pub struct DrawContext<'ttf, 'a, 'b> {
    /// Font handler
    pub font_handler: &'b mut FontHandler<'ttf, 'a>,
    /// Canvas
    pub canvas: &'b mut WindowCanvas,
    /// Texture creator
    pub texture_creator: &'b TextureCreator<WindowContext>,
}

/// Clear screen
pub fn clear_screen(canvas: &mut sdl2::render::WindowCanvas) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
}

/// Draw frame
pub fn draw_frame(
    canvas: &mut sdl2::render::WindowCanvas,
    rect: Rect,
) -> Result<(), Box<dyn Error>> {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.draw_rect(rect)?;

    Ok(())
}

/// Draw text
pub fn draw_text(
    canvas: &mut sdl2::render::WindowCanvas,
    texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    font: &Font,
    text: &str,
    x: u32,
    y: u32,
) -> Result<(), Box<dyn Error>> {
    let surface = font.render(text).blended(Color::RGB(255, 255, 255))?;
    let texture = texture_creator.create_texture_from_surface(&surface)?;

    let TextureQuery { width, height, .. } = texture.query();
    let target = rectf!(x, y, width, height);

    canvas.copy(&texture, None, Some(target))?;

    Ok(())
}
