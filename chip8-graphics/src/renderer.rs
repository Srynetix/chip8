//! CHIP-8 graphics renderer

use std::sync::{Arc};

use chip8_core::types::{SharedC8ByteVec};

use piston_window::{PistonWindow, WindowSettings};
use piston_window::{clear, rectangle};

const RENDERER_WIDTH: u32 = 64;
const RENDERER_HEIGHT: u32 = 32;
const RENDERER_SCALE: u32 = 10;

const RENDERER_CLEAR_COLOR: [f32; 4] = [1.0; 4];

/// Color struct
pub struct Color([f32; 4]);

/// CHIP-8 renderer struct
pub struct Renderer {
    window: PistonWindow
}

impl Color {

    /// Create a color from byte
    pub fn from_byte(byte: u8) -> &'static Color {
        match byte {
            0 => Self::black(),
            1 => Self::white(),
            _ => Self::black()
        }
    }

    /// Create a black color
    pub fn black() -> &'static Color {
        &Color([0.0, 0.0, 0.0, 1.0])
    }

    /// Create a white color
    pub fn white() -> &'static Color {
        &Color([1.0; 4])
    }
}

impl Renderer {

    /// Create a new renderer
    pub fn new(window_title: String) -> Self {
        let window: PistonWindow = WindowSettings::new(
                                                window_title,
                                                [RENDERER_WIDTH * RENDERER_SCALE, RENDERER_HEIGHT * RENDERER_SCALE])
                                            .exit_on_esc(true)
                                            .resizable(false)
                                            .vsync(true)
                                            .build()
                                            .unwrap();

        Renderer {
            window: window
        }
    }

    /// Start loop
    pub fn run(&mut self, screen_lock: SharedC8ByteVec) {
        let handle = Arc::clone(&screen_lock);
        
        while let Some(event) = self.window.next() {
            self.window.draw_2d(&event, |context, g2d| {
                clear(RENDERER_CLEAR_COLOR, g2d);

                {
                    let screen = handle.read().unwrap();
                    for (idx, px) in screen.iter().enumerate() {
                        let idx = idx as u32;
                        let x = idx % RENDERER_WIDTH;
                        let y = idx / RENDERER_WIDTH;

                        rectangle(
                            Color::from_byte(*px).0,
                            [(x * RENDERER_SCALE) as f64, (y * RENDERER_SCALE) as f64, RENDERER_SCALE as f64, RENDERER_SCALE as f64],
                            context.transform,
                            g2d);
                    }
                }
            });
        }
    }
}