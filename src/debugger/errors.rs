//! Debugger errors.

use std::error::Error;
use std::fmt;

/// Breakpoint error.
#[derive(Debug)]
pub struct BadBreakpoint(pub String);

impl Error for BadBreakpoint {
    fn description(&self) -> &str {
        "bad breakpoint"
    }
}

impl fmt::Display for BadBreakpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bad breakpoint: {}", self.0)
    }
}
