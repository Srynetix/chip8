use chip8_core::{drivers::WindowInterface, errors::CResult};
use chip8_drivers::WinitWindowDriver;

fn main() -> CResult {
    let mut driver = WinitWindowDriver::new();
    driver.run_gui()
}
