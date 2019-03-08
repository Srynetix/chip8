//! CHIP-8 breakpoints

use std::fmt;

use crate::core::types::C8Addr;

/// Breakpoints
#[derive(Default)]
pub struct Breakpoints(pub Vec<C8Addr>);

impl Breakpoints {
    /// Init
    pub fn new() -> Self {
        Breakpoints(Vec::new())
    }

    /// Register
    ///
    /// # Arguments
    ///
    /// * `addr` - Address
    ///
    pub fn register(&mut self, addr: C8Addr) {
        if self.get_breakpoint(addr).is_none() {
            debug!("Registering breakpoint at address {:04X}", addr);
            self.0.push(addr);
        }
    }

    /// Unregister
    ///
    /// # Arguments
    ///
    /// * `addr` - Address
    ///
    pub fn unregister(&mut self, addr: C8Addr) {
        if let Some(idx) = self.get_breakpoint(addr) {
            debug!("Unregistering breakpoint at address {:04X}", addr);
            self.0.remove(idx);
        }
    }

    /// Get breakpoint for address.
    ///
    /// # Arguments
    ///
    /// * `addr` - Address
    ///
    pub fn get_breakpoint(&self, addr: C8Addr) -> Option<usize> {
        self.0.iter().position(|&x| x == addr)
    }

    /// Check breakpoint at address.
    pub fn check_breakpoint(&self, addr: C8Addr) -> bool {
        self.get_breakpoint(addr).is_some()
    }

    /// Dump breakpoints in console
    pub fn dump_breakpoints(&self) {
        println!("{:?}", &self);
    }
}

impl fmt::Debug for Breakpoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Breakpoints:")?;
        if self.0.is_empty() {
            writeln!(f, "  None")?;
        } else {
            for i in &self.0 {
                writeln!(f, "  - {:04X}", i)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoints() {
        let mut bps = Breakpoints::new();

        assert!(!bps.check_breakpoint(0x1234));

        bps.register(0x1234);
        bps.register(0x1234);
        assert!(bps.check_breakpoint(0x1234));

        bps.unregister(0x1234);
        assert!(!bps.check_breakpoint(0x1234));
        bps.unregister(0x1234);

        bps.dump_breakpoints();
    }
}
