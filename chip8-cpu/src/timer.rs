//! CHIP-8 CPU timer

use std::fmt;

use super::types::{C8Byte};

/// CHIP-8 CPU timer
pub struct Timer {
    title: String,
    count: C8Byte,
}

impl Timer {
    
    /// Create new timer
    pub fn new(title: String) -> Self {
        Timer {
            title: title,
            count: 0
        }
    }

    /// Decrement timer
    pub fn decrement(&mut self) -> &Self {
        if self.count > 0 {
            self.count -= 1;

            if self.count == 0 {
                debug!("- Timer `{}` finished.", self.title)
            }
        }

        self
    }

    /// Reset timer with value
    /// 
    /// # Arguments
    /// 
    /// * `value`: Value
    /// 
    pub fn reset(&mut self, value: C8Byte) -> &Self {
        self.count = value;
        self
    }

    /// Get value
    pub fn get_value(&self) -> C8Byte {
        self.count
    }
}

impl fmt::Debug for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02X}", self.count)
    }
}