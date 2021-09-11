//! CHIP-8 emulator.

use chip8::core::error::CResult;

fn main() -> CResult {
    chip8::start_shell()
}
