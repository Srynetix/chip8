//! CPU stack.

use std::fmt;

use nanoserde::{DeBin, SerBin};

use super::types::{C8Addr, C8Byte};

/// CPU stack depth.
const STACK_DEPTH: usize = 16;

/// CPU stack struct.
#[derive(Clone, DeBin, SerBin, Default)]
pub struct Stack {
    data: Vec<C8Addr>,
    pointer: C8Byte,
}

impl Stack {
    /// Create new stack.
    ///
    /// # Returns
    ///
    /// * Stack instance.
    ///
    pub fn new() -> Self {
        Stack {
            data: vec![0; STACK_DEPTH],
            pointer: 0,
        }
    }

    /// Store address in stack.
    ///
    /// # Arguments
    ///
    /// * `addr` - Address to store.
    ///
    pub fn push(&mut self, addr: C8Addr) {
        if self.pointer as usize >= STACK_DEPTH {
            panic!("CPU stack is full ! (limit: {})", STACK_DEPTH);
        }

        self.data[self.pointer as usize] = addr;
        self.pointer += 1
    }

    /// Get data.
    ///
    /// # Returns
    ///
    /// * Data.
    ///
    pub fn get_data(&self) -> &[C8Addr] {
        &self.data
    }

    /// Get pointer.
    ///
    /// # Returns
    ///
    /// * Pointer.
    ///
    pub fn get_pointer(&self) -> C8Byte {
        self.pointer
    }

    /// Peek value in stack.
    pub fn peek(&self, idx: usize) -> C8Addr {
        self.data[idx]
    }

    /// Pop address from stack.
    ///
    /// # Returns
    ///
    /// * Last stack address.
    ///
    pub fn pop(&mut self) -> C8Addr {
        if self.pointer == 0 {
            panic!("CPU stack is empty !");
        }

        self.pointer -= 1;
        self.data[self.pointer as usize]
    }

    /// Check if empty.
    ///
    /// # Returns
    ///
    /// * `true` if empty.
    /// * `false` if not.
    ///
    pub fn empty(&self) -> bool {
        self.pointer == 0
    }

    /// Reset stack.
    pub fn reset(&mut self) {
        self.data = vec![0; STACK_DEPTH];
        self.pointer = 0;
    }

    /// Load from save.
    ///
    /// # Arguments
    ///
    /// * `stack` - Stack.
    ///
    pub fn load_from_save(&mut self, stack: Stack) {
        self.data = stack.data;
        self.pointer = stack.pointer;
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, v) in self.data.iter().enumerate() {
            writeln!(f, "    S{:X}: {:04X},", idx, v)?;
        }

        writeln!(f, "    SP: {:02X}", self.pointer)
    }
}
