//! CHIP-8 breakpoints

use std::fmt;

use super::types::C8Addr;

/// Breakpoints
#[derive(Default)]
pub struct Breakpoints(Vec<C8Addr>);

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
        if self.check_breakpoint(addr).is_none() {
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
        if let Some(idx) = self.check_breakpoint(addr) {
            debug!("Unregistering breakpoint at address {:04X}", addr);
            self.0.remove(idx);
        }
    }

    /// Check for breakpoint
    ///
    /// # Arguments
    ///
    /// * `addr` - Address
    ///
    pub fn check_breakpoint(&self, addr: C8Addr) -> Option<usize> {
        self.0.iter().position(|&x| x == addr)
    }

    /// Dump breakpoints
    pub fn dump_breakpoints(&self) {
        println!("{:?}", &self);
    }
}

impl fmt::Debug for Breakpoints {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Breakpoints: \n")?;
        if self.0.is_empty() {
            write!(f, "  None\n")?;
        } else {
            for i in &self.0 {
                write!(f, "  - {:04X}\n", i)?;
            }
        }

        Ok(())
    }
}
