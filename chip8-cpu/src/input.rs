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

const RESET_KEYCODE: Keycode = Keycode::F5;
const SAVE_STATE_KEYCODE: Keycode = Keycode::F7;
const LOAD_STATE_KEYCODE: Keycode = Keycode::F8;

/// Input state data
#[derive(Clone)]
pub struct InputStateData {   
    data: Vec<C8Byte>,
    key_binding: HashMap<C8Byte, Keycode>,
    last_pressed_key: C8Byte,
    input_pressed: bool,

    /// Flags
    pub flags: InputStateFlags
}

/// Input state flags
#[derive(Clone)]
pub struct InputStateFlags {
    /// Should close
    pub should_close: bool,
    /// Should reset
    pub should_reset: bool,
    /// Should save
    pub should_save: bool,
    /// Should load
    pub should_load: bool   
}

/// Input state struct
pub struct InputState {
    event_pump: sdl2::EventPump,
    pub data: InputStateData
}

impl InputState {

    /// Create new input state
    /// 
    /// # Arguments
    /// 
    /// * `context` - SDL2 context
    /// 
    pub fn new(context: &sdl2::Sdl) -> Self {
        let vec = vec![0; INPUT_STATE_COUNT];

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
            data: InputStateData {
                data: vec,
                last_pressed_key: INPUT_EMPTY_KEY,
                input_pressed: false,
                key_binding: initial_binding,
                flags: InputStateFlags {
                    should_close: false,
                    should_reset: false,
                    should_save: false,
                    should_load: false
                }
            }
        }
    }

    /// Update input state
    pub fn update_state(&mut self) {
        let events: Vec<Event> = self.event_pump.poll_iter().collect();

        for event in events {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.data.flags.should_close = true;
                },
                Event::KeyDown { keycode: Some(RESET_KEYCODE), .. } => {
                    self.data.flags.should_reset = true;
                },
                Event::KeyDown { keycode: Some(LOAD_STATE_KEYCODE), .. } => {
                    self.data.flags.should_load = true;
                },
                Event::KeyDown { keycode: Some(SAVE_STATE_KEYCODE), .. } => {
                    self.data.flags.should_save = true;
                }
                _ => {}
            }
        }

        // Keyboard state
        for key in 0..INPUT_STATE_COUNT {
            let key8 = key as C8Byte;
            let kb = Scancode::from_keycode(*self.data.key_binding.get(&key8).unwrap()).unwrap();

            if self.event_pump.keyboard_state().is_scancode_pressed(kb) {
                self.press(key8);
            } else {
                self.release(key8);
            }
        }
    }

    /// Wait for input
    pub fn wait_for_input(&mut self) -> C8Byte {
        self.data.input_pressed = false;

        loop {
            self.update_state();

            if self.data.input_pressed || self.data.flags.should_close {
                break;
            }

            sleep(Duration::from_millis(10));
        }

        self.data.last_pressed_key
    }

    /// Press input
    /// 
    /// # Arguments
    /// 
    /// * `key` - Input key
    /// 
    pub fn press(&mut self, key: C8RegIdx) {
        if key as usize >= INPUT_STATE_COUNT {
            panic!("Key `{}` does not exist.", key);
        }

        self.data.data[key as usize] = 1;
        self.data.last_pressed_key = key;
        self.data.input_pressed = true;
    }

    /// Release input
    /// 
    /// # Arguments
    /// 
    /// * `key` - Input key
    /// 
    pub fn release(&mut self, key: C8RegIdx) {
        if key as usize >= INPUT_STATE_COUNT {
            panic!("Key `{}` does not exist.", key);
        }

        self.data.data[key as usize] = 0;
        self.data.last_pressed_key = INPUT_EMPTY_KEY;
        self.data.input_pressed = false;
    }

    /// Get input
    /// 
    /// # Arguments
    /// 
    /// * `key` - Input key
    /// 
    pub fn get(&self, key: C8RegIdx) -> C8Byte {
        if key as usize >= INPUT_STATE_COUNT {
            panic!("Key `{}` does not exist.", key);
        }

        self.data.data[key as usize]
    }

    /// Dump
    pub fn dump(&self) {
        println!("{:?}", &self);
    }

    /// Load from save
    /// 
    /// # Arguments
    /// 
    /// * `data` - Input state data
    /// 
    pub fn load_from_save(&mut self, data: InputStateData) {
        self.data = data;
    }

    /// Reset
    pub fn reset(&mut self) {
        self.data.data = vec![0; INPUT_STATE_COUNT];
        self.data.last_pressed_key = INPUT_EMPTY_KEY;
        self.data.input_pressed = false;

        self.data.flags.should_close = false;
        self.data.flags.should_reset = false;
        self.data.flags.should_save = false;
        self.data.flags.should_load = false;  
    }
}

impl fmt::Debug for InputState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, v) in self.data.data.iter().enumerate() {
            write!(
                f,
                "    K{:X}: {}\n",
                idx, v
            )?;
        }

        write!(f, "    LK: {}\n", self.data.last_pressed_key)
    }
}