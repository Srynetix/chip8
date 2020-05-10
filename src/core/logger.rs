//! Logger.

use std::io;

/// Initialize logger.
///
/// # Arguments
///
/// * `level` - Log level.
///
/// # Returns
///
/// * Logger result.
///
pub fn init_logger(level: log::LevelFilter) -> Result<(), log::SetLoggerError> {
    fern::Dispatch::new()
        // Perform allocation-free log formatting.
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        // Add blanket level filter.
        .level(level)
        // Output to stdout.
        .chain(io::stdout())
        // Apply globally.
        .apply()
}
