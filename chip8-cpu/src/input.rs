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
use std::collections::HashMap;

use super::types::{C8RegIdx, C8Byte};

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::{Scancode, Keycode};

/// Input state count
pub const INPUT_STATE_COUNT: usize = 16;
/// Input empty key
pub const INPUT_EMPTY_KEY: C8Byte = 0xFF;

/// Input state struct
pub struct InputState {
    event_pump: sdl2::EventPump,
    data: Vec<C8Byte>,
    key_binding: HashMap<C8Byte, Keycode>,

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

        let mut initial_binding = HashMap::new();
        initial_binding.insert(0x1, Keycode::Num1);
        initial_binding.insert(0x2, Keycode::Num2);
        initial_binding.insert(0x3, Keycode::Num3);
        initial_binding.insert(0xC, Keycode::Num4);
        
        initial_binding.insert(0x4, Keycode::A);
        initial_binding.insert(0x5, Keycode::Z);
        initial_binding.insert(0x6, Keycode::E);
        initial_binding.insert(0xD, Keycode::R);
        
        initial_binding.insert(0x7, Keycode::Q);
        initial_binding.insert(0x8, Keycode::S);
        initial_binding.insert(0x9, Keycode::D);
        initial_binding.insert(0xE, Keycode::F);
        
        initial_binding.insert(0xA, Keycode::W);
        initial_binding.insert(0x0, Keycode::X);
        initial_binding.insert(0xB, Keycode::C);
        initial_binding.insert(0xF, Keycode::V);

        InputState {
            event_pump: context.event_pump().unwrap(),
            data: vec,
            last_pressed_key: INPUT_EMPTY_KEY,
            should_close: false,
            input_pressed: false,
            key_binding: initial_binding
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
                _ => {}
            }
        }

        // Keyboard state
        for key in 0..INPUT_STATE_COUNT {
            let key8 = key as C8Byte;
            let kb = Scancode::from_keycode(*self.key_binding.get(&key8).unwrap()).unwrap();

            if self.event_pump.keyboard_state().is_scancode_pressed(kb) {
                self.press(key8);
            } else {
                self.release(key8);
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
        self.last_pressed_key = 255;
        self.input_pressed = false;
    }

    /// Get input
    pub fn get(&self, key: C8RegIdx) -> C8Byte {
        if key as usize >= INPUT_STATE_COUNT {
            panic!("Key `{}` does not exist.", key);
        }

        self.data[key as usize]
    }

    /// Dump
    pub fn dump(&self) {
        println!("{:?}", &self);
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