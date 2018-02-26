//! CHIP-8 video memory
//! 
//! 1: On
//! 0: Off

use std::fmt;

use super::types::C8Byte;

/// Video memory width
pub const VIDEO_MEMORY_WIDTH: usize = 64;
/// Video memory height
pub const VIDEO_MEMORY_HEIGHT: usize = 32;

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

    /// Toggle pixel position
    /// Return true if collision
    /// 
    /// # Arguments
    /// 
    /// * `pos` - Position
    /// 
    pub fn toggle_pixel(&mut self, pos: usize) -> bool {
        // For now, only handle 0 and 1
        let mut flip = false;
        
        if self.0[pos] == 1 {
            self.0[pos] = 0;
            flip = true;
        } else {
            self.0[pos] = 1;
        }

        flip
    }

    /// Toggle pixel w/ X/Y coordinates
    /// Return true if collision
    /// 
    /// # Arguments
    /// 
    /// * `x` - X coordinate
    /// * `y` - Y coordinate
    /// 
    pub fn toggle_pixel_xy(&mut self, x: usize, y: usize) -> bool {
        self.toggle_pixel(x + y * VIDEO_MEMORY_WIDTH)
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