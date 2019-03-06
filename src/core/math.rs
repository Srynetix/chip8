//! Math utils

use num::Integer;

/// Absolute modulo operator
pub fn modulo<T: Integer + Copy>(n: T, modv: T) -> T {
    let r = n % modv;

    if r < T::zero() {
        r + modv
    } else {
        r
    }
}
