//! CHIP-8 logger

use std::io;
use std::fs::File;
use std::io::Write;

use fern;
use log;
use chrono;

/// Initialize logger
///
/// # Arguments
///
/// - `level` - Log level
///
pub fn init_logger(level: log::LogLevelFilter) -> Result<(), log::SetLoggerError> {
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!("{}[{}][{}] {}",
                chrono::Local::now()
                    .format("[%H:%M:%S]"),
                record.target(),
                record.level(),
                message))
        })
        // Add blanket level filter -
        .level(level)
        // Output to stdout
        .chain(io::stdout())
        // Apply globally
        .apply()
}

/// Trace execution
pub fn trace_exec(tracefile_handle: &mut Option<File>, line: &str) {
    if let &mut Some(ref mut tracefile) = tracefile_handle {
        writeln!(tracefile, "{}", line).unwrap();
    }
}