//! CHIP-8 device

use chip8_core::{CPU, Cartridge};
use chip8_graphics::Renderer;

/// CHIP-8 device struct
pub struct Device {
    cpu: CPU,
    renderer: Option<Renderer>
}

/// CHIP-8 device builder
pub struct DeviceBuilder {
    show_renderer: bool
}

impl DeviceBuilder {

    /// Create a DeviceBuilder
    pub fn new() -> DeviceBuilder {
        DeviceBuilder {
            show_renderer: true
        }
    }

    /// Enable/Disable renderer
    pub fn renderer(&mut self, value: bool) -> &Self {
        self.show_renderer = value;
        self
    }

    /// Build CHIP-8 device
    pub fn build(&self) -> Device {
        Device {
            cpu: CPU::new(),
            renderer: if self.show_renderer { Some(Renderer::new()) } else { None }
        }
    }
}

impl Device {

    /// Show CPU debug
    pub fn debug_cpu(&self) {
        println!("> CPU");
        println!("{:?}", self.cpu);

        println!("> Is renderer active: {}", self.renderer.is_some());
    }

    /// Start device
    pub fn start(&mut self) {
        println!("> Starting device...");

        self.cpu.load_font_in_memory();
        self.cpu.run();
    }

    /// Read CHIP-8 cartridge
    pub fn read_cartridge(&mut self, cartridge: &Cartridge) {
        println!("> Reading cartridge...");

        self.cpu.load_cartridge_data(cartridge.get_data());
    }
}