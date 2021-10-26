//! Math utils.

use num_integer::Integer;

/// Absolute modulo operator.
///
/// # Arguments
///
/// * `n` - Number.
/// * `modv` - Modulo value.
///
/// # Returns
///
/// * Modulo value.
///
pub fn modulo<T: Integer + Copy>(n: T, modv: T) -> T {
    let r = n % modv;

    if r < T::zero() {
        r + modv
    } else {
        r
    }
}
