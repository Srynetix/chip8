//! CHIP-8 CPU memory

use std::fmt;

use chip8_core::types::{C8Byte, C8Addr};

use super::opcodes::extract_opcode_from_array;

/// CHIP-8 CPU memory vars
const MEMORY_SIZE: usize = 4096;
const CHUNK_SIZE: usize = 64;
const INITIAL_MEMORY_POINTER: C8Addr = 0x200;

/// CHIP-8 CPU memory struct
pub struct Memory{
    data: Vec<C8Byte>,
    pointer: C8Addr
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
    pub fn write_data_at_offset(&mut self, offset: C8Addr, data: &[C8Byte]) {
        for (idx, v) in data.iter().enumerate() {
            self.data[(offset + idx as C8Addr) as usize] = *v; 
        }
    }

    /// Write byte at offset
    /// 
    /// # Arguments
    /// 
    /// * `offset` - Offset
    /// * `byte` - Byte
    /// 
    pub fn write_byte_at_offset(&mut self, offset: C8Addr, byte: C8Byte) {
        self.data[offset as usize] = byte
    }

    /// Write data at pointer
    /// 
    /// # Arguments
    /// 
    /// * `data` - Data (bytes)
    /// 
    pub fn write_data_at_pointer(&mut self, data: &[C8Byte]) {
        let pointer = self.pointer;

        self.write_data_at_offset(pointer, data)
    }

    /// Get data at offset
    /// 
    /// # Arguments
    /// 
    /// * `offset` - Offset
    /// * `count` - Count
    /// 
    pub fn read_data_at_offset(&self, offset: C8Addr, count: C8Addr) -> &[C8Byte] {
        &self.data[(offset as usize)..((offset + count) as usize)]
    }

    /// Get byte at offset
    /// 
    /// # Arguments
    /// 
    /// * `offset` - Offset
    /// 
    pub fn read_byte_at_offset(&self, offset: C8Addr) -> C8Byte {
        self.data[offset as usize]
    }

    /// Set pointer
    /// 
    /// # Arguments
    /// 
    /// * `pointer` - Pointer
    /// 
    pub fn set_pointer(&mut self, pointer: C8Addr) {
        self.pointer = pointer;
    }

    /// Get pointer
    pub fn get_pointer(&self) -> C8Addr {
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
    pub fn read_opcode(&self) -> C8Addr {
        extract_opcode_from_array(&self.data, self.pointer as usize)
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