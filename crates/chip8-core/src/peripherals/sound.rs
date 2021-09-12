//! CHIP-8 sound

use crate::drivers::AudioInterface;

/// Sound peripheral.
#[derive(Default)]
pub struct Sound;

impl Sound {
    /// New sound peripheral.
    pub fn new() -> Self {
        Default::default()
    }

    /// Play beep.
    pub fn play_beep(&self, driver: &mut dyn AudioInterface) {
        driver.play_beep()
    }
}
