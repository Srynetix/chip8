//! Debugger stream

/// Debugger stream line
pub struct DebuggerStreamLine {
    pub error: bool,
    pub content: String,
}

/// Debugger stream
pub struct DebuggerStream {
    lines: Vec<DebuggerStreamLine>,
    use_console: bool,
}

impl Default for DebuggerStream {
    fn default() -> Self {
        Self {
            lines: vec![],
            use_console: false,
        }
    }
}

impl DebuggerStream {
    /// Create new stream
    pub fn new() -> Self {
        Default::default()
    }

    /// Use console
    pub fn set_use_console(&mut self, v: bool) {
        self.use_console = v;
    }

    /// Write to stdout
    pub fn writeln_stdout<T: AsRef<str>>(&mut self, s: T) {
        if self.use_console {
            println!("{}", s.as_ref());
        } else {
            self.lines.push(DebuggerStreamLine {
                error: false,
                content: s.as_ref().to_string(),
            });
        }
    }

    /// Write to stderr
    pub fn writeln_stderr<T: AsRef<str>>(&mut self, s: T) {
        if self.use_console {
            eprintln!("{}", s.as_ref());
        } else {
            self.lines.push(DebuggerStreamLine {
                error: true,
                content: s.as_ref().to_string(),
            });
        }
    }

    /// Get lines
    pub fn get_lines(&self) -> &[DebuggerStreamLine] {
        &self.lines
    }
}
