//! CHIP-8 emulator

extern crate chip8;

use chip8::{DeviceBuilder, Cartridge};

fn main() {

    let mut device = DeviceBuilder::new()
                        .renderer(false)
                        .build();
                         
    let cartridge = Cartridge::load_from_games_directory("test.ch8");

    device.read_cartridge(&cartridge);
    device.start();
    device.debug_cpu();

    cartridge.print_disassembly();
}
