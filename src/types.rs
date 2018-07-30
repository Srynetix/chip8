//! CHIP-8 core types

/// `C8Byte`: CHIP-8 byte type
pub type C8Byte = u8;

/// `C8Addr`: CHIP-8 address type
pub type C8Addr = u16;

/// `C8RegIdx`: CHIP-8 register index
pub type C8RegIdx = u8;

/// Convert hex addr
/// 
/// # Arguments
/// 
/// * `s` - Input string
/// 
pub fn convert_hex_addr(s: &str) -> C8Addr {
    if &s[2..] == "0x" {
        C8Addr::from_str_radix(&s[2..], 16).unwrap()
    } else {
        C8Addr::from_str_radix(s, 16).unwrap()        
    }
}