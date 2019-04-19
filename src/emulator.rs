//! CHIP-8 emulator

use std::fs::{File, OpenOptions};
use std::io::Write;

use time::{self, PreciseTime};

use super::core::cpu::CPU;
use super::core::error::CResult;
use super::core::opcodes;
use super::core::savestate::{MissingSaveState, SaveState};
use super::peripherals::cartridge::Cartridge;

const TIMER_FRAME_LIMIT: i64 = 16;
const CPU_FRAME_LIMIT: i64 = 2;

/// CHIP-8 emulator
#[derive(Default)]
pub struct Emulator {
    /// CPU handle
    pub cpu: CPU,
}

/// Emulation state
#[derive(Debug)]
pub enum EmulationState {
    /// Quit
    Quit,
    /// Reset
    Reset,
    /// Normal
    Normal,
    /// Wait for input
    WaitForInput,
}

/// Emulator context
pub struct EmulatorContext {
    tracefile_handle: Option<File>,
    timer_frametime: PreciseTime,
    cpu_frametime: PreciseTime,
    frametime: PreciseTime,
}

impl Default for EmulatorContext {
    fn default() -> Self {
        Self {
            tracefile_handle: None,
            timer_frametime: time::PreciseTime::now(),
            cpu_frametime: time::PreciseTime::now(),
            frametime: time::PreciseTime::now(),
        }
    }
}

impl EmulatorContext {
    /// Create new emulator context
    pub fn new() -> Self {
        Default::default()
    }
}

impl Emulator {
    /// Create new CHIP-8 emulator
    pub fn new() -> Self {
        Emulator { cpu: CPU::new() }
    }

    /// Set CPU tracefile.
    ///
    /// # Arguments
    ///
    /// * `tracefile` - Tracefile
    ///
    pub fn set_tracefile(&mut self, tracefile: &str) {
        self.cpu.tracefile(tracefile);
    }

    /// Load game
    pub fn load_game(&mut self, cartridge: &Cartridge) {
        self.cpu.load_font_in_memory();
        self.cpu.load_cartridge_data(cartridge);
    }

    /// Save state
    pub fn save_state(&self, name: &str) {
        let savestate = SaveState::save_from_cpu(&self.cpu);
        savestate.write_to_file(&format!("{}.sav", name));
    }

    /// Load state
    pub fn load_state(&mut self, name: &str) -> CResult {
        let filename = format!("{}.sav", name);
        let savestate = SaveState::read_from_file(&filename);
        match savestate {
            None => Err(Box::new(MissingSaveState(filename.clone()))),
            Some(ss) => {
                self.cpu.load_savestate(ss);
                Ok(())
            }
        }
    }

    /// Reset
    pub fn reset(&mut self, cartridge: &Cartridge, ctx: &mut EmulatorContext) {
        // Reset CPU
        self.cpu.reset();

        // Reload data
        self.cpu.load_font_in_memory();
        self.cpu.load_cartridge_data(cartridge);

        // Reset vars
        ctx.timer_frametime = time::PreciseTime::now();
        ctx.cpu_frametime = time::PreciseTime::now();
        ctx.frametime = time::PreciseTime::now();
    }

    /// Step emulation
    pub fn step(&mut self, _cartridge: &Cartridge, ctx: &mut EmulatorContext) -> EmulationState {
        let cpu_framelimit = if self.cpu.schip_mode {
            CPU_FRAME_LIMIT / 2
        } else {
            CPU_FRAME_LIMIT
        };

        // Handle input lock
        if self.cpu.peripherals.input.is_locked() {
            if self.cpu.peripherals.input.is_lock_key_set() {
                let reg = self.cpu.peripherals.input.get_lock_register();
                let key = self.cpu.peripherals.input.get_lock_key();

                // Set register
                self.cpu.registers.set_register(reg, key);

                // Unlock
                self.cpu.peripherals.input.unlock();
            } else {
                // Wait for key
                return EmulationState::WaitForInput;
            }
        }

        if ctx
            .cpu_frametime
            .to(time::PreciseTime::now())
            .num_milliseconds()
            >= cpu_framelimit
        {
            // Read next instruction
            let opcode = self.cpu.peripherals.memory.read_opcode();
            trace_exec!(
                ctx.tracefile_handle,
                "[{:08X}] {:04X} - Reading opcode 0x{:04X}...",
                self.cpu.instruction_count,
                self.cpu.peripherals.memory.get_pointer(),
                opcode
            );

            // Trace
            let opcode_enum = opcodes::get_opcode_enum(opcode);
            let (assembly, verbose) = opcodes::get_opcode_str(&opcode_enum);
            trace_exec!(ctx.tracefile_handle, "  - {:20} ; {}", assembly, verbose);

            // Execute instruction
            if self.cpu.execute_instruction(&opcode_enum) {
                return EmulationState::Quit;
            }

            self.cpu.instruction_count += 1;

            ctx.cpu_frametime = time::PreciseTime::now();
        }

        if ctx
            .timer_frametime
            .to(time::PreciseTime::now())
            .num_milliseconds()
            >= TIMER_FRAME_LIMIT
        {
            // Handle timers
            self.cpu.decrement_timers();
            ctx.timer_frametime = time::PreciseTime::now();
        }

        ctx.frametime = time::PreciseTime::now();

        EmulationState::Normal
    }

    /// Run loop
    pub fn run_loop(&mut self, cartridge: &Cartridge) {
        let mut ctx = EmulatorContext::new();

        // Load game
        self.load_game(cartridge);

        // Get tracefile
        ctx.tracefile_handle = match self.cpu.tracefile {
            Some(ref path) => Some(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(path)
                    .unwrap(),
            ),
            None => None,
        };

        loop {
            match self.step(cartridge, &mut ctx) {
                EmulationState::Quit => break,
                EmulationState::Reset => continue,
                EmulationState::Normal => {}
                EmulationState::WaitForInput => {}
            }
        }
    }
}
