//! CHIP-8 breakpoints.

use std::fmt;

use tracing::info;

use crate::core::types::C8Addr;

/// Breakpoints.
#[derive(Default)]
pub struct Breakpoints(pub Vec<C8Addr>);

impl Breakpoints {
    /// Create breakpoints.
    ///
    /// # Returns
    ///
    /// * Breakpoints instance.
    ///
    pub fn new() -> Self {
        Breakpoints(Vec::new())
    }

    /// Register breakpoint.
    ///
    /// # Arguments
    ///
    /// * `addr` - Address.
    ///
    pub fn register(&mut self, addr: C8Addr) {
        if self.get_breakpoint(addr).is_none() {
            info!("registering breakpoint at address {:04X}", addr);
            self.0.push(addr);
        }
    }

    /// Unregister breakpoint.
    ///
    /// # Arguments
    ///
    /// * `addr` - Address.
    ///
    pub fn unregister(&mut self, addr: C8Addr) {
        if let Some(idx) = self.get_breakpoint(addr) {
            info!("unregistering breakpoint at address {:04X}", addr);
            self.0.remove(idx);
        }
    }

    /// Get breakpoint for address.
    ///
    /// # Arguments
    ///
    /// * `addr` - Address.
    ///
    /// # Returns
    ///
    /// * Breakpoint option.
    ///
    pub fn get_breakpoint(&self, addr: C8Addr) -> Option<usize> {
        self.0.iter().position(|&x| x == addr)
    }

    /// Check breakpoint at address.
    ///
    /// # Arguments
    ///
    /// * `addr` - Address.
    ///
    /// # Returns
    ///
    /// * `true` if breakpoint exists.
    /// * `false` if not.
    ///
    pub fn check_breakpoint(&self, addr: C8Addr) -> bool {
        self.get_breakpoint(addr).is_some()
    }

    /// Dump breakpoints in console.
    pub fn dump_breakpoints(&self) {
        println!("{:?}", &self);
    }
}

impl fmt::Debug for Breakpoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "breakpoints:")?;
        if self.0.is_empty() {
            writeln!(f, "  none")?;
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
