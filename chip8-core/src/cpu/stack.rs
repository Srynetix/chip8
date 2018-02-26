//! CHIP-8 CPU stack

use std::fmt;

use super::types::{C8Byte, C8Addr};

/// CHIP-8 CPU stack depth
const STACK_DEPTH: usize = 16;

/// CHIP-8 CPU stack struct
pub struct Stack {
    data: Vec<C8Addr>,
    pointer: C8Byte
}

impl Stack {

    /// Create new stack
    pub fn new() -> Self {
        Stack {
            data: vec![0; STACK_DEPTH],
            pointer: 0
        }
    }

    /// Store address in stack
    ///
    /// # Arguments
    ///
    /// * `addr` - Address to store
    ///
    pub fn store(&mut self, addr: C8Addr) {
        if self.pointer as usize >= STACK_DEPTH {
            panic!("CPU stack is full ! (limit: {})", STACK_DEPTH);
        }

        self.data[self.pointer as usize] = addr;
        self.pointer += 1
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, v) in self.data.iter().enumerate() {
            write!(f, "    S{:X}: {:02X},\n", idx, v)?;
        }

        write!(f, "    SP: {:04X}\n", self.pointer)
    }
}