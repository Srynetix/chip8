//! CPU timer.

use std::fmt;

use super::types::C8Byte;

/// CPU timer.
#[derive(Clone, Serialize, Deserialize)]
pub struct Timer {
    title: String,
    count: C8Byte,
}

impl Timer {
    /// Create new timer.
    ///
    /// # Arguments
    ///
    /// * `title` - Timer title.
    ///
    /// # Returns
    ///
    /// * Timer instance.
    ///
    pub fn new(title: String) -> Self {
        Timer { title, count: 0 }
    }

    /// Decrement timer.
    ///
    /// # Returns
    ///
    /// * Timer instance.
    ///
    pub fn decrement(&mut self) -> &Self {
        if self.count > 0 {
            self.count -= 1;

            // End
            if self.count == 0 {
                if self.title == "Sound" {
                    println!("** BEEP **");
                }
            }
        }

        self
    }

    /// Reset timer with value.
    ///
    /// # Arguments
    ///
    /// * `value` - Value.
    ///
    /// # Returns
    ///
    /// * Timer instance.
    ///
    pub fn reset(&mut self, value: C8Byte) -> &Self {
        self.count = value;
        self
    }

    /// Get value.
    ///
    /// # Returns
    ///
    /// * Value.
    ///
    pub fn get_value(&self) -> C8Byte {
        self.count
    }

    /// Load from save.
    ///
    /// # Arguments
    ///
    /// * `timer` - Timer.
    ///
    pub fn load_from_save(&mut self, timer: Timer) {
        self.count = timer.count;
        self.title = timer.title;
    }
}

impl fmt::Debug for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02X}", self.count)
    }
}
