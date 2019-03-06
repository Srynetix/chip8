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
        if let Some(ref mut tf) = $tracefile {
            writeln!(tf, $format, $($args)*).unwrap();
        }
    }
}

/// Rectf
#[macro_export]
macro_rules! rectf(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);
