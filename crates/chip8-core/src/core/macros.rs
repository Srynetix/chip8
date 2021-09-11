//! Macros.

/// Trace execution.
///
/// # Arguments
///
/// - `tracefile` - Tracefile handle.
/// - `format` - String format.
/// - `args` - Optional format args.
///
#[macro_export]
macro_rules! trace_exec {
    ($tracefile:expr, $format:expr, $($args:tt)*) => {
        if let Some(ref mut hndl) = $tracefile {
            match hndl {
                crate::emulator::TracefileHandle::Stdout => println!($format, $($args)*),
                crate::emulator::TracefileHandle::File(ref mut file) => writeln!(file, $format, $($args)*).unwrap()
            }
        }
    }
}
