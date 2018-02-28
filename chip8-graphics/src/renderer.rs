//! CHIP-8 graphics renderer

use std::sync::{Arc};
use std::thread::{sleep};
use std::time::Duration;

use chip8_core::types::{C8Byte, SharedC8ByteVec};

use sdl2;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const RENDERER_WIDTH: u32 = 64;
const RENDERER_HEIGHT: u32 = 32;
const RENDERER_SCALE: u32 = 4;

/// CHIP-8 renderer struct
pub struct Renderer {
    sdl_context: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>
}

fn color_from_byte(byte: C8Byte) -> Color {
    match byte {
        0 => Color::RGB(0, 0, 0),
        _ => Color::RGB(255, 255, 255)
    }
}

impl Renderer {

    /// Create a new renderer
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(
            "CHIP-8 Emulator", RENDERER_WIDTH * RENDERER_SCALE, RENDERER_HEIGHT * RENDERER_SCALE)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Renderer {
            sdl_context: sdl_context,
            canvas: canvas
        }
    }

    /// Start loop
    pub fn run(&mut self, screen_lock: SharedC8ByteVec) {
        let screen = Arc::clone(&screen_lock);
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }

            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.clear();

            {
                for (idx, px) in screen.iter().enumerate() {
                    let idx = idx as u32;
                    let x = idx % RENDERER_WIDTH;
                    let y = idx / RENDERER_WIDTH;
                    let px = px.read().unwrap();

                    self.canvas.set_draw_color(color_from_byte(*px));
                    self.canvas.fill_rect(Rect::new((x * RENDERER_SCALE) as i32, (y * RENDERER_SCALE) as i32, RENDERER_SCALE, RENDERER_SCALE)).expect("Error while drawing.");
                }
            }

            self.canvas.present();
            
            sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
        
        // while let Some(event) = self.window.next() {
        //     self.window.draw_2d(&event, |context, g2d| {
        //         clear(RENDERER_CLEAR_COLOR, g2d);

        //         {
        //             let screen = handle.read().unwrap();
        //             for (idx, px) in screen.iter().enumerate() {
        //                 let idx = idx as u32;
        //                 let x = idx % RENDERER_WIDTH;
        //                 let y = idx / RENDERER_WIDTH;

        //                 rectangle(
        //                     Color::from_byte(*px).0,
        //                     [(x * RENDERER_SCALE) as f64, (y * RENDERER_SCALE) as f64, RENDERER_SCALE as f64, RENDERER_SCALE as f64],
        //                     context.transform,
        //                     g2d);
        //             }
        //         }
        //     });
        // }
    }
}