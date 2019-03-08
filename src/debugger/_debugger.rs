//! CHIP-8 debugger

use std::cell::RefCell;
use std::rc::Rc;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::core::cpu::CPU;
use crate::core::opcodes::{get_opcode_enum, get_opcode_str};
use crate::core::types::{convert_hex_addr, C8Addr};

type CPURef = Rc<RefCell<CPU>>;

/// Debugger
pub struct Debugger {}

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

/// Debugger context
pub struct DebuggerContext {
    /// Last command
    pub last_command: Option<Command>,
    /// Running
    pub running: bool,
    /// Address
    pub address: C8Addr,
    /// Is stepping
    pub is_stepping: bool,
    /// Is continuing
    pub is_continuing: bool,
}

impl Default for DebuggerContext {
    fn default() -> Self {
        Self {
            last_command: None,
            address: 0,
            running: true,
            is_stepping: false,
            is_continuing: false,
        }
    }
}

impl DebuggerContext {
    /// Create new context
    pub fn new() -> Self {
        Default::default()
    }

    /// Set debugger address
    pub fn set_address(&mut self, addr: C8Addr) {
        self.address = addr;
    }

    /// Is paused?
    pub fn is_paused(&self) -> bool {
        !self.is_continuing
    }
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

    /// Show line
    ///
    /// # Arguments
    ///
    /// * `addr` - Address
    ///
    fn show_line(&self, cpu: &CPURef, ctx: &DebuggerContext, addr: C8Addr) {
        let opcode = cpu.borrow().peripherals.memory.read_opcode_at_address(addr);
        let opcode_enum = get_opcode_enum(opcode);
        let (asm, txt) = get_opcode_str(&opcode_enum);

        let cursor = if ctx.address == addr { "-->" } else { "" };

        println!("{:04X}| {:3} {:20} ; {}", addr, cursor, asm, txt);
    }

    /// Show line context
    fn show_line_context(
        &self,
        cpu: &CPURef,
        ctx: &DebuggerContext,
        prev_size: u16,
        next_size: u16,
    ) {
        let base_addr = ctx.address;

        // Limit = 0200
        let min_limit = std::cmp::max(base_addr - prev_size * 2, 0x0200);
        let max_limit = base_addr + next_size * 2;

        for addr in (min_limit..=max_limit).step_by(2) {
            self.show_line(cpu, ctx, addr);
        }
    }

    /// Show complete source
    fn show_source(&self, cpu: &CPURef, ctx: &DebuggerContext) {
        let code_end_pointer = cpu.borrow().peripherals.memory.get_end_pointer();
        for addr in (0x0200..=code_end_pointer).step_by(2) {
            self.show_line(cpu, ctx, addr);
        }
    }

    fn read_line(&self, rl: &mut rustyline::Editor<()>, cpu: &CPURef, ctx: &mut DebuggerContext) {
        let readline = rl.readline("> ");

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());

                if let Some(ref command) = self.read_command(&line) {
                    ctx.last_command = Some(command.clone());
                    self.handle_command(cpu, ctx, command.clone());
                } else {
                    println!("{}: command unknown", line);
                }
            }

            Err(ReadlineError::Interrupted) => {
                ctx.last_command = Some(Command::Quit);
            }

            Err(ReadlineError::Eof) => {
                ctx.last_command = Some(Command::Quit);
            }

            Err(err) => {
                println!("Error in readline: {:?}", err);
            }
        }
    }

    fn handle_command(&self, cpu: &CPURef, ctx: &DebuggerContext, command: Command) {
        match command {
            Command::Dump(ref device) => match &device[..] {
                "memory" | "m" => println!("{:?}", cpu.borrow().peripherals.memory),
                "video" | "v" => cpu.borrow().peripherals.screen.dump_screen(),
                "input" | "i" => println!("{:?}", cpu.borrow().peripherals.input),
                "registers" | "r" => println!("{:?}", cpu.borrow().registers),
                "stack" | "s" => println!("{:?}", cpu.borrow().stack),
                "timers" | "t" => {
                    println!("{:?}", cpu.borrow().delay_timer);
                    println!("{:?}", cpu.borrow().sound_timer);
                }
                _ => cpu.borrow().show_debug(),
            },
            Command::ReadMemory(addr, count) => {
                println!("Reading memory at {:04X} on {} byte(s).", addr, count);
                println!(
                    "{:?}",
                    cpu.borrow()
                        .peripherals
                        .memory
                        .read_data_at_offset(addr, count)
                );
            }
            Command::Where => self.show_line(cpu, ctx, ctx.address),
            Command::List(sz) => self.show_line_context(cpu, ctx, sz, sz),
            Command::LongList => self.show_source(cpu, ctx),
            Command::Help => self.show_help(),
            Command::ListBreakpoints => cpu.borrow().breakpoints.dump_breakpoints(),
            Command::Empty => {}
            _ => {}
        }
    }

    /// Run loop
    pub fn run_loop(&self, cpu: &CPURef, addr: C8Addr) -> Option<Command> {
        let mut rl = Editor::<()>::new();
        println!("Debugger on address {:04X}.", addr);

        let mut ctx = DebuggerContext::new();
        ctx.address = addr;

        self.show_line_context(cpu, &ctx, 1, 1);

        while ctx.running {
            self.read_line(&mut rl, cpu, &mut ctx);
        }

        ctx.last_command.as_ref().cloned()
    }

    /// Run
    // pub fn run(&self) -> Option<Command> {
    //     let mut rl = Editor::<()>::new();
    //     println!("Debugger on address {:04X}.", self.addr);

    //     self.show_line_context(1, 1);

    //     #[allow(unused_assignments)]
    //     let mut last_command: Option<Command> = None;

    //     'running: loop {
    //         let readline = rl.readline("> ");

    //         match readline {
    //             Ok(line) => {
    //                 rl.add_history_entry(line.as_ref());

    //                 if let Some(ref command) = self.read_command(&line) {
    //                     last_command = Some(command.clone());

    //                     match *command {
    //                         Command::Dump(ref device) => match &device[..] {
    //                             "memory" | "m" => {
    //                                 println!("{:?}", self.cpu.borrow().peripherals.memory)
    //                             }
    //                             "video" | "v" => self.cpu.borrow().peripherals.screen.dump_screen(),
    //                             "input" | "i" => {
    //                                 println!("{:?}", self.cpu.borrow().peripherals.input)
    //                             }
    //                             "registers" | "r" => println!("{:?}", self.cpu.borrow().registers),
    //                             "stack" | "s" => println!("{:?}", self.cpu.borrow().stack),
    //                             "timers" | "t" => {
    //                                 println!("{:?}", self.cpu.borrow().delay_timer);
    //                                 println!("{:?}", self.cpu.borrow().sound_timer);
    //                             }
    //                             _ => self.cpu.borrow().show_debug(),
    //                         },
    //                         Command::ReadMemory(addr, count) => {
    //                             println!("Reading memory at {:04X} on {} byte(s).", addr, count);
    //                             println!(
    //                                 "{:?}",
    //                                 self.cpu
    //                                     .borrow()
    //                                     .peripherals
    //                                     .memory
    //                                     .read_data_at_offset(addr, count)
    //                             );
    //                         }
    //                         Command::Where => self.show_line(self.addr),
    //                         Command::List(sz) => self.show_line_context(sz, sz),
    //                         Command::LongList => self.show_source(),
    //                         Command::Help => self.show_help(),
    //                         Command::ListBreakpoints => {
    //                             self.cpu.borrow().breakpoints.dump_breakpoints()
    //                         }
    //                         Command::Empty => {}
    //                         _ => break 'running,
    //                     }
    //                 } else {
    //                     println!("{}: command unknown", line);
    //                 }
    //             }

    //             Err(ReadlineError::Interrupted) => {
    //                 last_command = Some(Command::Quit);
    //                 break 'running;
    //             }

    //             Err(ReadlineError::Eof) => {
    //                 last_command = Some(Command::Quit);
    //                 break 'running;
    //             }

    //             Err(err) => {
    //                 println!("Error in readline: {:?}", err);
    //             }
    //         }
    //     }

    //     last_command
    // }

    /// Read command
    ///
    /// # Arguments
    ///
    /// * `cmd` - Read command
    ///
    pub fn read_command(&self, cmd: &str) -> Option<Command> {
        let cmd_split: Vec<&str> = cmd.split(' ').collect();
        let command = cmd_split[0];

        match command {
            "quit" | "q" => Some(Command::Quit),
            "continue" | "c" => Some(Command::Continue),
            "dump" | "d" => {
                if cmd_split.len() == 2 {
                    Some(Command::Dump(cmd_split[1].to_string()))
                } else {
                    println!("usage: dump device");
                    println!("  devices:");
                    println!("    - memory");
                    println!("    - input");
                    println!("    - stack");
                    println!("    - registers");
                    println!("    - timers");
                    println!("    - video");
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
                    println!("usage: list [context_size=2]");
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
                        println!("error: bad address {}", cmd_split[1]);
                        None
                    }
                } else {
                    println!("usage: read-mem addr count");
                    None
                }
            }
            "add-bp" | "b" => {
                if cmd_split.len() == 2 {
                    if let Some(addr) = convert_hex_addr(cmd_split[1]) {
                        Some(Command::AddBreakpoint(addr))
                    } else {
                        println!("error: bad address {}", cmd_split[1]);
                        None
                    }
                } else {
                    println!("usage: add-bp addr");
                    None
                }
            }
            "rem-bp" | "rb" => {
                if cmd_split.len() == 2 {
                    if let Some(addr) = convert_hex_addr(cmd_split[1]) {
                        Some(Command::RemoveBreakpoint(addr))
                    } else {
                        println!("error: bad address {}", cmd_split[1]);
                        None
                    }
                } else {
                    println!("usage: rem-bp addr");
                    None
                }
            }
            "list-bp" | "lb" => Some(Command::ListBreakpoints),
            "" => Some(Command::Empty),
            _ => None,
        }
    }

    /// Show help
    fn show_help(&self) {
        println!("Available commands: ");
        println!("  continue|c      - Continue");
        println!("  dump|d          - Dump device");
        println!("  where|w         - Show current line");
        println!("  list|l          - Show current line with context");
        println!("  longlist|ll     - Show complete source");
        println!("  step|s          - Step");
        println!("  add-bp|b        - Add breakpoint at address");
        println!("  rem-bp|rb       - Remove breakpoint at address");
        println!("  list-bp|lb      - List breakpoints");
        println!("  read-mem|rmem   - Read memory at offset");
        println!("  quit|q          - Quit program");
        println!("  help|h          - Show this help");
    }
}
