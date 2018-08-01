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
#[derive(Clone)]
pub enum Command {
    /// Quit
    Quit,
    /// Continue
    Continue,
    /// Show line
    Show,
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

    /// Run
    pub fn run(&self) -> Option<Command> {
        let mut rl = Editor::<()>::new();
        println!("Debugger on address {:04X}.", self.addr);

        let opcode = self
            .cpu
            .borrow()
            .peripherals
            .memory
            .read_opcode_at_address(self.addr);
        let opcode_enum = get_opcode_enum(opcode);
        let (opcode_asm, opcode_txt) = get_opcode_str(&opcode_enum);
        println!("  - {:20} ; {}", opcode_asm, opcode_txt);

        #[allow(unused_assignments)]
        let mut last_command = None;

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
                            Command::Show => {
                                println!("  - {:20} ; {}", opcode_asm, opcode_txt);
                            }
                            Command::Help => self.show_help(),
                            Command::ListBreakpoints => {
                                self.cpu.borrow().breakpoints.dump_breakpoints()
                            }
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
            "show" | "s" => Some(Command::Show),
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
            "add-bp" | "ab" => {
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
            _ => None,
        }
    }

    /// Show help
    fn show_help(&self) {
        println!("Available commands: ");
        println!("  continue|c      - Continue");
        println!("  dump|d          - Dump device");
        println!("  show|s          - Show line");
        println!("  next|n          - Step");
        println!("  add-bp|ab       - Add breakpoint at address");
        println!("  rem-bp|rb       - Remove breakpoint at address");
        println!("  list-bp|lb      - List breakpoints");
        println!("  read-mem|rmem   - Read memory at offset");
        println!("  quit|q          - Quit program");
        println!("  help|h          - Show this help");
    }
}
