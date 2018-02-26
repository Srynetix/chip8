//! CHIP-8 video memory

use std::fmt;

use super::types::C8Byte;

const VIDEO_MEMORY_WIDTH: usize = 64;
const VIDEO_MEMORY_HEIGHT: usize = 32;

const VIDEO_MEMORY_SIZE: usize = VIDEO_MEMORY_WIDTH * VIDEO_MEMORY_HEIGHT;

/// CHIP-8 video memory struct
pub struct VideoMemory(Vec<C8Byte>);

impl VideoMemory {

    /// Create new video memory
    pub fn new() -> Self {
        VideoMemory(vec![0; VIDEO_MEMORY_SIZE])
    }

    /// Clear screen
    pub fn clear_screen(&mut self) {
        for x in 0..self.0.len() {
            self.0[x] = 0;
        }
    }
}

impl fmt::Debug for VideoMemory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "    -> Size: {} x {}\n", VIDEO_MEMORY_WIDTH, VIDEO_MEMORY_HEIGHT)?;

        for j in 0..VIDEO_MEMORY_HEIGHT {
            write!(f, "    ")?;

            for i in 0..VIDEO_MEMORY_WIDTH {
                write!(f, "{:02X} ", self.0[i + j * VIDEO_MEMORY_WIDTH])?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}