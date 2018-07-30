//! CHIP-8 CPU memory

use std::fmt;

use super::opcodes::extract_opcode_from_array;
use super::types::{C8Addr, C8Byte};

/// CHIP-8 CPU memory vars
const MEMORY_SIZE: usize = 4096;
const CHUNK_SIZE: usize = 64;

/// CHIP-8 initial memory pointer
pub const INITIAL_MEMORY_POINTER: C8Addr = 0x200;

/// CHIP-8 CPU memory struct
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Memory {
    data: Vec<C8Byte>,
    pointer: C8Addr,
}

impl Memory {
    /// Create new memory
    pub fn new() -> Self {
        Memory {
            data: vec![0; MEMORY_SIZE],
            pointer: INITIAL_MEMORY_POINTER,
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
        self.read_opcode_at_address(self.pointer)
    }

    /// Read opcode at address
    ///
    /// # Arguments
    ///
    /// * `addr` - Address
    ///
    pub fn read_opcode_at_address(&self, addr: C8Addr) -> C8Addr {
        extract_opcode_from_array(&self.data, addr as usize)
    }

    /// Reset memory
    pub fn reset(&mut self) {
        self.data = vec![0; MEMORY_SIZE];
        self.pointer = INITIAL_MEMORY_POINTER;
    }

    /// Load from save
    ///
    /// # Arguments
    ///
    /// * `memory` - Memory
    ///
    pub fn load_from_save(&mut self, memory: Memory) {
        self.data = memory.data;
        self.pointer = memory.pointer;
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Print row
        print_row(f)?;
        print_separator(f)?;

        for (idx, chunk) in self.data.chunks(CHUNK_SIZE).enumerate() {
            write!(
                f,
                "    {:04X}-{:04X} | ",
                idx * CHUNK_SIZE,
                (idx + 1) * CHUNK_SIZE
            )?;

            for chunk_value in chunk.iter() {
                write!(f, "{:02X} ", chunk_value)?;
            }

            write!(f, "\n")?;
        }

        // Reprint row
        print_separator(f)?;
        print_row(f)?;

        write!(f, "    PC: {:04X}\n", self.pointer)
    }
}

fn print_separator(f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "                ")?;
    for _ in 0..CHUNK_SIZE {
        write!(f, "---")?;
    }

    write!(f, "\n")
}

fn print_row(f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "                ")?;
    for v in 0..CHUNK_SIZE {
        write!(f, "{:02X} ", v * 2)?;
    }

    write!(f, "\n")
}
