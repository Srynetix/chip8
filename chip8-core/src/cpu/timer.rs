//! CHIP-8 CPU timer

use std::fmt;

use super::types::C8Byte;

/// CHIP-8 CPU timer
pub struct Timer(C8Byte);

impl Timer {
    
    /// Create new timer
    pub fn new() -> Self {
        Timer(0)
    }

    /// Decrement timer
    pub fn decrement(&mut self) -> &Self {
        if self.0 > 0 {
            self.0 -= 1;
        }

        self
    }

    /// Reset timer with value
    /// 
    /// # Arguments
    /// 
    /// * `value`: Value
    pub fn reset(&mut self, value: C8Byte) -> &Self {
        self.0 = value;

        self
    }

    /// Get value
    pub fn get_value(&self) -> C8Byte {
        self.0
    }
}

impl fmt::Debug for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02X}", self.0)
    }
}