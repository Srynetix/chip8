//! CHIP-8 device board

use super::input::InputState;
use super::memory::Memory;
use super::screen::Screen;

use sdl2;

/// Peripherals
pub struct Peripherals {
    /// Input
    pub input: InputState,
    /// Memory
    pub memory: Memory,
    /// Screen
    pub screen: Screen
}

impl Peripherals {
    
    /// Create peripherals
    pub fn new() -> Self {
        // Create SDL context
        let context = sdl2::init().unwrap();

        Peripherals {
            input: InputState::new(&context),
            memory: Memory::new(),
            screen: Screen::new(&context)
        }
    }
}

