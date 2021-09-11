//! Debugger stream.

/// Debugger stream line.
pub struct DebuggerStreamLine {
    pub error: bool,
    pub content: String,
}

/// Debugger stream.
pub struct DebuggerStream {
    lines: Vec<DebuggerStreamLine>,
    console: bool,
}

impl Default for DebuggerStream {
    fn default() -> Self {
        Self {
            lines: vec![],
            console: false,
        }
    }
}

impl DebuggerStream {
    /// Create new stream.
    ///
    /// # Returns
    ///
    /// * Debugger stream.
    ///
    pub fn new() -> Self {
        Default::default()
    }

    /// Use console.
    ///
    /// # Arguments
    ///
    /// * `v` - Value.
    ///
    pub fn use_console(&mut self, v: bool) {
        self.console = v;
    }

    /// Write to stdout.
    ///
    /// # Arguments
    ///
    /// * `s` - String line.
    ///
    pub fn writeln_stdout<T: AsRef<str>>(&mut self, s: T) {
        if self.console {
            println!("{}", s.as_ref());
        } else {
            self.lines.push(DebuggerStreamLine {
                error: false,
                content: s.as_ref().to_string(),
            });
        }
    }

    /// Write to stderr.
    ///
    /// # Arguments
    ///
    /// * `s` - String line.
    ///
    pub fn writeln_stderr<T: AsRef<str>>(&mut self, s: T) {
        if self.console {
            eprintln!("{}", s.as_ref());
        } else {
            self.lines.push(DebuggerStreamLine {
                error: true,
                content: s.as_ref().to_string(),
            });
        }
    }

    /// Get lines.
    ///
    /// # Returns
    ///
    /// * Debugger lines.
    ///
    pub fn get_lines(&self) -> &[DebuggerStreamLine] {
        &self.lines
    }
}
