//! CHIP-8 video memory
//! 
//! 1: On
//! 0: Off

use std::fmt;
use std::sync::{Arc, RwLock};

use chip8_core::types::{SharedC8ByteVec};

/// Video memory width
pub const VIDEO_MEMORY_WIDTH: usize = 64;
/// Video memory height
pub const VIDEO_MEMORY_HEIGHT: usize = 32;

const VIDEO_MEMORY_SIZE: usize = VIDEO_MEMORY_WIDTH * VIDEO_MEMORY_HEIGHT;

/// CHIP-8 video memory struct
pub struct VideoMemory(SharedC8ByteVec);

impl VideoMemory {

    /// Create new video memory
    pub fn new() -> Self {
        VideoMemory(
            Arc::new(
                RwLock::new(
                    vec![0; VIDEO_MEMORY_SIZE]
                )
            )
        )
    }

    /// Clear screen
    pub fn clear_screen(&mut self) {
        let screen_handle = Arc::clone(&self.0); 
        let mut screen = screen_handle.write().expect("Could not write to screen");

        for x in 0..screen.len() {
            screen[x] = 0;
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
        let screen_handle = Arc::clone(&self.0);         
        let mut screen = screen_handle.write().expect("Could not write to screen");        
        
        if screen[pos] == 1 {
            screen[pos] = 0;
            flip = true;
        } else {
            screen[pos] = 1;
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

    /// Get read-only data
    pub fn get_read_only_data(&self) -> SharedC8ByteVec {
        self.0.clone()
    }
}

impl fmt::Debug for VideoMemory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let screen_handle = Arc::clone(&self.0);         
        let screen = screen_handle.read().expect("Could not read screen");
        write!(f, "    -> Size: {} x {}\n", VIDEO_MEMORY_WIDTH, VIDEO_MEMORY_HEIGHT)?;

        for j in 0..VIDEO_MEMORY_HEIGHT {
            write!(f, "    ")?;

            for i in 0..VIDEO_MEMORY_WIDTH {
                write!(f, "{:02X} ", screen[i + j * VIDEO_MEMORY_WIDTH])?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}