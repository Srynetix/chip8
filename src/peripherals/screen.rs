//! CHIP-8 video memory.

use std::fmt;

use sdl2;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use crate::core::error::CResult;
use crate::core::font::FONT_CHAR_WIDTH;
use crate::core::types::C8Byte;

/// Video memory width.
pub const VIDEO_MEMORY_WIDTH: usize = 64;
/// Video memory height.
pub const VIDEO_MEMORY_HEIGHT: usize = 32;
/// Renderer scale.
pub const RENDERER_SCALE: usize = 10;

const PIXEL_FADE_COEFFICIENT: f32 = 0.8;
const VIDEO_MEMORY_SIZE: usize = VIDEO_MEMORY_WIDTH * VIDEO_MEMORY_HEIGHT;

/// Screen mode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScreenMode {
    /// Standard screen.
    Standard,
    /// Extended screen.
    Extended,
}

/// Screen scroll direction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScreenScrollDirection {
    /// Down.
    Down,
    /// Right.
    Right,
    /// Left.
    Left,
    /// Disabled.
    Disabled
}

/// Screen scroll.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenScroll {
    /// Scrolling.
    pub scrolling: bool,
    /// Lines.
    pub lines: C8Byte,
    /// Direction.
    pub direction: ScreenScrollDirection
}

/// Screen data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenData {
    data: Vec<C8Byte>,
    alpha: Vec<C8Byte>,
    mode: ScreenMode,
    pub scroll: ScreenScroll,
}

/// Screen memory struct.
pub struct Screen {
    /// Screen data.
    pub data: ScreenData,
}

impl Default for Screen {
    fn default() -> Self {
        let data = vec![0; VIDEO_MEMORY_SIZE];
        let alpha = vec![0; VIDEO_MEMORY_SIZE];

        Screen {
            data: ScreenData {
                data,
                alpha,
                mode: ScreenMode::Standard,
                scroll: ScreenScroll {
                    scrolling: false,
                    lines: 0,
                    direction: ScreenScrollDirection::Disabled,
                }
            },
        }
    }
}

impl Screen {
    /// Create new screen.
    ///
    /// # Returns
    ///
    /// * Screen instance.
    ///
    pub fn new() -> Self {
        Default::default()
    }

    /// Reload screen for mode.
    ///
    /// # Arguments
    ///
    /// * `mode` - Screen mode.
    ///
    pub fn reload_screen_for_mode(&mut self, mode: ScreenMode) {
        self.data.mode = mode;

        let coef = self.get_screen_size_coef();
        self.data.data = vec![0; VIDEO_MEMORY_SIZE * coef * coef];
        self.data.alpha = vec![0; VIDEO_MEMORY_SIZE * coef * coef];
    }

    /// Get screen size coef.
    ///
    /// # Returns
    ///
    /// * Screen size coef.
    ///
    fn get_screen_size_coef(&self) -> usize {
        match self.data.mode {
            ScreenMode::Standard => 1,
            ScreenMode::Extended => 2,
        }
    }

    /// Check if screen is scrolling.
    ///
    /// # Returns
    ///
    /// * `true` if scrolling.
    /// * `false` if not.
    ///
    pub fn is_scrolling(&self) -> bool {
        self.data.scroll.scrolling
    }

    /// Draw sprite.
    ///
    /// # Arguments
    ///
    /// * `r1` - X position.
    /// * `r2` - Y position.
    /// * `sprite` - Sprite to draw.
    ///
    /// # Returns
    ///
    /// `true` if collision.
    /// `false` if not.
    ///
    pub fn draw_sprite(&mut self, r1: C8Byte, r2: C8Byte, sprite: &[C8Byte]) -> bool {
        let coef = self.get_screen_size_coef();

        let byte = sprite.len();
        let mut collision = false;

        for (i, code) in sprite.iter().enumerate().take(byte) {
            let y = ((r2 as usize) + (i as usize)) % (VIDEO_MEMORY_HEIGHT * coef);
            let mut shift = FONT_CHAR_WIDTH - 1;

            for j in 0..FONT_CHAR_WIDTH {
                let x = ((r1 as usize) + (j as usize)) % (VIDEO_MEMORY_WIDTH * coef);

                if code & (0x1 << shift) != 0 && self.toggle_pixel_xy(x, y) {
                    collision = true;
                }

                if shift > 0 {
                    shift -= 1;
                }
            }
        }

        collision
    }

    /// Clear screen.
    pub fn clear_screen(&mut self) {
        for x in 0..self.data.data.len() {
            self.data.data[x] = 0
        }
    }

    /// Fade pixels.
    pub fn fade_pixels(&mut self) {
        for x in 0..self.data.data.len() {
            if self.data.data[x] == 0 && self.data.alpha[x] > 0 {
                self.data.alpha[x] = (f32::from(self.data.alpha[x]) * PIXEL_FADE_COEFFICIENT) as u8;
            }
        }
    }

    /// Toggle pixel position.
    ///
    /// # Arguments
    ///
    /// * `pos` - Position.
    ///
    /// # Returns
    ///
    /// * `true` if collision.
    /// * `false` if not.
    ///
    pub fn toggle_pixel(&mut self, pos: usize) -> bool {
        // For now, only handle 0 and 1.
        let mut flip = false;
        let pixel = self.data.data[pos];

        if pixel == 1 {
            self.data.data[pos] = 0;
            self.data.alpha[pos] = 255;
            flip = true;
        } else {
            self.data.data[pos] = 1;
            self.data.alpha[pos] = 255;
        }

        flip
    }

    /// Render screen.
    ///
    /// # Arguments
    ///
    /// * `origin_x` - X origin.
    /// * `origin_y` - Y origin.
    /// * `renderer` - Renderer.
    ///
    /// # Returns
    ///
    /// * Result.
    ///
    pub fn render(&mut self, origin_x: u32, origin_y: u32, renderer: &mut WindowCanvas) -> CResult {
        // Render to surface.
        for (pos, px) in self.data.data.iter().enumerate() {
            let x = pos % VIDEO_MEMORY_WIDTH;
            let y = pos / VIDEO_MEMORY_WIDTH;

            let alpha = &self.data.alpha[pos];

            let color = color_from_byte(*px, *alpha);
            renderer.set_draw_color(color);

            renderer.fill_rect(rectf!(
                origin_x as i32 + (x * RENDERER_SCALE) as i32,
                origin_y as i32 + (y * RENDERER_SCALE) as i32,
                RENDERER_SCALE as u32,
                RENDERER_SCALE as u32
            ))?;
        }

        self.fade_pixels();

        Ok(())
    }

    /// Toggle pixel w/ X/Y coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate.
    /// * `y` - Y coordinate.
    ///
    /// # Returns
    ///
    /// * `true` if collision.
    /// * `false` if not.
    ///
    pub fn toggle_pixel_xy(&mut self, x: usize, y: usize) -> bool {
        let coef = self.get_screen_size_coef();
        self.toggle_pixel(x + y * (VIDEO_MEMORY_WIDTH * coef))
    }

    /// Dump screen.
    pub fn dump_screen(&self) {
        println!("{:?}", &self);
    }

    /// Reset screen.
    pub fn reset(&mut self) {
        self.data.data = vec![0; VIDEO_MEMORY_SIZE];
        self.data.alpha = vec![255; VIDEO_MEMORY_SIZE];
        self.data.mode = ScreenMode::Standard;
    }

    /// Load from save.
    ///
    /// # Arguments
    ///
    /// * `screen_data` - Screen data.
    ///
    pub fn load_from_save(&mut self, screen_data: ScreenData) {
        self.data.data = screen_data.data;
        self.data.alpha = screen_data.alpha;
        self.data.mode = screen_data.mode;
    }
}

impl fmt::Debug for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let coef = self.get_screen_size_coef();
        writeln!(
            f,
            "    -> size: {} x {}",
            VIDEO_MEMORY_WIDTH * coef,
            VIDEO_MEMORY_HEIGHT * coef
        )?;

        for j in 0..(VIDEO_MEMORY_HEIGHT * coef) {
            write!(f, "    ")?;

            for i in 0..(VIDEO_MEMORY_WIDTH * coef) {
                let pixel = self.data.data[i + j * (VIDEO_MEMORY_WIDTH * coef)];
                if pixel == 0 {
                    write!(f, " ")?;
                } else {
                    write!(f, "█")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn color_from_byte(byte: C8Byte, alpha: C8Byte) -> Color {
    match byte {
        0 => Color::RGB(alpha, alpha, alpha),
        _ => Color::RGB(255, 255, 255),
    }
}
