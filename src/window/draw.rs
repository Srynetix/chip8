//! Draw utils.

use sdl2::pixels::Color;
use sdl2::render::{TextureCreator, TextureQuery, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use sdl2::VideoSubsystem;

pub use sdl2::rect::Rect;

use crate::core::error::CResult;

use super::font::FontHandler;

/// Screen width.
pub const SCREEN_WIDTH: u32 = 640;
/// Screen height.
pub const SCREEN_HEIGHT: u32 = 320;
/// Window width.
pub const WINDOW_WIDTH: u32 = 960;
/// Window height.
pub const WINDOW_HEIGHT: u32 = 720;

/// Draw context.
pub struct DrawContext<'ttf, 'a, 'b> {
    /// Font handler.
    pub font_handler: &'b mut FontHandler<'ttf, 'a>,
    /// Canvas.
    pub canvas: &'b mut WindowCanvas,
    /// Texture creator.
    pub texture_creator: &'b TextureCreator<WindowContext>,
    /// Video subsystem.
    pub video_subsystem: &'b VideoSubsystem,
}

/// Clear screen.
///
/// # Arguments
///
/// * `canvas` - Canvas.
///
pub fn clear_screen(canvas: &mut sdl2::render::WindowCanvas) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
}

/// Draw frame.
///
/// # Arguments
///
/// * `canvas` - Canvas.
/// * `rect` - Rect.
///
/// # Returns
///
/// * Result.
///
pub fn draw_frame(canvas: &mut sdl2::render::WindowCanvas, rect: Rect) -> CResult {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.draw_rect(rect)?;

    Ok(())
}

/// Draw text.
///
/// # Arguments
///
/// * `canvas` - Canvas.
/// * `texture_creator` - Texture creator.
/// * `font` - Font.
/// * `text` - Text.
/// * `x` - X coordinate.
/// * `y` - Y coordinate.
///
/// # Returns
///
/// * Result.
///
pub fn draw_text(
    canvas: &mut sdl2::render::WindowCanvas,
    texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    font: &Font,
    text: &str,
    x: u32,
    y: u32,
) -> CResult {
    draw_text_ex(
        canvas,
        texture_creator,
        font,
        text,
        x,
        y,
        Color::RGB(255, 255, 255),
    )
}

/// Draw text ex.
///
/// # Arguments
///
/// * `canvas` - Canvas.
/// * `texture_creator` - Texture creator.
/// * `font` - Font.
/// * `text` - Text.
/// * `x` - X coordinate.
/// * `y` - Y coordinate.
///
/// # Returns
///
/// * Result.
///
pub fn draw_text_ex(
    canvas: &mut sdl2::render::WindowCanvas,
    texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    font: &Font,
    text: &str,
    x: u32,
    y: u32,
    color: Color,
) -> CResult {
    let split_text = text.split('\n');
    let cur_x = x;
    let mut cur_y = y;
    let font_height = font.height() as u32 + 4 as u32;

    for text in split_text {
        if !text.is_empty() {
            let surface = font.render(text).blended(color)?;
            let texture = texture_creator.create_texture_from_surface(&surface)?;

            let TextureQuery { width, height, .. } = texture.query();
            let target = rectf!(cur_x, cur_y, width, height);

            canvas.copy(&texture, None, Some(target))?;
        }

        cur_y += font_height;
    }

    Ok(())
}
