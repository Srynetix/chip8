//! Debugger module

mod breakpoints;
mod context;
mod errors;
mod stream;

use rustyline::error::ReadlineError;
use sdl2::EventPump;

use crate::core::cpu::CPU;
use crate::core::opcodes::{get_opcode_enum, get_opcode_str};
use crate::core::types::{convert_hex_addr, C8Addr};
use crate::emulator::{Emulator, EmulatorContext};
use crate::peripherals::cartridge::Cartridge;
use crate::peripherals::memory::INITIAL_MEMORY_POINTER;

use context::DebuggerMode;

pub use breakpoints::Breakpoints;
pub use context::DebuggerContext;
pub use stream::DebuggerStream;

/// Debugger
pub struct Debugger {}

/// Debugger state
#[derive(Debug)]
pub enum DebuggerState {
    /// Quit
    Quit,
    /// Normal
    Normal,
}

/// Debugger command
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    /// Quit
    Quit,
    /// Continue
    Continue,
    /// Show current line
    Where,
    /// Current line with context
    List(u16),
    /// Complete source
    LongList,
    /// Dump CPU
    Dump(String),
    /// Read memory at offset
    ReadMemory(C8Addr, C8Addr),
    /// Step instruction
    Step,
    /// Add breakpoint
    AddBreakpoint(C8Addr),
    /// Remove breakpoint
    RemoveBreakpoint(C8Addr),
    /// List breakpoints
    ListBreakpoints,
    /// Show help
    Help,
    /// Empty
    Empty,
}

impl Default for Debugger {
    fn default() -> Self {
        Self {}
    }
}

impl Debugger {
    /// Create new debugger
    pub fn new() -> Self {
        Default::default()
    }

    /// Step
    pub fn step(
        &self,
        emulator: &mut Emulator,
        emulator_ctx: &mut EmulatorContext,
        debug_ctx: &mut DebuggerContext,
        cartridge: &Cartridge,
        pump: &mut EventPump,
        stream: &mut DebuggerStream,
    ) -> DebuggerState {
        // Should quit?
        if debug_ctx.should_quit {
            return DebuggerState::Quit;
        }

        // Check for breakpoint
        if debug_ctx.is_continuing && !debug_ctx.breakpoint_hit {
            let pointer = emulator.cpu.peripherals.memory.get_pointer();
            if debug_ctx.breakpoints.check_breakpoint(pointer) {
                debug_ctx.breakpoint_hit = true;
                debug_ctx.has_moved = true;
                debug_ctx.pause();
            }
        }

        // Step
        if debug_ctx.is_stepping || debug_ctx.is_continuing {
            emulator.cpu.peripherals.input.process_input(pump);
            emulator.step(cartridge, emulator_ctx);

            // Just moved
            debug_ctx.has_moved = true;
            // Change debugger address
            debug_ctx.set_address(emulator.cpu.peripherals.memory.get_pointer());

            if debug_ctx.is_stepping {
                debug_ctx.is_stepping = false;
            }

            if debug_ctx.breakpoint_hit {
                debug_ctx.breakpoint_hit = false;
            }
        }

        // Interactive mode
        if let DebuggerMode::Interactive = debug_ctx.mode {
            if debug_ctx.is_paused() {
                if debug_ctx.has_moved {
                    self.show_line_context(&emulator.cpu, debug_ctx, stream, 1, 1);
                    debug_ctx.has_moved = false;
                }

                self.start_prompt(&emulator.cpu, debug_ctx, stream);
            }
        }

        DebuggerState::Normal
    }

    /// Start prompt
    pub fn start_prompt(&self, cpu: &CPU, ctx: &mut DebuggerContext, stream: &mut DebuggerStream) {
        'read: loop {
            let readline = ctx.editor.readline("> ");

            match readline {
                Ok(line) => {
                    ctx.editor.add_history_entry(line.as_ref());
                    let command = self.read_command(&line, stream);

                    match command {
                        Some(cmd) => {
                            self.handle_command(cpu, ctx, stream, cmd);
                            break 'read;
                        }
                        None => stream.writeln_stderr(format!("unknown command: {}", line)),
                    }
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                    ctx.should_quit = true;
                    break 'read;
                }
                Err(err) => {
                    stream.writeln_stderr(format!("readline error: {}", err));
                }
            }
        }
    }

    /// Read command
    ///
    /// # Arguments
    ///
    /// * `cmd` - Read command
    ///
    pub fn read_command(&self, cmd: &str, stream: &mut DebuggerStream) -> Option<Command> {
        let cmd_split: Vec<&str> = cmd.split(' ').collect();
        let command = cmd_split[0];

        match command {
            "quit" | "q" => Some(Command::Quit),
            "continue" | "c" => Some(Command::Continue),
            "dump" | "d" => {
                if cmd_split.len() == 2 {
                    Some(Command::Dump(cmd_split[1].to_string()))
                } else {
                    stream.writeln_stdout("usage: dump device");
                    stream.writeln_stdout("  devices:");
                    stream.writeln_stdout("    - memory");
                    stream.writeln_stdout("    - input");
                    stream.writeln_stdout("    - stack");
                    stream.writeln_stdout("    - registers");
                    stream.writeln_stdout("    - timers");
                    stream.writeln_stdout("    - video");
                    None
                }
            }
            "where" | "w" => Some(Command::Where),
            "list" | "l" => {
                if cmd_split.len() == 1 {
                    // Default context (2, 2)
                    Some(Command::List(2))
                } else if cmd_split.len() == 2 {
                    let sz = cmd_split[1].parse::<u16>().unwrap();
                    Some(Command::List(sz))
                } else {
                    stream.writeln_stdout("usage: list [context_size=2]");
                    None
                }
            }
            "longlist" | "ll" => Some(Command::LongList),
            "step" | "s" => Some(Command::Step),
            "help" | "h" => Some(Command::Help),
            "read-mem" | "rmem" => {
                if cmd_split.len() == 3 {
                    if let Some(addr) = convert_hex_addr(cmd_split[1]) {
                        Some(Command::ReadMemory(
                            addr,
                            cmd_split[2].parse::<C8Addr>().unwrap(),
                        ))
                    } else {
                        stream.writeln_stderr(format!("error: bad address {}", cmd_split[1]));
                        None
                    }
                } else {
                    stream.writeln_stdout("usage: read-mem addr count");
                    None
                }
            }
            "add-bp" | "b" => {
                if cmd_split.len() == 2 {
                    if let Some(addr) = convert_hex_addr(cmd_split[1]) {
                        Some(Command::AddBreakpoint(addr))
                    } else {
                        stream.writeln_stderr(format!("error: bad address {}", cmd_split[1]));
                        None
                    }
                } else {
                    stream.writeln_stdout("usage: add-bp addr");
                    None
                }
            }
            "rem-bp" | "rb" => {
                if cmd_split.len() == 2 {
                    if let Some(addr) = convert_hex_addr(cmd_split[1]) {
                        Some(Command::RemoveBreakpoint(addr))
                    } else {
                        stream.writeln_stderr(format!("error: bad address {}", cmd_split[1]));
                        None
                    }
                } else {
                    stream.writeln_stdout("usage: rem-bp addr");
                    None
                }
            }
            "list-bp" | "lb" => Some(Command::ListBreakpoints),
            "" => Some(Command::Empty),
            _ => None,
        }
    }

    /// Handle command
    pub fn handle_command(
        &self,
        cpu: &CPU,
        ctx: &mut DebuggerContext,
        stream: &mut DebuggerStream,
        command: Command,
    ) {
        match command {
            Command::Dump(ref device) => match &device[..] {
                "memory" | "m" => println!("{:?}", cpu.peripherals.memory),
                "video" | "v" => cpu.peripherals.screen.dump_screen(),
                "input" | "i" => println!("{:?}", cpu.peripherals.input),
                "registers" | "r" => println!("{:?}", cpu.registers),
                "stack" | "s" => println!("{:?}", cpu.stack),
                "timers" | "t" => {
                    println!("{:?}", cpu.delay_timer);
                    println!("{:?}", cpu.sound_timer);
                }
                _ => cpu.show_debug(),
            },
            Command::ReadMemory(addr, count) => {
                println!("reading memory at {:04X} on {} byte(s)", addr, count);
                println!(
                    "{:?}",
                    cpu.peripherals.memory.read_data_at_offset(addr, count)
                );
            }
            Command::Step => ctx.is_stepping = true,
            Command::Continue => ctx.is_continuing = true,
            Command::Where => self.show_line(cpu, ctx, stream, ctx.address),
            Command::List(sz) => self.show_line_context(cpu, ctx, stream, sz, sz),
            Command::LongList => self.show_source(cpu, ctx, stream),
            Command::Help => self.show_help(stream),
            Command::AddBreakpoint(addr) => {
                ctx.register_breakpoint(addr);
                println!("breakpoint added to address 0x{:04X}", addr);
            }
            Command::RemoveBreakpoint(addr) => {
                ctx.unregister_breakpoint(addr);
                println!("breakpoint removed from address 0x{:04X}", addr);
            }
            Command::ListBreakpoints => ctx.breakpoints.dump_breakpoints(),
            Command::Quit => ctx.should_quit = true,
            _ => {}
        }
    }

    ////////////////
    // PRIVATE

    fn show_line(
        &self,
        cpu: &CPU,
        ctx: &DebuggerContext,
        stream: &mut DebuggerStream,
        addr: C8Addr,
    ) {
        let opcode = cpu.peripherals.memory.read_opcode_at_address(addr);
        let opcode_enum = get_opcode_enum(opcode);
        let (asm, txt) = get_opcode_str(&opcode_enum);

        let cursor = if ctx.address == addr { "-->" } else { "" };

        stream.writeln_stdout(format!("{:04X}| {:3} {:20} ; {}", addr, cursor, asm, txt));
    }

    fn show_line_context(
        &self,
        cpu: &CPU,
        ctx: &DebuggerContext,
        stream: &mut DebuggerStream,
        prev_size: u16,
        next_size: u16,
    ) {
        let base_addr = ctx.address;

        // Limit = INITIAL_MEMORY_POINTER
        let min_limit = std::cmp::max(base_addr - prev_size * 2, INITIAL_MEMORY_POINTER);
        let max_limit = base_addr + next_size * 2;

        for addr in (min_limit..=max_limit).step_by(2) {
            self.show_line(cpu, ctx, stream, addr);
        }
    }

    fn show_source(&self, cpu: &CPU, ctx: &DebuggerContext, stream: &mut DebuggerStream) {
        let code_end_pointer = cpu.peripherals.memory.get_end_pointer();
        for addr in (INITIAL_MEMORY_POINTER..=code_end_pointer).step_by(2) {
            self.show_line(cpu, ctx, stream, addr);
        }
    }

    fn show_help(&self, stream: &mut DebuggerStream) {
        stream.writeln_stdout("available commands: ");
        stream.writeln_stdout("  continue|c      - continue");
        stream.writeln_stdout("  dump|d          - dump device");
        stream.writeln_stdout("  where|w         - show current line");
        stream.writeln_stdout("  list|l          - show current line with context");
        stream.writeln_stdout("  longlist|ll     - show complete source");
        stream.writeln_stdout("  step|s          - step");
        stream.writeln_stdout("  add-bp|b        - add breakpoint at address");
        stream.writeln_stdout("  rem-bp|rb       - remove breakpoint at address");
        stream.writeln_stdout("  list-bp|lb      - list breakpoints");
        stream.writeln_stdout("  read-mem|rmem   - read memory at offset");
        stream.writeln_stdout("  quit|q          - quit program");
        stream.writeln_stdout("  help|h          - show this help");
    }
}
