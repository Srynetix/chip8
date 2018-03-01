//! CHIP-8 graphics renderer

use std::thread::{sleep};
use std::time::Duration;
use std::sync::{Arc, RwLock};

use chip8_core::types::{C8Byte};
use chip8_cpu::CPU;

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
    pub fn run(&mut self, cpu: Arc<RwLock<CPU>>) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        let input = cpu.read().unwrap().get_input_state();
        let screen = cpu.read().unwrap().get_video_memory();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    Event::KeyDown { keycode: Some(keycode), .. } => {
                        let key = key_handle(keycode);
                        if key != 0xFF {
                            input.press(key);
                        }
                    },
                    Event::KeyUp { keycode: Some(keycode), .. } => {
                        let key = key_handle(keycode);
                        if key != 0xFF {
                            input.release(key);
                        }
                    },
                    _ => {}
                }
            }

            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.clear();

            {
                for (idx, px) in screen.get_raw_data().iter().enumerate() {
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
    }
}

fn key_handle(keycode: Keycode) -> C8Byte {
    match keycode {
        Keycode::A => 0x1,
        Keycode::Z => 0x2,
        Keycode::E => 0x3,
        Keycode::R => 0xC,

        Keycode::Q => 0x4,
        Keycode::S => 0x5,
        Keycode::D => 0x6,
        Keycode::F => 0xD,

        Keycode::W => 0x7,
        Keycode::X => 0x8,
        Keycode::C => 0x9,
        Keycode::V => 0xE,

        Keycode::T => 0xA,
        Keycode::Y => 0x0,
        Keycode::U => 0xB,
        Keycode::I => 0xF,

        _ => 0xFF
    }
}