//! Save state.

use std::{
    error::Error,
    fmt,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use nanoserde::{DeBin, SerBin};

use super::{cpu::CPU, registers::Registers, stack::Stack, timer::Timer};
use crate::peripherals::{input::InputState, memory::Memory, screen::ScreenData};

/// Missing save state.
#[derive(Debug)]
pub struct MissingSaveState(pub String);

impl Error for MissingSaveState {
    fn description(&self) -> &str {
        "missing save state"
    }
}

impl fmt::Display for MissingSaveState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "game save state is not found: {}", self.0)
    }
}

/// Save state.
#[derive(Clone, SerBin, DeBin, Debug)]
pub struct SaveState {
    /// Version.
    pub version: String,
    /// Input state.
    pub input: InputState,
    /// Memory.
    pub memory: Memory,
    /// Registers.
    pub registers: Registers,
    /// Screen data.
    pub screen_data: ScreenData,
    /// Stack.
    pub stack: Stack,
    /// Delay timer.
    pub delay_timer: Timer,
    /// Sound timer.
    pub sound_timer: Timer,
    /// Instruction count.
    pub instruction_count: usize,
}

impl SaveState {
    /// Create save state from CPU.
    ///
    /// # Arguments
    ///
    /// * `cpu` - CPU.
    ///
    /// # Returns
    ///
    /// * Save state instance.
    ///
    pub fn save_from_cpu(cpu: &CPU) -> SaveState {
        SaveState {
            version: env!("CARGO_PKG_VERSION").to_owned(),
            input: cpu.peripherals.input.clone(),
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
    /// * `path` - Path to file.
    ///
    pub fn write_to_file(&self, path: &str) {
        let state_bin = SerBin::serialize_bin(self);
        let mut file = File::create(path).expect("Could not create savestate file.");
        file.write_all(&state_bin)
            .expect("Error when writing savestate.");
    }

    /// Read save state from file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to file.
    ///
    /// # Returns
    ///
    /// * Save state option.
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

        Some(DeBin::deserialize_bin(&data).unwrap())
    }
}
