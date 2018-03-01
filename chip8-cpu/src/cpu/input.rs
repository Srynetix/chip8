//! CHIP-8 input system
//! 
//! Keys:
//!     1 2 3 C
//!     4 5 6 D
//!     7 8 9 E
//!     A 0 B F
//! 


use std::fmt;
use std::sync::{RwLock};

use chip8_core::types::{C8RegIdx, C8Byte};

const INPUT_STATE_COUNT: usize = 16;

/// Input state struct
pub struct InputState(Vec<RwLock<C8Byte>>);

impl InputState {

    /// Create new input state
    pub fn new() -> Self {
        let mut vec = Vec::with_capacity(INPUT_STATE_COUNT);
        for _ in 0..INPUT_STATE_COUNT {
            vec.push(RwLock::new(0));
        }

        InputState(vec)
    }

    /// Press input
    pub fn press(&self, key: C8RegIdx) {
        if key as usize >= INPUT_STATE_COUNT {
            panic!("Key `{}` does not exist.", key);
        }

        let state_key = &self.0[key as usize];
        *(state_key.write().unwrap()) = 1;
    }

    /// Release input
    pub fn release(&self, key: C8RegIdx) {
        if key as usize >= INPUT_STATE_COUNT {
            panic!("Key `{}` does not exist.", key);
        }

        let state_key = &self.0[key as usize];
        *(state_key.write().unwrap()) = 0;
    }

    /// Get input
    pub fn get(&self, key: C8RegIdx) -> C8Byte {
        if key as usize >= INPUT_STATE_COUNT {
            panic!("Key `{}` does not exist.", key);
        }

        let state_key = &self.0[key as usize];
        *(state_key.read().unwrap())
    }
}

impl fmt::Debug for InputState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, v) in self.0.iter().enumerate() {
            write!(
                f,
                "    K{:X}: {}\n",
                idx, *(v.read().unwrap()) 
            )?;
        }

        write!(f, "\n")
    }
}