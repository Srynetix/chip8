//! Core types.

/// CHIP-8 byte type.
pub type C8Byte = u8;

/// CHIP-8 address type.
pub type C8Addr = u16;

/// CHIP-8 register index.
pub type C8RegIdx = u8;

/// Convert hexadecimal address.
///
/// # Arguments
///
/// * `s` - Input string.
///
/// # Returns
///
/// * Address option.
///
pub fn convert_hex_addr(s: &str) -> Option<C8Addr> {
    if s.len() >= 2 && &s[2..] == "0x" {
        _convert_hex_addr(&s[2..])
    } else {
        _convert_hex_addr(s)
    }
}

fn _convert_hex_addr(s: &str) -> Option<C8Addr> {
    C8Addr::from_str_radix(s, 16).ok()
}

/// Convert hexadecimal byte.
///
/// # Arguments
///
/// * `s` - Input string.
///
/// # Returns
///
/// * Byte option.
///
pub fn convert_hex_byte(s: &str) -> Option<C8Byte> {
    C8Byte::from_str_radix(s, 16).ok()
}
