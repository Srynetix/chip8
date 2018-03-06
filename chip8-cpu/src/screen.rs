//! CHIP-8 video memory
//! 
//! 1: On
//! 0: Off

use std::fmt;

use super::types::{C8Byte};
use super::font::{FONT_CHAR_WIDTH};

use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

/// Video memory width
pub const VIDEO_MEMORY_WIDTH: usize = 64;
/// Video memory height
pub const VIDEO_MEMORY_HEIGHT: usize = 32;
/// Renderer scale
pub const RENDERER_SCALE: usize = 10;

const VIDEO_MEMORY_SIZE: usize = VIDEO_MEMORY_WIDTH * VIDEO_MEMORY_HEIGHT;

/// CHIP-8 screen memory struct
pub struct Screen {
    data: Vec<C8Byte>,
    alpha: Vec<C8Byte>,
    renderer: sdl2::render::WindowCanvas
}

impl Screen {

    /// Create new video memory
    pub fn new(context: &sdl2::Sdl) -> Self {
        let mut data = Vec::with_capacity(VIDEO_MEMORY_SIZE);
        let mut alpha = Vec::with_capacity(VIDEO_MEMORY_SIZE);
        for _ in 0..VIDEO_MEMORY_SIZE {
            data.push(0);
            alpha.push(0);
        }

        let video_subsystem = context.video().unwrap();

        let window = video_subsystem.window("CHIP-8 Emulator",
                                            (VIDEO_MEMORY_WIDTH * RENDERER_SCALE) as u32,
                                            (VIDEO_MEMORY_HEIGHT * RENDERER_SCALE) as u32)
                                    .position_centered()
                                    .build()
                                    .unwrap();

        let mut renderer = window
                            .into_canvas()
                            .build()
                            .unwrap();

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();

        Screen {
            renderer,
            data,
            alpha
        }
    }

    /// Draw sprite
    pub fn draw_sprite(&mut self, r1: C8Byte, r2: C8Byte, sprite: &[C8Byte]) -> bool {
        let byte = sprite.len();
        let mut collision = false;

        for i in 0..byte {
            let code = sprite[i];
            let y = ((r2 as usize) + (i as usize)) % VIDEO_MEMORY_HEIGHT;
            let mut shift = FONT_CHAR_WIDTH - 1;

            for j in 0..FONT_CHAR_WIDTH {
                let x = ((r1 as usize) + (j as usize)) % VIDEO_MEMORY_WIDTH;

                if code & (0x1 << shift) != 0 {
                    if self.toggle_pixel_xy(x, y) {
                        collision = true;
                    }
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
        for x in 0..self.data.len() {
            self.data[x] = 0
        }

        self.render();
    }

    /// Fade pixels
    pub fn fade_pixels(&mut self) {
        for x in 0..self.data.len() {
            if self.data[x] == 0 {
                if self.alpha[x] > 0 {
                    self.alpha[x] = (self.alpha[x] as f32 * 0.975) as u8;
                }
            }
        }
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
        let pixel = self.data[pos];        
        
        if pixel == 1 {
            self.data[pos] = 0;
            self.alpha[pos] = 255;
            flip = true;
        } else {
            self.data[pos] = 1;
            self.alpha[pos] = 255;
        }

        flip
    }

    /// Render screen
    pub fn render(&mut self) {
        self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.renderer.clear();

        for (pos, px) in self.data.iter().enumerate() {
            let x = pos % VIDEO_MEMORY_WIDTH;
            let y = pos / VIDEO_MEMORY_WIDTH;
            let alpha = &self.alpha[pos];

            let color = color_from_byte(*px, *alpha);
            self.renderer.set_draw_color(color);
            self.renderer.fill_rect(
                Rect::new(
                    (x * RENDERER_SCALE) as i32,
                    (y * RENDERER_SCALE) as i32,
                    RENDERER_SCALE as u32,
                    RENDERER_SCALE as u32)
                )
                .expect("Error while drawing.");
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
        self.toggle_pixel(x + y * VIDEO_MEMORY_WIDTH)
    }

    /// Dump screen
    pub fn dump_screen(&self) {
        println!("{:?}", &self);
    }
}

impl fmt::Debug for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "    -> Size: {} x {}\n", VIDEO_MEMORY_WIDTH, VIDEO_MEMORY_HEIGHT)?;

        for j in 0..VIDEO_MEMORY_HEIGHT {
            write!(f, "    ")?;

            for i in 0..VIDEO_MEMORY_WIDTH {
                let pixel = self.data[i + j * VIDEO_MEMORY_WIDTH];
                if pixel == 0 {
                    write!(f, " ")?;
                } else {
                    write!(f, "█")?;
                }
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn color_from_byte(byte: C8Byte, alpha: C8Byte) -> Color {
    match byte {
        0 => Color::RGB(alpha, alpha, alpha),
        _ => Color::RGB(255, 255, 255)
    }
}