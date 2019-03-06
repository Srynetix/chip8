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

use sdl2::keyboard::{Keycode, Scancode};
use sdl2::EventPump;

use crate::core::types::{C8Byte, C8RegIdx};

/// Input state count
pub const INPUT_STATE_COUNT: usize = 16;
/// Input empty key
pub const INPUT_EMPTY_KEY: C8Byte = 0xFF;

/// Input state data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputStateData {
    data: Vec<C8Byte>,
    last_pressed_key: C8Byte,
    input_pressed: bool,

    /// Lock
    pub lock: InputLock,
}

/// Input lock
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputLock {
    /// Active
    pub active: bool,
    /// Register
    pub register: C8RegIdx,
    /// Key
    pub key: C8Byte,
}

impl InputLock {
    /// Check if key is set
    pub fn is_key_set(&self) -> bool {
        self.key != INPUT_EMPTY_KEY
    }

    /// Is locked
    pub fn is_locked(&self) -> bool {
        self.active
    }

    /// Lock
    pub fn lock(&mut self, register: C8RegIdx) -> bool {
        if self.active {
            false
        } else {
            self.active = true;
            self.register = register;
            self.key = INPUT_EMPTY_KEY;

            true
        }
    }

    /// Unlock
    pub fn unlock(&mut self) -> bool {
        if !self.active {
            false
        } else {
            self.active = false;
            self.register = INPUT_EMPTY_KEY;
            self.key = INPUT_EMPTY_KEY;

            true
        }
    }

    /// Set key
    pub fn set_key(&mut self, key: C8Byte) {
        self.key = key;
    }
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
                lock: InputLock {
                    active: false,
                    register: 255,
                    key: 255,
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
    pub fn wait_for_input(&mut self, register: C8RegIdx) {
        self.data.lock.active = true;
        self.data.lock.register = register;
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

        // Handle lock
        if self.data.lock.is_locked() && !self.data.lock.is_key_set() {
            self.data.lock.set_key(key);
        }
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
