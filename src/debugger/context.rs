//! Debugger context

use rustyline::Editor;

use crate::core::error::CResult;
use crate::core::types::{convert_hex_addr, C8Addr};
use crate::debugger::Breakpoints;

use super::errors::BadBreakpoint;

pub enum DebuggerMode {
    /// Interactive
    Interactive,
    /// Manual
    Manual,
}

/// Debugger context
pub struct DebuggerContext {
    /// Running
    pub running: bool,
    /// Address
    pub address: C8Addr,
    /// Is stepping
    pub is_stepping: bool,
    /// Is continuing
    pub is_continuing: bool,
    /// Has just hit breakpoint
    pub breakpoint_hit: bool,
    /// Has moved
    pub has_moved: bool,
    /// Should quit
    pub should_quit: bool,
    /// Editor
    pub editor: Editor<()>,
    /// Mode
    pub mode: DebuggerMode,
    /// Breakpoints
    pub breakpoints: Breakpoints,
}

impl Default for DebuggerContext {
    fn default() -> Self {
        Self {
            address: 0,
            running: true,
            is_stepping: false,
            is_continuing: false,
            breakpoint_hit: false,
            has_moved: false,
            should_quit: false,
            editor: Editor::<()>::new(),
            mode: DebuggerMode::Interactive,
            breakpoints: Breakpoints::new(),
        }
    }
}

impl DebuggerContext {
    /// Create new context
    pub fn new() -> Self {
        Default::default()
    }

    /// Set debugger address
    pub fn set_address(&mut self, addr: C8Addr) {
        self.address = addr;
    }

    /// Pause
    pub fn pause(&mut self) {
        self.is_continuing = false;
        self.is_stepping = false;
    }

    /// Is paused?
    pub fn is_paused(&self) -> bool {
        !self.is_continuing
    }

    /// Set manual mode
    pub fn set_manual(&mut self) {
        self.mode = DebuggerMode::Manual;
    }

    /// Set interactive mode
    pub fn set_interactive(&mut self) {
        self.mode = DebuggerMode::Interactive;
    }

    /// Register breakpoint
    pub fn register_breakpoint(&mut self, addr: C8Addr) {
        self.breakpoints.register(addr);
    }

    /// Unregister breakpoint
    pub fn unregister_breakpoint(&mut self, addr: C8Addr) {
        self.breakpoints.unregister(addr);
    }

    /// Register breakpoint as string
    pub fn register_breakpoint_str(&mut self, addr: &str) -> CResult {
        if let Some(addr) = convert_hex_addr(addr) {
            self.breakpoints.register(addr);
            Ok(())
        } else {
            Err(Box::new(BadBreakpoint(String::from(addr))))
        }
    }
}
