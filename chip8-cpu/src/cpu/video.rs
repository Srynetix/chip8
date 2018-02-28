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
        let mut vec = Vec::with_capacity(VIDEO_MEMORY_SIZE);
        for _ in 0..VIDEO_MEMORY_SIZE {
            vec.push(RwLock::new(0));
        }

        VideoMemory(
            Arc::new(
                vec
            )
        )
    }

    /// Clear screen
    pub fn clear_screen(&mut self) {
        let screen = Arc::clone(&self.0); 

        for x in 0..screen.len() {
            let mut pixel = screen[x].write().expect("Could not write to screen");
            *pixel = 0;
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
        let screen = Arc::clone(&self.0);         
        let mut pixel = screen[pos].write().expect("Could not write to screen");        
        
        if *pixel == 1 {
            *pixel = 0;
            flip = true;
        } else {
            *pixel = 1;
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
        let screen = Arc::clone(&self.0);         
        write!(f, "    -> Size: {} x {}\n", VIDEO_MEMORY_WIDTH, VIDEO_MEMORY_HEIGHT)?;

        for j in 0..VIDEO_MEMORY_HEIGHT {
            write!(f, "    ")?;

            for i in 0..VIDEO_MEMORY_WIDTH {
                let pixel = screen[i + j * VIDEO_MEMORY_WIDTH].read().expect("Could not read screen");
                write!(f, "{:02X} ", *pixel)?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}