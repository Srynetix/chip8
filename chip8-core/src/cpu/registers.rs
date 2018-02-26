//! CHIP-8 CPU Registers

use std::fmt;

use super::types::C8Byte;

/// CHIP-8 register count
const REGISTER_COUNT: usize = 16;

/// CHIP-8 CPU Registers
pub struct Registers {
    data: Vec<C8Byte>,
    i: C8Byte
}

impl Registers {

    /// Create CPU Registers
    pub fn new() -> Self {
        Registers {
            data: vec![0; REGISTER_COUNT],
            i: 0
        }
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, rx) in self.data.iter().enumerate() {
            write!(f, "    V{:X}: {:02X},\n", idx, rx)?;
        }

        write!(f, "    I: {:02X}\n", self.i)
    }
}