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

/// Create rect.
///
/// # Arguments
///
/// - `x` - X value.
/// - `y` - Y value.
/// - `w` - W value.
/// - `h` - H value.
///
/// # Returns
///
/// * Rect.
///
#[macro_export]
macro_rules! rectf(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);
