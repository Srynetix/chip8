//! CHIP-8 save state

use std::io::{Read, Write};

use std::fs::File;
use std::path::Path;

use bincode::{deserialize, serialize};

use crate::core::cpu::CPU;
use crate::core::registers::Registers;
use crate::core::stack::Stack;
use crate::core::timer::Timer;
use crate::peripherals::input::InputStateData;
use crate::peripherals::memory::Memory;
use crate::peripherals::screen::ScreenData;

/// CHIP-8 save state
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SaveState {
    /// Input state data
    pub input_data: InputStateData,
    /// Memory
    pub memory: Memory,
    /// Registers
    pub registers: Registers,
    /// Screen data
    pub screen_data: ScreenData,
    /// Stack
    pub stack: Stack,
    /// Delay timer
    pub delay_timer: Timer,
    /// Sound timer
    pub sound_timer: Timer,
    /// Instruction count
    pub instruction_count: usize,
}

impl SaveState {
    /// Create save state from CPU
    ///
    /// # Arguments
    ///
    /// * `cpu` - CPU
    ///
    pub fn save_from_cpu(cpu: &CPU) -> SaveState {
        SaveState {
            input_data: cpu.peripherals.input.data.clone(),
            memory: cpu.peripherals.memory.clone(),
            registers: cpu.registers.clone(),
            screen_data: cpu.peripherals.screen.data.clone(),
            stack: cpu.stack.clone(),
            delay_timer: cpu.delay_timer.clone(),
            sound_timer: cpu.sound_timer.clone(),
            instruction_count: cpu.instruction_count,
        }
    }

    /// Write save state to file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to file
    ///
    pub fn write_to_file(&self, path: &str) {
        let state_bin = serialize(&self).unwrap();
        let mut file = File::create(path).expect("Could not create savestate file.");
        file.write_all(&state_bin)
            .expect("Error when writing savestate.");
    }

    /// Read save state from file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to file
    ///
    pub fn read_from_file(path: &str) -> Option<SaveState> {
        let path_p = Path::new(path);
        if !path_p.exists() {
            return None;
        }

        let mut file = File::open(path).expect("Could not read savestate file.");
        let mut data = Vec::new();
        file.read_to_end(&mut data)
            .expect("Error when reading savestate.");

        Some(deserialize(&data).unwrap())
    }
}
