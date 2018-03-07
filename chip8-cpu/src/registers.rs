//! CHIP-8 CPU Registers

use std::fmt;

use super::types::{C8Byte, C8Addr, C8RegIdx};

/// CHIP-8 register count
const REGISTER_COUNT: usize = 16;

/// CHIP-8 CPU Registers
#[derive(Clone)]
pub struct Registers {
    data: Vec<C8Byte>,
    i: C8Addr
}

impl Registers {

    /// Create CPU Registers
    pub fn new() -> Self {
        Registers {
            data: vec![0; REGISTER_COUNT],
            i: 0
        }
    }

    /// Get register value
    ///
    /// # Arguments
    ///
    /// * `reg` - Register name
    ///
    pub fn get_register(&self, reg: C8RegIdx) -> C8Byte {
        let reg = reg as usize;

        if reg >= REGISTER_COUNT {
            panic!("Bad register name: {:X}", reg);
        }

        self.data[reg]
    }

    /// Get I register
    pub fn get_i_register(&self) -> C8Addr {
        self.i
    }

    /// Set register value
    ///
    /// # Arguments
    ///
    /// * `reg` - Register name
    /// * `value` - Byte value
    ///
    pub fn set_register(&mut self, reg: C8RegIdx, value: C8Byte) {
        let reg = reg as usize;
        
        if reg >= REGISTER_COUNT {
            panic!("Bad register name: {:X}", reg);
        }

        self.data[reg] = value;
    }

    /// Set carry register
    ///
    /// # Arguments
    ///
    /// * `value` - Byte value
    ///
    pub fn set_carry_register(&mut self, value: C8Byte) {
        self.data[15] = value;
    }

    /// Set I register
    ///
    /// # Arguments
    ///
    /// * `value` - Address
    /// 
    pub fn set_i_register(&mut self, value: C8Addr) {
        self.i = value;
    }

    /// Reset registers
    pub fn reset(&mut self) {
        self.data = vec![0; REGISTER_COUNT];
        self.i = 0;
    }

    /// Load from save
    /// 
    /// # Arguments
    /// 
    /// * `registers` - Registers
    /// 
    pub fn load_from_save(&mut self, registers: Registers) {
        self.data = registers.data;
        self.i = registers.i;
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