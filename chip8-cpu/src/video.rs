//! CHIP-8 video memory
//! 
//! 1: On
//! 0: Off

use std::fmt;
use std::sync::{RwLock};

use chip8_core::types::{C8Byte};

/// Video memory width
pub const VIDEO_MEMORY_WIDTH: usize = 64;
/// Video memory height
pub const VIDEO_MEMORY_HEIGHT: usize = 32;

const VIDEO_MEMORY_SIZE: usize = VIDEO_MEMORY_WIDTH * VIDEO_MEMORY_HEIGHT;

/// CHIP-8 video memory struct
pub struct VideoMemory(Vec<RwLock<C8Byte>>);

impl VideoMemory {

    /// Create new video memory
    pub fn new() -> Self {
        let mut vec = Vec::with_capacity(VIDEO_MEMORY_SIZE);
        for _ in 0..VIDEO_MEMORY_SIZE {
            vec.push(RwLock::new(0));
        }

        VideoMemory(vec)
    }

    /// Clear screen
    pub fn clear_screen(&self) {
        for x in 0..self.0.len() {
            let mut pixel = self.0[x].write().expect("Could not write to screen");
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
    pub fn toggle_pixel(&self, pos: usize) -> bool {
        // For now, only handle 0 and 1
        let mut flip = false;
        let mut pixel = self.0[pos].write().expect("Could not write to screen");        
        
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
    pub fn toggle_pixel_xy(&self, x: usize, y: usize) -> bool {
        self.toggle_pixel(x + y * VIDEO_MEMORY_WIDTH)
    }

    /// Get raw data
    pub fn get_raw_data(&self) -> &Vec<RwLock<C8Byte>> {
        &self.0
    }
}

impl fmt::Debug for VideoMemory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "    -> Size: {} x {}\n", VIDEO_MEMORY_WIDTH, VIDEO_MEMORY_HEIGHT)?;

        for j in 0..VIDEO_MEMORY_HEIGHT {
            write!(f, "    ")?;

            for i in 0..VIDEO_MEMORY_WIDTH {
                let pixel = self.0[i + j * VIDEO_MEMORY_WIDTH].read().expect("Could not read screen");
                write!(f, "{:02X} ", *pixel)?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}