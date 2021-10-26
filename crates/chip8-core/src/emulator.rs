//! CHIP-8 emulator.

use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use tracing::{error, info};

use super::{
    core::{
        cpu::CPU,
        opcodes,
        savestate::{MissingSaveState, SaveState},
    },
    errors::CResult,
    peripherals::cartridge::Cartridge,
    trace_exec,
};

const TIMER_FRAME_LIMIT: u64 = 16;
const CPU_FRAME_LIMIT: u64 = 0;

/// CHIP-8 emulator.
#[derive(Default)]
pub struct Emulator {
    /// CPU handle.
    pub cpu: CPU,
}

/// Emulation state.
#[derive(Debug)]
pub enum EmulationState {
    /// Quit.
    Quit,
    /// Reset.
    Reset,
    /// Normal.
    Normal,
    /// Wait for input.
    WaitForInput,
    /// Wait for delay.
    WaitForDelay,
}

/// Tracefile handle.
#[derive(Debug)]
pub enum TracefileHandle {
    /// File
    File(File),
    /// Stdout
    Stdout,
}

/// Emulator context.
#[derive(Default)]
pub struct EmulatorContext {
    tracefile_handle: Option<TracefileHandle>,
    timer_frametime: u64,
    cpu_frametime: u64,
}

impl EmulatorContext {
    /// Create new emulator context.
    ///
    /// # Returns
    ///
    /// * Emulator context.
    ///
    pub fn new() -> Self {
        Default::default()
    }

    /// Prepare tracefile
    ///
    /// # Arguments
    ///
    /// * `tracefile` - Tracefile
    ///
    pub fn prepare_tracefile(&mut self, tracefile: &Option<String>) {
        self.tracefile_handle = match tracefile {
            Some(ref path) => {
                if path == "-" {
                    info!(message = "Tracefile defined.", output = "stdout");
                    Some(TracefileHandle::Stdout)
                } else {
                    info!(
                        message = "Tracefile defined.",
                        output = %path
                    );
                    Some(TracefileHandle::File(
                        OpenOptions::new()
                            .write(true)
                            .create(true)
                            .open(path)
                            .unwrap(),
                    ))
                }
            }
            None => None,
        };
    }
}

impl Emulator {
    /// Create new CHIP-8 emulator.
    ///
    /// # Returns
    ///
    /// * Emulator instance.
    ///
    pub fn new() -> Self {
        Default::default()
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

    /// Load game.
    ///
    /// # Arguments
    ///
    /// * `cartridge` - Cartridge.
    ///
    pub fn load_game(&mut self, cartridge: &Cartridge) {
        self.cpu.load_font_in_memory();
        self.cpu.load_cartridge_data(cartridge);
    }

    /// Save state.
    ///
    /// # Arguments
    ///
    /// * `name` - Game name.
    ///
    pub fn save_state(&self, name: &str) {
        let savestate = SaveState::save_from_cpu(&self.cpu);
        let path = format!("{}.sav", name);
        savestate.write_to_file(&path);

        info!(
            message = "Game state saved.",
            title = %name,
            path = %path,
        );
    }

    /// Load state.
    ///
    /// # Arguments
    ///
    /// * `name` - Game name.
    ///
    /// # Returns
    ///
    /// * Result.
    ///
    pub fn load_state(&mut self, name: &str) -> CResult {
        let filename = format!("{}.sav", name);
        let savestate = SaveState::read_from_file(&filename);
        match savestate {
            None => {
                error!(
                    message = "Game state loading error.",
                    title = %name,
                    path = %filename,
                );
                Err(Box::new(MissingSaveState(filename)))
            }
            Some(ss) => {
                self.cpu.load_savestate(ss);
                info!(
                    message = "Game state loaded.",
                    title = %name,
                    path = %filename,
                );
                Ok(())
            }
        }
    }

    /// Reset.
    ///
    /// # Arguments
    ///
    /// * `cartridge` - Cartridge.
    /// * `ctx` - Emulator context.
    ///
    pub fn reset(&mut self, cartridge: &Cartridge, ctx: &mut EmulatorContext) {
        // Reset CPU.
        self.cpu.reset();

        // Reload data.
        self.cpu.load_font_in_memory();
        self.cpu.load_cartridge_data(cartridge);

        // Reset vars.
        ctx.timer_frametime = 0;
        ctx.cpu_frametime = 0;

        info!(message = "Emulator reset.")
    }

    /// Step emulation.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Emulator context.
    ///
    /// # Returns
    ///
    /// * Emulation state.
    ///
    pub fn step(&mut self, ctx: &mut EmulatorContext) -> EmulationState {
        let cpu_framelimit = if self.cpu.schip_mode {
            CPU_FRAME_LIMIT / 2
        } else {
            CPU_FRAME_LIMIT
        };

        // Handle input lock.
        if self.cpu.peripherals.input.is_locked() {
            if self.cpu.peripherals.input.is_lock_key_set() {
                let reg = self.cpu.peripherals.input.get_lock_register();
                let key = self.cpu.peripherals.input.get_lock_key();

                // Set register.
                self.cpu.registers.set_register(reg, key);

                // Unlock.
                self.cpu.peripherals.input.unlock();
            } else {
                // Wait for key.
                return EmulationState::WaitForInput;
            }
        }

        // Handle scroll.
        if self.cpu.peripherals.screen.is_scrolling() {
            self.cpu.peripherals.screen.apply_scroll();
        }

        if ctx.cpu_frametime >= cpu_framelimit {
            // Read next instruction.
            let opcode = self.cpu.peripherals.memory.read_opcode();
            trace_exec!(
                ctx.tracefile_handle,
                "[{:08X}] {:04X} - reading opcode 0x{:04X}...",
                self.cpu.instruction_count,
                self.cpu.peripherals.memory.get_pointer(),
                opcode
            );

            // Trace.
            let opcode_enum = opcodes::get_opcode_enum(opcode);
            let (assembly, verbose) = opcodes::get_opcode_str(&opcode_enum);
            trace_exec!(ctx.tracefile_handle, "  - {:20} ; {}", assembly, verbose);

            // Execute instruction.
            if self.cpu.execute_instruction(&opcode_enum) {
                return EmulationState::Quit;
            }

            self.cpu.instruction_count += 1;

            ctx.cpu_frametime = 0;
        } else {
            ctx.cpu_frametime += 1;
        }

        if ctx.timer_frametime >= TIMER_FRAME_LIMIT {
            // Handle timers.
            self.cpu.decrement_timers();
            ctx.timer_frametime = 0;
        } else {
            ctx.timer_frametime += 1;
        }

        EmulationState::Normal
    }
}
