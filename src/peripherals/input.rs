//! Input system.
//!
//! Keys:
//!     1 2 3 C
//!     4 5 6 D
//!     7 8 9 E
//!     A 0 B F
//!

use std::fmt;

use serde_derive::{Deserialize, Serialize};

use crate::core::types::{C8Byte, C8RegIdx};

/// Input state count.
pub const INPUT_STATE_COUNT: usize = 16;
/// Input empty key.
pub const INPUT_EMPTY_KEY: C8Byte = 0xFF;

/// Input lock.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputLock {
    active: bool,
    register: C8RegIdx,
    key: C8Byte,
}

impl InputLock {
    /// Check if key is set.
    ///
    /// # Returns
    ///
    /// * `true` if a key is set.
    /// * `false` if not.
    ///
    pub fn is_key_set(&self) -> bool {
        self.key != INPUT_EMPTY_KEY
    }

    /// Check if locked.
    ///
    /// # Returns
    ///
    /// * `true` if locked.
    /// * `false` if not.
    ///
    pub fn is_locked(&self) -> bool {
        self.active
    }

    /// Reset.
    pub fn reset(&mut self) {
        self.active = false;
        self.register = INPUT_EMPTY_KEY;
        self.key = INPUT_EMPTY_KEY;
    }

    /// Enable lock.
    ///
    /// # Arguments
    ///
    /// * `register` - Register.
    ///
    /// # Returns
    ///
    /// * `true` if lock has been successfully enabled.
    /// * `false` if lock was already enabled.
    ///
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

    /// Disable lock.
    ///
    /// # Returns
    ///
    /// * `true` if lock has been successfully disabled.
    /// * `false` if lock was already disabled.
    ///
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

    /// Set key.
    ///
    /// # Arguments
    ///
    /// * `key` - Key.
    ///
    pub fn set_key(&mut self, key: C8Byte) {
        self.key = key;
    }
}

/// Input state.
#[derive(Clone, Serialize, Deserialize)]
pub struct InputState {
    /// Key data.
    data: Vec<C8Byte>,
    /// Last pressed key.
    last_pressed_key: C8Byte,
    /// Input is pressed?
    input_pressed: bool,
    /// Lock.
    lock: InputLock,
}

impl Default for InputState {
    fn default() -> Self {
        let vec = vec![0; INPUT_STATE_COUNT];

        Self {
            data: vec,
            last_pressed_key: INPUT_EMPTY_KEY,
            input_pressed: false,
            lock: InputLock {
                active: false,
                register: INPUT_EMPTY_KEY,
                key: INPUT_EMPTY_KEY,
            },
        }
    }
}

impl InputState {
    /// Create new input state.
    ///
    /// # Returns
    ///
    /// * Input state instance.
    ///
    pub fn new() -> Self {
        Default::default()
    }

    /// Wait for input.
    ///
    /// # Arguments
    ///
    /// * `register` - Register.
    ///
    pub fn wait_for_input(&mut self, register: C8RegIdx) {
        self.lock.active = true;
        self.lock.register = register;
    }

    /// Press input.
    ///
    /// # Arguments
    ///
    /// * `key` - Input key.
    ///
    pub fn press(&mut self, key: C8RegIdx) {
        if key as usize >= INPUT_STATE_COUNT {
            panic!("Key `{}` does not exist.", key);
        }

        self.data[key as usize] = 1;
        self.last_pressed_key = key;
        self.input_pressed = true;

        // Handle lock
        if self.lock.is_locked() && !self.lock.is_key_set() {
            self.lock.set_key(key);
        }
    }

    /// Unlock.
    ///
    /// # Returns
    ///
    /// * `true` if successfully unlocked.
    /// * `false` if not.
    ///
    pub fn unlock(&mut self) -> bool {
        self.lock.unlock()
    }

    /// Is locked.
    ///
    /// # Returns
    ///
    /// * `true` if locked.
    /// * `false` if not.
    ///
    pub fn is_locked(&self) -> bool {
        self.lock.is_locked()
    }

    /// Is lock key set.
    ///
    /// # Returns
    ///
    /// * `true` if lock key set.
    /// * `false` if not.
    ///
    pub fn is_lock_key_set(&self) -> bool {
        self.lock.is_key_set()
    }

    /// Get lock key.
    ///
    /// # Returns
    ///
    /// * Lock key value.
    ///
    pub fn get_lock_key(&self) -> C8Byte {
        self.lock.key
    }

    /// Get lock register.
    ///
    /// # Returns
    ///
    /// * Lock register.
    ///
    pub fn get_lock_register(&self) -> C8RegIdx {
        self.lock.register
    }

    /// Release input.
    ///
    /// # Arguments
    ///
    /// * `key` - Input key
    ///
    pub fn release(&mut self, key: C8RegIdx) {
        if key as usize >= INPUT_STATE_COUNT {
            panic!("Key `{}` does not exist.", key);
        }

        self.data[key as usize] = 0;
        self.last_pressed_key = INPUT_EMPTY_KEY;
        self.input_pressed = false;
    }

    /// Get input.
    ///
    /// # Arguments
    ///
    /// * `key` - Input key.
    ///
    /// # Returns
    ///
    /// * Input value.
    ///
    pub fn get(&self, key: C8RegIdx) -> C8Byte {
        if key as usize >= INPUT_STATE_COUNT {
            panic!("Key `{}` does not exist.", key);
        }

        self.data[key as usize]
    }

    /// Get input data.
    ///
    /// # Returns
    ///
    /// * Input data.
    ///
    pub fn get_data(&self) -> &[C8Byte] {
        &self.data
    }

    /// Get last pressed key.
    ///
    /// # Returns
    ///
    /// * Last pressed key.
    ///
    pub fn get_last_pressed_key(&self) -> C8Byte {
        self.last_pressed_key
    }

    /// Load from save.
    ///
    /// # Arguments
    ///
    /// * `data` - Input state data.
    ///
    pub fn load_from_save(&mut self, data: InputState) {
        self.data = data.data;
        self.last_pressed_key = data.last_pressed_key;
        self.input_pressed = data.input_pressed;
        self.lock = data.lock;
    }

    /// Reset.
    pub fn reset(&mut self) {
        self.data = vec![0; INPUT_STATE_COUNT];
        self.last_pressed_key = INPUT_EMPTY_KEY;
        self.input_pressed = false;
        self.lock.reset();
    }
}

impl fmt::Debug for InputState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, v) in self.data.iter().enumerate() {
            writeln!(f, "    K{:X}: {}", idx, v)?;
        }

        writeln!(f, "    LK: {}", self.last_pressed_key)
    }
}
