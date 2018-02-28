//! CHIP-8 device

use std::{thread};
use std::sync::{Arc, RwLock};

use chip8_cpu::{CPU, Cartridge};
use chip8_graphics::Renderer;

/// CHIP-8 device struct
pub struct Device {
    cpu: CPU
}

impl Device {

    /// Create device
    pub fn new() -> Self {
        let mut cpu = CPU::new();
        cpu.load_font_in_memory();

        Device {
            cpu: cpu 
        }
    }

    /// Show CPU debug
    pub fn debug_cpu(&self) {
        println!("> CPU");
        println!("{:?}", self.cpu);
    }

    /// Run device
    /// 
    /// # Arguments
    /// 
    /// * `verbose` - Verbose mode
    /// 
    pub fn run(self) {
        let mut renderer = Renderer::new("CHIP-8".to_string());

        println!("> Starting device...");

        let mut cpu = self.cpu;
        
        let running = Arc::new(RwLock::new(true));
        let screen_handle = Arc::clone(&cpu.get_video_memory());
        let running_handle = Arc::clone(&running);

        let handle = thread::spawn(move || {
            while *(running_handle.read().unwrap()) {
                cpu.read_next_instruction();
                cpu.decrement_timers();
            }
        });

        println!("> Starting renderer...");
        renderer.run(screen_handle);
        println!("> Stopping renderer...");   

        *(running.write().unwrap()) = false;

        handle.join().unwrap();

        println!("> Stopping CPU...");        
    }

    /// Read CHIP-8 cartridge
    pub fn read_cartridge(&mut self, cartridge: &Cartridge) {
        println!("> Reading cartridge...");

        self.cpu.load_cartridge_data(cartridge.get_data());
    }
}