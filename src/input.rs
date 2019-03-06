//! CHIP-8 input system
//!
//! Keys:
//!     1 2 3 C
//!     4 5 6 D
//!     7 8 9 E
//!     A 0 B F
//!

use std::collections::HashMap;
use std::fmt;

use super::types::{C8Byte, C8RegIdx};

// use sdl2;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::EventPump;

/// Input state count
pub const INPUT_STATE_COUNT: usize = 16;
/// Input empty key
pub const INPUT_EMPTY_KEY: C8Byte = 0xFF;

// const RESET_KEYCODE: Keycode = Keycode::F5;
// const SAVE_STATE_KEYCODE: Keycode = Keycode::F7;
// const LOAD_STATE_KEYCODE: Keycode = Keycode::F8;

/// Input state data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputStateData {
    data: Vec<C8Byte>,
    last_pressed_key: C8Byte,
    input_pressed: bool,

    /// Flags
    pub flags: InputStateFlags,
}

/// Input state flags
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputStateFlags {
    /// Should close
    pub should_close: bool,
    /// Should reset
    pub should_reset: bool,
    /// Should save
    pub should_save: bool,
    /// Should load
    pub should_load: bool,
}

/// Input state struct
pub struct InputState {
    /// State data
    pub data: InputStateData,
    key_binding: HashMap<C8Byte, Keycode>,
}

impl Default for InputState {
    fn default() -> Self {
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

        Self {
            key_binding: initial_binding,
            data: InputStateData {
                data: vec,
                last_pressed_key: INPUT_EMPTY_KEY,
                input_pressed: false,
                flags: InputStateFlags {
                    should_close: false,
                    should_reset: false,
                    should_save: false,
                    should_load: false,
                },
            },
        }
    }
}

impl InputState {
    /// Create new input state
    pub fn new() -> Self {
        Default::default()
    }

    /// Update input state
    pub fn update_state(&mut self) {
        // let events: Vec<Event> = self.event_pump.poll_iter().collect();

        // for event in events {
        //     match event {
        //         Event::Quit { .. }
        //         | Event::KeyDown {
        //             keycode: Some(Keycode::Escape),
        //             ..
        //         } => {
        //             self.data.flags.should_close = true;
        //         }
        //         Event::KeyDown {
        //             keycode: Some(RESET_KEYCODE),
        //             ..
        //         } => {
        //             self.data.flags.should_reset = true;
        //         }
        //         Event::KeyDown {
        //             keycode: Some(LOAD_STATE_KEYCODE),
        //             ..
        //         } => {
        //             self.data.flags.should_load = true;
        //         }
        //         Event::KeyDown {
        //             keycode: Some(SAVE_STATE_KEYCODE),
        //             ..
        //         } => {
        //             self.data.flags.should_save = true;
        //         }
        //         _ => {}
        //     }
        // }
    }

    /// Process input
    pub fn process_input(&mut self, event_pump: &mut EventPump) {
        // Keyboard state
        for key in 0..INPUT_STATE_COUNT {
            let key8 = key as C8Byte;
            let kb = Scancode::from_keycode(self.key_binding[&key8]).unwrap();

            if event_pump.keyboard_state().is_scancode_pressed(kb) {
                self.press(key8);
            } else {
                self.release(key8);
            }
        }
    }

    /// Wait for input
    pub fn wait_for_input(&mut self) -> C8Byte {
        // self.data.input_pressed = false;

        // loop {
        //     self.update_state();

        //     if self.data.input_pressed || self.data.flags.should_close {
        //         break;
        //     }

        //     sleep(Duration::from_millis(10));
        // }

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

    /// Get input data
    pub fn get_data(&self) -> &[C8Byte] {
        &self.data.data
    }

    /// Get last pressed key
    pub fn get_last_pressed_key(&self) -> C8Byte {
        self.data.last_pressed_key
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, v) in self.data.data.iter().enumerate() {
            writeln!(f, "    K{:X}: {}", idx, v)?;
        }

        writeln!(f, "    LK: {}", self.data.last_pressed_key)
    }
}
