//! CHIP-8 input system
//! 
//! Keys:
//!     1 2 3 C
//!     4 5 6 D
//!     7 8 9 E
//!     A 0 B F
//! 

use std::fmt;
use std::thread::{sleep};
use std::time::{Duration};

use chip8_core::types::{C8RegIdx, C8Byte};

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

/// Input state count
pub const INPUT_STATE_COUNT: usize = 16;
/// Input empty key
pub const INPUT_EMPTY_KEY: C8Byte = 0xFF;

/// Input state struct
pub struct InputState {
    event_pump: sdl2::EventPump,
    data: Vec<C8Byte>,

    last_pressed_key: C8Byte,
    input_pressed: bool,

    /// Should close
    pub should_close: bool    
}

impl InputState {

    /// Create new input state
    pub fn new(context: &sdl2::Sdl) -> Self {
        let mut vec = Vec::with_capacity(INPUT_STATE_COUNT);
        for _ in 0..INPUT_STATE_COUNT {
            vec.push(0);
        }

        InputState {
            event_pump: context.event_pump().unwrap(),
            data: vec,
            last_pressed_key: INPUT_EMPTY_KEY,
            should_close: false,
            input_pressed: false
        }
    }

    /// Update input state
    pub fn update_state(&mut self) {
        let events: Vec<Event> = self.event_pump.poll_iter().collect();

        for event in events {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.should_close = true;
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    let key = key_handle(keycode);
                    if key <= 0xF {
                        self.press(key);
                    }
                },
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    let key = key_handle(keycode);
                    if key <= 0xF {
                        self.release(key);
                    }
                },
                _ => {}
            }
        }
    }

    /// Wait for input
    pub fn wait_for_input(&mut self) -> C8Byte {
        self.input_pressed = false;

        loop {
            self.update_state();

            if self.input_pressed || self.should_close {
                break;
            }

            sleep(Duration::from_millis(10));
        }

        self.last_pressed_key
    }

    /// Check if a key is valid
    pub fn is_key_valid(key: C8Byte) -> bool {
        key <= INPUT_STATE_COUNT as C8Byte
    }

    /// Press input
    pub fn press(&mut self, key: C8RegIdx) {
        if key as usize >= INPUT_STATE_COUNT {
            panic!("Key `{}` does not exist.", key);
        }

        self.data[key as usize] = 1;
        self.last_pressed_key = key;
        self.input_pressed = true;
    }

    /// Release input
    pub fn release(&mut self, key: C8RegIdx) {
        if key as usize >= INPUT_STATE_COUNT {
            panic!("Key `{}` does not exist.", key);
        }

        self.data[key as usize] = 0;
        self.last_pressed_key = key;        
        self.input_pressed = true;
    }

    /// Get input
    pub fn get(&self, key: C8RegIdx) -> C8Byte {
        if key as usize >= INPUT_STATE_COUNT {
            panic!("Key `{}` does not exist.", key);
        }

        self.data[key as usize]
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

impl fmt::Debug for InputState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, v) in self.data.iter().enumerate() {
            write!(
                f,
                "    K{:X}: {}\n",
                idx, v
            )?;
        }

        write!(f, "    LK: {}\n", self.last_pressed_key)
    }
}