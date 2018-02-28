//! CHIP-8 core types

use std::sync::{RwLock, Arc};

/// C8Byte: CHIP-8 byte type
pub type C8Byte = u8;

/// C8Short: CHIP-8 short type
pub type C8Short = u16;

/// C8Addr: CHIP-8 address type
pub type C8Addr = u16;

/// C8RegIdx: CHIP-8 register index
pub type C8RegIdx = u8;

/// C8ByteVec: CHIP-8 byte vec
pub type C8ByteVec = Vec<C8Byte>;

/// C8AddrVec: CHIP-8 address vec
pub type C8AddrVec = Vec<C8Addr>;

/// SharedC8ByteVec: Shared CHIP-8 byte vec
pub type SharedC8ByteVec = Arc<RwLock<C8ByteVec>>;