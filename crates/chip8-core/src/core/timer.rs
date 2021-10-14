//! CPU timer.

use std::fmt;

use nanoserde::{DeBin, SerBin};

use super::types::C8Byte;

/// CPU timer.
#[derive(Clone, SerBin, DeBin)]
pub struct Timer {
    title: String,
    count: C8Byte,
    will_finish: bool,
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
        Timer {
            title,
            count: 0,
            will_finish: false,
        }
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

            if self.count == 0 {
                self.will_finish = true;
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
        self.will_finish = false;
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

    /// Finished.
    pub fn finished(&mut self) -> bool {
        if self.will_finish {
            self.will_finish = false;
            true
        } else {
            false
        }
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
