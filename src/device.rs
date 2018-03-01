//! CHIP-8 device

use std::{thread};
use std::sync::{Arc, RwLock};

use time;

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
    pub fn run(self) {
        let mut renderer = Renderer::new();

        println!("> Starting device...");

        let mut cpu = self.cpu;
        
        let running = Arc::new(RwLock::new(true));
        let screen_handle = Arc::clone(&cpu.get_video_memory());
        let input_handle = Arc::clone(&cpu.get_input_state());
        let running_handle = Arc::clone(&running);

        let handle = thread::spawn(move || {
            let mut start_time = time::PreciseTime::now();

            while *(running_handle.read().unwrap()) {
                cpu.read_next_instruction();

                // Decrement timers 60Hz
                let current_time = time::PreciseTime::now();
                if start_time.to(current_time).num_milliseconds() > 16 {
                    cpu.decrement_timers();
                    start_time = current_time;
                }
            }
        });

        println!("> Starting renderer...");
        renderer.run(input_handle, screen_handle);
        println!("> Stopping renderer...");   

        *(running.write().unwrap()) = false;

        handle.join().unwrap();

        println!("> Stopping CPU...");        
    }

    /// Read CHIP-8 cartridge
    /// 
    /// # Arguments
    /// 
    /// * `cartridge` - Cartridge to load
    /// 
    pub fn read_cartridge(&mut self, cartridge: &Cartridge) {
        println!("> Reading cartridge...");

        self.cpu.load_cartridge_data(cartridge);
    }
}