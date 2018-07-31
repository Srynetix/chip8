//! Emulator

use std::fs::OpenOptions;
use std::io::Write;
use std::process;
use std::rc::Rc;
use time;

use super::cartridge::Cartridge;
use super::cpu::CPU;
use super::debugger::{Command, Debugger};
use super::opcodes;
use super::savestate::SaveState;
use super::types::C8Addr;

const TIMER_FRAME_LIMIT: i64 = 16;
const CPU_FRAME_LIMIT: i64 = 2;
const SCREEN_FRAME_LIMIT: i64 = 16;

pub struct Emulator {
    pub cpu: Rc<CPU>,
}

impl Emulator {
    /// Create new CHIP-8 emulator
    pub fn new() -> Self {
        let cpu = Rc::new(CPU::new());

        Emulator { cpu }
    }

    /// Run emulator with a cartridge.
    ///
    /// # Arguments
    ///
    /// * `cartridge` - Cartridge
    ///
    pub fn run(&mut self, cartridge: &Cartridge) {
        let game_name: String = cartridge.get_title().to_string();
        let cloned_cpu = self.cpu.clone();
        let cpu = Rc::get_mut(&mut self.cpu).unwrap();

        cpu.load_font_in_memory();
        cpu.load_cartridge_data(cartridge);

        let mut last_debugger_command: Option<Command> = None;
        let mut break_next_instruction: Option<C8Addr> = None;
        let mut tracefile_handle = match cpu.tracefile {
            Some(ref path) => Some(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(path)
                    .unwrap(),
            ),
            None => None,
        };

        let mut timer_frametime = time::PreciseTime::now();
        let mut cpu_frametime = time::PreciseTime::now();
        let mut frametime = time::PreciseTime::now();

        loop {
            let cpu_framelimit = if cpu.schip_mode {
                CPU_FRAME_LIMIT / 2
            } else {
                CPU_FRAME_LIMIT
            };

            // Check if CPU should stop
            if cpu.peripherals.input.data.flags.should_close {
                break;
            }

            // Check if CPU should reset
            if cpu.peripherals.input.data.flags.should_reset {
                // Reset CPU
                cpu.reset();

                // Reload data
                cpu.load_font_in_memory();
                cpu.load_cartridge_data(cartridge);

                // Reset vars
                last_debugger_command = None;
                break_next_instruction = None;
                timer_frametime = time::PreciseTime::now();
                cpu_frametime = time::PreciseTime::now();
                frametime = time::PreciseTime::now();

                // Restart !
                continue;
            }

            // Check if CPU should save
            if cpu.peripherals.input.data.flags.should_save {
                cpu.peripherals.input.data.flags.should_save = false;

                println!("Saving state...");
                let savestate = SaveState::save_from_cpu(&cpu);
                savestate.write_to_file(&format!("{}.sav", game_name));

                // self.savestate = Some(;
                println!("State saved.");
            }

            if cpu.peripherals.input.data.flags.should_load {
                cpu.peripherals.input.data.flags.should_load = false;

                println!("Loading state...");
                let savestate = SaveState::read_from_file(&format!("{}.sav", game_name));
                match savestate {
                    None => println!("No state found."),
                    Some(ss) => cpu.load_savestate(ss),
                }
            }

            if cpu_frametime
                .to(time::PreciseTime::now())
                .num_milliseconds()
                >= cpu_framelimit
            {
                // Read next instruction
                let opcode = cpu.peripherals.memory.read_opcode();
                trace_exec!(
                    tracefile_handle,
                    "[{:08X}] {:04X} - Reading opcode 0x{:04X}...",
                    cpu.instruction_count,
                    cpu.peripherals.memory.get_pointer(),
                    opcode
                );

                // Check for breakpoints
                if break_next_instruction.is_none() {
                    let pointer = cpu.peripherals.memory.get_pointer();
                    if cpu.breakpoints.check_breakpoint(pointer).is_some() {
                        break_next_instruction = Some(pointer);
                    }
                }

                // Break ?
                if let Some(addr) = break_next_instruction {
                    trace_exec!(tracefile_handle, "{:?}", &cpu);

                    'debugger: loop {
                        {
                            let debugger = Debugger::new(&cloned_cpu, addr);
                            last_debugger_command = debugger.run();
                        }

                        if let Some(ref command) = last_debugger_command {
                            match *command {
                                Command::Quit => process::exit(1),
                                Command::AddBreakpoint(addr) => {
                                    println!("Adding breakpoint for address 0x{:04X}", addr);
                                    cpu.breakpoints.register(addr);
                                }
                                Command::RemoveBreakpoint(addr) => {
                                    println!("Removing breakpoint for address 0x{:04X}", addr);
                                    cpu.breakpoints.unregister(addr);
                                }
                                _ => break 'debugger,
                            }
                        }
                    }
                }

                // Trace
                let opcode_enum = opcodes::get_opcode_enum(opcode);
                let (assembly, verbose) = opcodes::get_opcode_str(&opcode_enum);
                trace_exec!(tracefile_handle, "  - {:20} ; {}", assembly, verbose);

                // Update state
                cpu.peripherals.input.update_state();

                // Execute instruction
                if cpu.execute_instruction(&opcode_enum) {
                    break;
                }

                // Handle last debugger command
                if let Some(ref command) = last_debugger_command {
                    match *command {
                        Command::Continue => {
                            break_next_instruction = None;
                        }
                        Command::Next => {
                            break_next_instruction = Some(cpu.peripherals.memory.get_pointer());
                        }
                        _ => {}
                    }
                }

                cpu_frametime = time::PreciseTime::now();
            }

            if timer_frametime
                .to(time::PreciseTime::now())
                .num_milliseconds()
                >= TIMER_FRAME_LIMIT
            {
                // Handle timers
                cpu.decrement_timers();
                timer_frametime = time::PreciseTime::now();
            }

            if frametime.to(time::PreciseTime::now()).num_milliseconds() >= SCREEN_FRAME_LIMIT {
                // Update screen
                cpu.peripherals.screen.fade_pixels();
                cpu.peripherals.screen.render();
                frametime = time::PreciseTime::now();

                cpu.instruction_count += 1;
            }
        }
    }
}
