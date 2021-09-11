//! Logger.

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
    env_logger::init();
    Ok(())
}
