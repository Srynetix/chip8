//! CHIP-8 breakpoints

use chip8_core::types::{C8Addr};

/// Breakpoints
pub struct Breakpoints(Vec<C8Addr>);

impl Breakpoints {
    
    /// Init
    pub fn new() -> Self {
        Breakpoints(Vec::new())
    }

    /// Register
    pub fn register(&mut self, addr: C8Addr) {
        self.0.push(addr);
    }

    /// Check for breakpoint
    pub fn check_breakpoint(&self, addr: C8Addr) -> Option<C8Addr> {
        self.0.iter().find(|&&x| x == addr).map(|x| x.clone())
    }
}