//! CHIP-8 video memory
//!
//! 1: On
//! 0: Off

use std::fmt;

use super::font::FONT_CHAR_WIDTH;
use super::types::C8Byte;

use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

/// Video memory width
pub const VIDEO_MEMORY_WIDTH: usize = 64;
/// Video memory height
pub const VIDEO_MEMORY_HEIGHT: usize = 32;
/// Renderer scale
pub const RENDERER_SCALE: usize = 10;

const PIXEL_FADE_COEFFICIENT: f32 = 0.9;
const VIDEO_MEMORY_SIZE: usize = VIDEO_MEMORY_WIDTH * VIDEO_MEMORY_HEIGHT;

const WINDOW_TITLE: &str = "CHIP-8 Emulator";
const SCHIP_WINDOW_TITLE: &str = "Super CHIP-8 Emulator";

/// Screen mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScreenMode {
    Standard,
    Extended,
}

/// CHIP-8 screen data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenData {
    data: Vec<C8Byte>,
    alpha: Vec<C8Byte>,
    mode: ScreenMode,
}

/// CHIP-8 screen memory struct
pub struct Screen {
    pub data: ScreenData,
    renderer: sdl2::render::WindowCanvas,
}

impl Screen {
    /// Create new video memory
    ///
    /// # Arguments
    ///
    /// * `context` - SDL2 context
    ///
    pub fn new(context: &sdl2::Sdl) -> Self {
        let data = vec![0; VIDEO_MEMORY_SIZE];
        let alpha = vec![0; VIDEO_MEMORY_SIZE];

        let video_subsystem = context.video().unwrap();

        let window = video_subsystem
            .window(
                WINDOW_TITLE,
                (VIDEO_MEMORY_WIDTH * RENDERER_SCALE) as u32,
                (VIDEO_MEMORY_HEIGHT * RENDERER_SCALE) as u32,
            ).position_centered()
            .build()
            .unwrap();

        let mut renderer = window.into_canvas().build().unwrap();

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();

        Screen {
            renderer,
            data: ScreenData {
                data,
                alpha,
                mode: ScreenMode::Standard,
            },
        }
    }

    /// Reload screen for mode
    ///
    /// # Arguments
    ///
    /// * `mode` - Screen mode
    ///
    pub fn reload_screen_for_mode(&mut self, mode: ScreenMode) {
        self.data.mode = mode;

        let coef = self.get_screen_size_coef();
        self.data.data = vec![0; VIDEO_MEMORY_SIZE * coef * coef];
        self.data.alpha = vec![0; VIDEO_MEMORY_SIZE * coef * coef];

        match self.data.mode {
            ScreenMode::Standard => self.set_title(WINDOW_TITLE),
            ScreenMode::Extended => self.set_title(SCHIP_WINDOW_TITLE),
        }
    }

    /// Get screen size coef
    fn get_screen_size_coef(&self) -> usize {
        match self.data.mode {
            ScreenMode::Standard => 1,
            ScreenMode::Extended => 2,
        }
    }

    /// Draw sprite
    ///
    /// # Arguments
    ///
    /// * `r1` - X position
    /// * `r2` - Y position
    /// * `sprite` - Sprite to draw
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

        self.render();

        collision
    }

    /// Clear screen
    pub fn clear_screen(&mut self) {
        for x in 0..self.data.data.len() {
            self.data.data[x] = 0
        }

        self.render();
    }

    /// Fade pixels
    pub fn fade_pixels(&mut self) {
        for x in 0..self.data.data.len() {
            if self.data.data[x] == 0 && self.data.alpha[x] > 0 {
                self.data.alpha[x] = (f32::from(self.data.alpha[x]) * PIXEL_FADE_COEFFICIENT) as u8;
            }
        }
    }

    /// Set title
    fn set_title(&mut self, title: &str) {
        self.renderer.window_mut().set_title(title).unwrap();
    }

    /// Toggle pixel position
    /// Return true if collision
    ///
    /// # Arguments
    ///
    /// * `pos` - Position
    ///
    pub fn toggle_pixel(&mut self, pos: usize) -> bool {
        // For now, only handle 0 and 1
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

    /// Render screen
    pub fn render(&mut self) {
        let coef = self.get_screen_size_coef();
        self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.renderer.clear();

        for (pos, px) in self.data.data.iter().enumerate() {
            let x = pos % (VIDEO_MEMORY_WIDTH * coef);
            let y = pos / (VIDEO_MEMORY_WIDTH * coef);
            let alpha = &self.data.alpha[pos];

            let color = color_from_byte(*px, *alpha);
            self.renderer.set_draw_color(color);
            self.renderer
                .fill_rect(Rect::new(
                    (x * (RENDERER_SCALE / coef)) as i32,
                    (y * (RENDERER_SCALE / coef)) as i32,
                    (RENDERER_SCALE / coef) as u32,
                    (RENDERER_SCALE / coef) as u32,
                )).expect("Error while drawing.");
        }

        self.renderer.present();
    }

    /// Toggle pixel w/ X/Y coordinates
    /// Return true if collision
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate
    /// * `y` - Y coordinate
    ///
    pub fn toggle_pixel_xy(&mut self, x: usize, y: usize) -> bool {
        let coef = self.get_screen_size_coef();
        self.toggle_pixel(x + y * (VIDEO_MEMORY_WIDTH * coef))
    }

    /// Dump screen
    pub fn dump_screen(&self) {
        println!("{:?}", &self);
    }

    /// Reset screen
    ///
    /// With a little flash !
    pub fn reset(&mut self) {
        self.data.data = vec![0; VIDEO_MEMORY_SIZE];
        self.data.alpha = vec![255; VIDEO_MEMORY_SIZE];
        self.data.mode = ScreenMode::Standard;
    }

    /// Load from save
    ///
    /// # Arguments
    ///
    /// * `screen_data` - Screen data
    ///
    pub fn load_from_save(&mut self, screen_data: ScreenData) {
        self.data.data = screen_data.data;
        self.data.alpha = screen_data.alpha;
        self.data.mode = screen_data.mode;
    }
}

impl fmt::Debug for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let coef = self.get_screen_size_coef();
        writeln!(
            f,
            "    -> Size: {} x {}",
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
