//! CHIP-8 emulator

use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::process;
use std::rc::Rc;

use time::{self, PreciseTime};

use super::core::cpu::CPU;
use super::core::opcodes;
use super::core::types::{convert_hex_addr, C8Addr};
use super::debugger::{Command, Debugger, SaveState};
use super::peripherals::cartridge::Cartridge;

const TIMER_FRAME_LIMIT: i64 = 16;
const CPU_FRAME_LIMIT: i64 = 2;
// const SCREEN_FRAME_LIMIT: i64 = 16;

/// CHIP-8 emulator
#[derive(Default)]
pub struct Emulator {
    /// CPU handle
    pub cpu: Rc<RefCell<CPU>>,
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
}

/// Emulator context
pub struct EmulatorContext {
    last_debugger_command: Option<Command>,
    break_next_instruction: Option<C8Addr>,
    tracefile_handle: Option<File>,
    timer_frametime: PreciseTime,
    cpu_frametime: PreciseTime,
    frametime: PreciseTime,
}

impl Default for EmulatorContext {
    fn default() -> Self {
        Self {
            last_debugger_command: None,
            break_next_instruction: None,
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
        let cpu = Rc::new(RefCell::new(CPU::new()));

        Emulator { cpu }
    }

    /// Register breakpoint.
    ///
    /// # Arguments
    ///
    /// * `address` - Address
    ///
    pub fn register_breakpoint(&self, addr: &str) {
        if let Some(addr) = convert_hex_addr(addr) {
            self.cpu.borrow_mut().breakpoints.register(addr);
        } else {
            println!("error while registering breakpoint: bad address {}", addr);
        }
    }

    /// Set CPU tracefile.
    ///
    /// # Arguments
    ///
    /// * `tracefile` - Tracefile
    ///
    pub fn set_tracefile(&self, tracefile: &str) {
        self.cpu.borrow_mut().tracefile(tracefile);
    }

    /// Load game
    pub fn load_game(&self, cartridge: &Cartridge) {
        self.cpu.borrow_mut().load_font_in_memory();
        self.cpu.borrow_mut().load_cartridge_data(cartridge);
    }

    /// Step emulation
    pub fn step(&self, cartridge: &Cartridge, ctx: &mut EmulatorContext) -> EmulationState {
        let game_name = cartridge.get_title();

        let cpu_framelimit = if self.cpu.borrow().schip_mode {
            CPU_FRAME_LIMIT / 2
        } else {
            CPU_FRAME_LIMIT
        };

        // Check if CPU should stop
        if self.cpu.borrow().peripherals.input.data.flags.should_close {
            return EmulationState::Quit;
        }

        // Check if CPU should reset
        if self.cpu.borrow().peripherals.input.data.flags.should_reset {
            // Reset CPU
            self.cpu.borrow_mut().reset();

            // Reload data
            self.cpu.borrow_mut().load_font_in_memory();
            self.cpu.borrow_mut().load_cartridge_data(cartridge);

            // Reset vars
            ctx.last_debugger_command = None;
            ctx.break_next_instruction = None;
            ctx.timer_frametime = time::PreciseTime::now();
            ctx.cpu_frametime = time::PreciseTime::now();
            ctx.frametime = time::PreciseTime::now();

            // Restart !
            return EmulationState::Reset;
        }

        // Check if CPU should save
        if self.cpu.borrow().peripherals.input.data.flags.should_save {
            self.cpu
                .borrow_mut()
                .peripherals
                .input
                .data
                .flags
                .should_save = false;

            println!("Saving state...");
            let savestate = SaveState::save_from_cpu(&self.cpu.borrow());
            savestate.write_to_file(&format!("{}.sav", game_name));

            println!("State saved.");
        }

        if self.cpu.borrow().peripherals.input.data.flags.should_load {
            self.cpu
                .borrow_mut()
                .peripherals
                .input
                .data
                .flags
                .should_load = false;

            println!("Loading state...");
            let savestate = SaveState::read_from_file(&format!("{}.sav", game_name));
            match savestate {
                None => println!("No state found."),
                Some(ss) => self.cpu.borrow_mut().load_savestate(ss),
            }
        }

        if ctx
            .cpu_frametime
            .to(time::PreciseTime::now())
            .num_milliseconds()
            >= cpu_framelimit
        {
            // Read next instruction
            let opcode = self.cpu.borrow().peripherals.memory.read_opcode();
            trace_exec!(
                ctx.tracefile_handle,
                "[{:08X}] {:04X} - Reading opcode 0x{:04X}...",
                self.cpu.borrow().instruction_count,
                self.cpu.borrow().peripherals.memory.get_pointer(),
                opcode
            );

            // Check for breakpoints
            if ctx.break_next_instruction.is_none() {
                let pointer = self.cpu.borrow().peripherals.memory.get_pointer();
                if self
                    .cpu
                    .borrow()
                    .breakpoints
                    .check_breakpoint(pointer)
                    .is_some()
                {
                    ctx.break_next_instruction = Some(pointer);
                }
            }

            // Break ?
            if let Some(addr) = ctx.break_next_instruction {
                trace_exec!(ctx.tracefile_handle, "{:?}", &self.cpu.borrow());

                'debugger: loop {
                    {
                        let debugger = Debugger::new(&self.cpu, addr);
                        ctx.last_debugger_command = debugger.run();
                    }

                    if let Some(ref command) = ctx.last_debugger_command {
                        match *command {
                            Command::Quit => process::exit(1),
                            Command::AddBreakpoint(addr) => {
                                println!("Adding breakpoint for address 0x{:04X}", addr);
                                self.cpu.borrow_mut().breakpoints.register(addr);
                            }
                            Command::RemoveBreakpoint(addr) => {
                                println!("Removing breakpoint for address 0x{:04X}", addr);
                                self.cpu.borrow_mut().breakpoints.unregister(addr);
                            }
                            _ => break 'debugger,
                        }
                    }
                }
            }

            // Trace
            let opcode_enum = opcodes::get_opcode_enum(opcode);
            let (assembly, verbose) = opcodes::get_opcode_str(&opcode_enum);
            trace_exec!(ctx.tracefile_handle, "  - {:20} ; {}", assembly, verbose);

            // Update state
            self.cpu.borrow_mut().peripherals.input.update_state();

            // Execute instruction
            if self.cpu.borrow_mut().execute_instruction(&opcode_enum) {
                return EmulationState::Quit;
            }

            self.cpu.borrow_mut().instruction_count += 1;

            // Handle last debugger command
            if let Some(ref command) = ctx.last_debugger_command {
                match *command {
                    Command::Continue => {
                        ctx.break_next_instruction = None;
                    }
                    Command::Next => {
                        ctx.break_next_instruction =
                            Some(self.cpu.borrow().peripherals.memory.get_pointer());
                    }
                    _ => {}
                }
            }

            ctx.cpu_frametime = time::PreciseTime::now();
        }

        if ctx
            .timer_frametime
            .to(time::PreciseTime::now())
            .num_milliseconds()
            >= TIMER_FRAME_LIMIT
        {
            // Handle timers
            self.cpu.borrow_mut().decrement_timers();
            ctx.timer_frametime = time::PreciseTime::now();
        }

        ctx.frametime = time::PreciseTime::now();

        EmulationState::Normal
    }

    /// Run loop
    pub fn run_loop(&self, cartridge: &Cartridge) {
        let mut ctx = EmulatorContext::new();

        // Load game
        self.load_game(cartridge);

        // Get tracefile
        ctx.tracefile_handle = match self.cpu.borrow().tracefile {
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
            }
        }
    }

    // / Run emulator with a cartridge.
    // /
    // / # Arguments
    // /
    // / * `cartridge` - Cartridge
    // /
    // pub fn run(&self, cartridge: &Cartridge) {
    //     let game_name: String = cartridge.get_title().to_string();

    //     self.cpu.borrow_mut().load_font_in_memory();
    //     self.cpu.borrow_mut().load_cartridge_data(cartridge);

    //     let mut last_debugger_command: Option<Command> = None;
    //     let mut break_next_instruction: Option<C8Addr> = None;
    //     let mut tracefile_handle = match self.cpu.borrow().tracefile {
    //         Some(ref path) => Some(
    //             OpenOptions::new()
    //                 .write(true)
    //                 .create(true)
    //                 .open(path)
    //                 .unwrap(),
    //         ),
    //         None => None,
    //     };

    //     let mut timer_frametime = time::PreciseTime::now();
    //     let mut cpu_frametime = time::PreciseTime::now();
    //     let mut frametime = time::PreciseTime::now();

    //     loop {
    //         let cpu_framelimit = if self.cpu.borrow().schip_mode {
    //             CPU_FRAME_LIMIT / 2
    //         } else {
    //             CPU_FRAME_LIMIT
    //         };

    //         // Check if CPU should stop
    //         if self.cpu.borrow().peripherals.input.data.flags.should_close {
    //             break;
    //         }

    //         // Check if CPU should reset
    //         if self.cpu.borrow().peripherals.input.data.flags.should_reset {
    //             // Reset CPU
    //             self.cpu.borrow_mut().reset();

    //             // Reload data
    //             self.cpu.borrow_mut().load_font_in_memory();
    //             self.cpu.borrow_mut().load_cartridge_data(cartridge);

    //             // Reset vars
    //             last_debugger_command = None;
    //             break_next_instruction = None;
    //             timer_frametime = time::PreciseTime::now();
    //             cpu_frametime = time::PreciseTime::now();
    //             frametime = time::PreciseTime::now();

    //             // Restart !
    //             continue;
    //         }

    //         // Check if CPU should save
    //         if self.cpu.borrow().peripherals.input.data.flags.should_save {
    //             self.cpu
    //                 .borrow_mut()
    //                 .peripherals
    //                 .input
    //                 .data
    //                 .flags
    //                 .should_save = false;

    //             println!("Saving state...");
    //             let savestate = SaveState::save_from_cpu(&self.cpu.borrow());
    //             savestate.write_to_file(&format!("{}.sav", game_name));

    //             println!("State saved.");
    //         }

    //         if self.cpu.borrow().peripherals.input.data.flags.should_load {
    //             self.cpu
    //                 .borrow_mut()
    //                 .peripherals
    //                 .input
    //                 .data
    //                 .flags
    //                 .should_load = false;

    //             println!("Loading state...");
    //             let savestate = SaveState::read_from_file(&format!("{}.sav", game_name));
    //             match savestate {
    //                 None => println!("No state found."),
    //                 Some(ss) => self.cpu.borrow_mut().load_savestate(ss),
    //             }
    //         }

    //         if cpu_frametime
    //             .to(time::PreciseTime::now())
    //             .num_milliseconds()
    //             >= cpu_framelimit
    //         {
    //             // Read next instruction
    //             let opcode = self.cpu.borrow().peripherals.memory.read_opcode();
    //             trace_exec!(
    //                 tracefile_handle,
    //                 "[{:08X}] {:04X} - Reading opcode 0x{:04X}...",
    //                 self.cpu.borrow().instruction_count,
    //                 self.cpu.borrow().peripherals.memory.get_pointer(),
    //                 opcode
    //             );

    //             // Check for breakpoints
    //             if break_next_instruction.is_none() {
    //                 let pointer = self.cpu.borrow().peripherals.memory.get_pointer();
    //                 if self
    //                     .cpu
    //                     .borrow()
    //                     .breakpoints
    //                     .check_breakpoint(pointer)
    //                     .is_some()
    //                 {
    //                     break_next_instruction = Some(pointer);
    //                 }
    //             }

    //             // Break ?
    //             if let Some(addr) = break_next_instruction {
    //                 trace_exec!(tracefile_handle, "{:?}", &self.cpu.borrow());

    //                 'debugger: loop {
    //                     {
    //                         let debugger = Debugger::new(&self.cpu, addr);
    //                         last_debugger_command = debugger.run();
    //                     }

    //                     if let Some(ref command) = last_debugger_command {
    //                         match *command {
    //                             Command::Quit => process::exit(1),
    //                             Command::AddBreakpoint(addr) => {
    //                                 println!("Adding breakpoint for address 0x{:04X}", addr);
    //                                 self.cpu.borrow_mut().breakpoints.register(addr);
    //                             }
    //                             Command::RemoveBreakpoint(addr) => {
    //                                 println!("Removing breakpoint for address 0x{:04X}", addr);
    //                                 self.cpu.borrow_mut().breakpoints.unregister(addr);
    //                             }
    //                             _ => break 'debugger,
    //                         }
    //                     }
    //                 }
    //             }

    //             // Trace
    //             let opcode_enum = opcodes::get_opcode_enum(opcode);
    //             let (assembly, verbose) = opcodes::get_opcode_str(&opcode_enum);
    //             trace_exec!(tracefile_handle, "  - {:20} ; {}", assembly, verbose);

    //             // Update state
    //             self.cpu.borrow_mut().peripherals.input.update_state();

    //             // Execute instruction
    //             if self.cpu.borrow_mut().execute_instruction(&opcode_enum) {
    //                 break;
    //             }

    //             // Handle last debugger command
    //             if let Some(ref command) = last_debugger_command {
    //                 match *command {
    //                     Command::Continue => {
    //                         break_next_instruction = None;
    //                     }
    //                     Command::Next => {
    //                         break_next_instruction =
    //                             Some(self.cpu.borrow().peripherals.memory.get_pointer());
    //                     }
    //                     _ => {}
    //                 }
    //             }

    //             cpu_frametime = time::PreciseTime::now();
    //         }

    //         if timer_frametime
    //             .to(time::PreciseTime::now())
    //             .num_milliseconds()
    //             >= TIMER_FRAME_LIMIT
    //         {
    //             // Handle timers
    //             self.cpu.borrow_mut().decrement_timers();
    //             timer_frametime = time::PreciseTime::now();
    //         }

    //         if frametime.to(time::PreciseTime::now()).num_milliseconds() >= SCREEN_FRAME_LIMIT {
    //             // Update screen
    //             // self.cpu.borrow_mut().peripherals.screen.fade_pixels();
    //             // self.cpu.borrow_mut().peripherals.screen.render();
    //             frametime = time::PreciseTime::now();

    //             self.cpu.borrow_mut().instruction_count += 1;
    //         }
    //     }
    // }
}
