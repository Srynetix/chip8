//! CHIP-8 CPU memory

use std::fmt;

use super::types::C8Byte;
use super::types::C8Short;

/// CHIP-8 CPU memory vars
const MEMORY_SIZE: usize = 4096;
const CHUNK_SIZE: usize = 64;
const INITIAL_MEMORY_POINTER: C8Short = 512;

/// CHIP-8 CPU memory struct
pub struct Memory{
    data: Vec<C8Byte>,
    pointer: C8Short
}

impl Memory {

    /// Create new memory
    pub fn new() -> Self {
        Memory {
            data: vec![0; MEMORY_SIZE],
            pointer: INITIAL_MEMORY_POINTER
        }
    }

    /// Write data at offset
    /// 
    /// # Arguments
    /// 
    /// * `offset` - Offset
    /// * `data` - Data (bytes)
    /// 
    pub fn write_data_at_offset(&mut self, offset: usize, data: &[C8Byte]) {
        for (idx, v) in data.iter().enumerate() {
            self.data[offset + idx] = *v; 
        }
    }

    /// Write data at pointer
    /// 
    /// # Arguments
    /// 
    /// * `data` - Data (bytes)
    pub fn write_data_at_pointer(&mut self, data: &[C8Byte]) {
        let pointer = self.pointer as usize;

        self.write_data_at_offset(pointer, data)
    }

    /// Set pointer
    /// 
    /// # Arguments
    /// 
    /// * `pointer` - Pointer
    /// 
    pub fn set_pointer(&mut self, pointer: C8Short) {
        self.pointer = pointer;
    }

    /// Get pointer
    pub fn get_pointer(&self) -> C8Short {
        self.pointer
    }

    /// Advance pointer of 2
    pub fn advance_pointer(&mut self) {
        self.pointer += 2;
    }

    /// Reset pointer at initial value
    pub fn reset_pointer(&mut self) {
        self.pointer = INITIAL_MEMORY_POINTER;
    }

    /// Read opcode
    pub fn read_opcode(&self) -> C8Short {
        let pc = self.pointer as usize;

        ((self.data[pc] as C8Short) << 8) + self.data[pc + 1] as C8Short
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, chunk) in self.data.chunks(CHUNK_SIZE).enumerate() {
            write!(
                f,
                "    {:04X}-{:04X} - ",
                idx * CHUNK_SIZE,
                (idx + 1) * CHUNK_SIZE)?;
            
            for chunk_value in chunk.iter() {
                write!(
                    f,
                    "{:02X} ",
                    chunk_value)?;
            }

            write!(f, "\n")?;
        }

        write!(f, "    PC: {:04X}\n", self.pointer)
    }
}