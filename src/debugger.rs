//! CHIP-8 debugger

use std::cell::RefCell;
use std::rc::Rc;

use super::cpu::CPU;
use super::opcodes::{get_opcode_enum, get_opcode_str};
use super::types::{convert_hex_addr, C8Addr};

use rustyline::error::ReadlineError;
use rustyline::Editor;

/// Debugger
pub struct Debugger {
    addr: C8Addr,
    cpu: Rc<RefCell<CPU>>,
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
    /// Next instruction
    Next,
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

impl Debugger {
    /// Create new debugger
    ///
    /// # Arguments
    ///
    /// * `cpu` - CPU reference
    /// * `addr` - Starting address
    ///
    pub fn new(cpu: &Rc<RefCell<CPU>>, addr: C8Addr) -> Debugger {
        Debugger {
            addr,
            cpu: cpu.clone(),
        }
    }

    /// Show line
    ///
    /// # Arguments
    ///
    /// * `addr` - Address
    ///
    fn show_line(&self, addr: C8Addr) {
        let opcode = self
            .cpu
            .borrow()
            .peripherals
            .memory
            .read_opcode_at_address(addr);
        let opcode_enum = get_opcode_enum(opcode);
        let (asm, txt) = get_opcode_str(&opcode_enum);

        let cursor = if self.addr == addr { "-->" } else { "" };

        println!("{:04X}> {:3} {:20} ; {}", addr, cursor, asm, txt);
    }

    /// Show line context
    fn show_line_context(&self, prev_size: u16, next_size: u16) {
        let base_addr = self.addr;

        // Limit = 0200
        let min_limit = std::cmp::max(base_addr - prev_size * 2, 0x0200);
        let max_limit = base_addr + next_size * 2;

        for addr in (min_limit..=max_limit).step_by(2) {
            self.show_line(addr);
        }
    }

    /// Show complete source
    fn show_source(&self) {
        let code_end_pointer = self.cpu.borrow().peripherals.memory.get_end_pointer();
        for addr in (0x0200..=code_end_pointer).step_by(2) {
            self.show_line(addr)
        }
    }

    /// Run
    pub fn run(&self) -> Option<Command> {
        let mut rl = Editor::<()>::new();
        println!("Debugger on address {:04X}.", self.addr);

        self.show_line_context(1, 1);

        #[allow(unused_assignments)]
        let mut last_command: Option<Command> = None;

        'running: loop {
            let readline = rl.readline("> ");

            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_ref());

                    if let Some(ref command) = self.read_command(&line) {
                        last_command = Some(command.clone());

                        match *command {
                            Command::Dump(ref device) => match &device[..] {
                                "memory" | "m" => {
                                    println!("{:?}", self.cpu.borrow().peripherals.memory)
                                }
                                "video" | "v" => self.cpu.borrow().peripherals.screen.dump_screen(),
                                "input" | "i" => {
                                    println!("{:?}", self.cpu.borrow().peripherals.input)
                                }
                                "registers" | "r" => println!("{:?}", self.cpu.borrow().registers),
                                "stack" | "s" => println!("{:?}", self.cpu.borrow().stack),
                                "timers" | "t" => {
                                    println!("{:?}", self.cpu.borrow().delay_timer);
                                    println!("{:?}", self.cpu.borrow().sound_timer);
                                }
                                _ => self.cpu.borrow().show_debug(),
                            },
                            Command::ReadMemory(addr, count) => {
                                println!("Reading memory at {:04X} on {} byte(s).", addr, count);
                                println!(
                                    "{:?}",
                                    self.cpu
                                        .borrow()
                                        .peripherals
                                        .memory
                                        .read_data_at_offset(addr, count)
                                );
                            }
                            Command::Where => self.show_line(self.addr),
                            Command::List(sz) => self.show_line_context(sz, sz),
                            Command::LongList => self.show_source(),
                            Command::Help => self.show_help(),
                            Command::ListBreakpoints => {
                                self.cpu.borrow().breakpoints.dump_breakpoints()
                            }
                            Command::Empty => {}
                            _ => break 'running,
                        }
                    } else {
                        println!("{}: command unknown", line);
                    }
                }

                Err(ReadlineError::Interrupted) => {
                    last_command = Some(Command::Quit);
                    break 'running;
                }

                Err(ReadlineError::Eof) => {
                    last_command = Some(Command::Quit);
                    break 'running;
                }

                Err(err) => {
                    println!("Error in readline: {:?}", err);
                }
            }
        }

        last_command
    }

    /// Read command
    ///
    /// # Arguments
    ///
    /// * `cmd` - Read command
    ///
    fn read_command(&self, cmd: &str) -> Option<Command> {
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
            "next" | "n" => Some(Command::Next),
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
        println!("  next|n          - Step");
        println!("  add-bp|b        - Add breakpoint at address");
        println!("  rem-bp|rb       - Remove breakpoint at address");
        println!("  list-bp|lb      - List breakpoints");
        println!("  read-mem|rmem   - Read memory at offset");
        println!("  quit|q          - Quit program");
        println!("  help|h          - Show this help");
    }
}
