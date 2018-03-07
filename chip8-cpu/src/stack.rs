//! CHIP-8 CPU stack

use std::fmt;

use super::types::{C8Byte, C8Addr};

/// CHIP-8 CPU stack depth
const STACK_DEPTH: usize = 16;

/// CHIP-8 CPU stack struct
#[derive(Clone)]
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
    pub fn push(&mut self, addr: C8Addr) {
        if self.pointer as usize >= STACK_DEPTH {
            panic!("CPU stack is full ! (limit: {})", STACK_DEPTH);
        }

        self.data[self.pointer as usize] = addr;
        self.pointer += 1
    }

    /// Pop address from stack
    pub fn pop(&mut self) -> C8Addr {
        if self.pointer == 0 {
            panic!("CPU stack is empty !");
        }

        self.pointer -= 1;
        self.data[self.pointer as usize]
    }

    /// Check if empty
    pub fn empty(&self) -> bool {
        self.pointer == 0
    }

    /// Reset stack
    pub fn reset(&mut self) {
        self.data = vec![0; STACK_DEPTH];
        self.pointer = 0;
    }

    /// Load from save
    /// 
    /// # Arguments
    /// 
    /// * `stack` - Stack
    /// 
    pub fn load_from_save(&mut self, stack: Stack) {
        self.data = stack.data;
        self.pointer = stack.pointer;
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, v) in self.data.iter().enumerate() {
            write!(f, "    S{:X}: {:04X},\n", idx, v)?;
        }

        write!(f, "    SP: {:02X}\n", self.pointer)
    }
}