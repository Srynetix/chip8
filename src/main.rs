//! CHIP-8 emulator

extern crate chip8;

use std::env;
use std::path::Path;

use chip8::{DeviceBuilder, Cartridge};

fn main() {
    let cargo_path = match env::var("CARGO_MANIFEST_DIR") {
        Ok(path) => path,
        Err(_) => panic!("Environment var CARGO_MANIFEST_DIR is not set")
    };

    let cartridge_path = Path::new(&cargo_path).join("test.ch8");

    let mut device = DeviceBuilder::new()
                        .renderer(false)
                        .build();
                         
    let cartridge = Cartridge::load_from_path(cartridge_path.to_str().unwrap());

    device.debug_cpu();
    device.read_cartridge(&cartridge);
    device.debug_cpu();

    let (assembly, verbose) = cartridge.disassemble();
    for i in 0..assembly.len() {
        println!("{:30} ; {}", assembly[i], verbose[i]);
    }
}
