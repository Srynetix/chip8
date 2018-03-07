//! CHIP-8 macros

/// Trace execution
/// 
/// # Arguments
/// 
/// - `tracefile` - Tracefile handle
/// - `format` - String format
/// - `args` - Optional format args
///
#[macro_export]
macro_rules! trace_exec {
    ($tracefile:expr, $format:expr, $($args:tt)*) => {        
        if let &mut Some(ref mut tf) = &mut $tracefile {
            writeln!(tf, $format, $($args)*).unwrap();
        }
    }
}