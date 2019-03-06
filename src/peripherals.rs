//! CHIP-8 device board

use super::input::InputState;
use super::memory::Memory;
use super::screen::Screen;

// use sdl2;

/// Peripherals
pub struct Peripherals {
    /// Input
    pub input: InputState,
    /// Memory
    pub memory: Memory,
    /// Screen
    pub screen: Screen,
}

impl Peripherals {
    /// Create peripherals
    pub fn new() -> Self {
        Peripherals {
            input: InputState::new(),
            memory: Memory::new(),
            screen: Screen::new(),
        }
    }
}

impl Default for Peripherals {
    fn default() -> Self {
        Self::new()
    }
}
