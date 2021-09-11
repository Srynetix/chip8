//! Peripherals.

pub mod cartridge;
pub mod input;
pub mod memory;
pub mod screen;

use self::input::InputState;
use self::memory::Memory;
use self::screen::Screen;

/// Peripherals.
pub struct Peripherals {
    /// Input.
    pub input: InputState,
    /// Memory.
    pub memory: Memory,
    /// Screen.
    pub screen: Screen,
}

impl Peripherals {
    /// Create new peripherals.
    ///
    /// # Returns
    ///
    /// * Peripherals instance.
    ///
    pub fn new() -> Self {
        Peripherals {
            input: InputState::new(),
            memory: Memory::new(),
            screen: Screen::new(),
        }
    }

    /// Reset peripherals.
    pub fn reset(&mut self) {
        self.memory.reset();
        self.input.reset();
        self.screen.reset();
    }
}

impl Default for Peripherals {
    fn default() -> Self {
        Self::new()
    }
}
