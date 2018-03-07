//! CHIP-8 debugger

use std::io::{self, Write};

use super::types::{C8Addr, convert_hex_addr};
use super::cpu::CPU;
use super::opcodes::{get_opcode_enum, get_opcode_str};

/// Debugger
pub struct Debugger<'a> {
    addr: C8Addr,
    cpu: &'a CPU
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
    /// Read memory at offset,
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
    Help
}

impl<'a> Debugger<'a> {

    /// Create new debugger
    /// 
    /// # Arguments
    /// 
    /// * `cpu` - CPU reference
    /// * `addr` - Starting address
    /// 
    pub fn new(cpu: &'a CPU, addr: C8Addr) -> Debugger<'a> {
        Debugger {
            addr,
            cpu
        }
    }

    /// Run
    pub fn run(&self) -> Option<Command> {
        println!("Debugger on address {:04X}.", self.addr);
        
        let opcode = self.cpu.peripherals.memory.read_opcode_at_address(self.addr);
        let opcode_enum = get_opcode_enum(opcode);
        let (opcode_asm, opcode_txt) = get_opcode_str(&opcode_enum);
        println!("  - {:20} ; {}", opcode_asm, opcode_txt);

        #[allow(unused_assignments)]
        let mut last_command = None;

        'running: loop {
            // Prompt
            print!("> ");
            io::stdout().flush().unwrap();

            // Read
            let mut buffer = String::new();
            match io::stdin().read_line(&mut buffer) {
                Ok(_) => {
                    let len = buffer.trim_right().len();
                    buffer.truncate(len);

                    if let Some(ref command) = self.read_command(&buffer) {
                        last_command = Some(command.clone());

                        match *command {
                            Command::Dump(ref device) => {
                                match &device[..] {
                                    "memory" | "m" => println!("{:?}", self.cpu.peripherals.memory),
                                    "video" | "v" => self.cpu.peripherals.screen.dump_screen(),
                                    "input" | "i" => println!("{:?}", self.cpu.peripherals.input),
                                    "registers" | "r" => println!("{:?}", self.cpu.registers),
                                    "stack" | "s" => println!("{:?}", self.cpu.stack),
                                    "timers" | "t" => {
                                        println!("{:?}", self.cpu.delay_timer);
                                        println!("{:?}", self.cpu.sound_timer);
                                    },
                                    _ => self.cpu.show_debug()
                                }
                            },
                            Command::ReadMemory(addr, count) => {
                                println!("Reading memory at {:04X} on {} byte(s).", addr, count);
                                println!("{:?}", self.cpu.peripherals.memory.read_data_at_offset(addr, count));
                            },
                            Command::Show => {
                                println!("  - {:20} ; {}", opcode_asm, opcode_txt);
                            },
                            Command::Help => {
                                self.show_help()
                            },
                            Command::ListBreakpoints => {
                                self.cpu.breakpoints.dump_breakpoints()
                            }
                            _ => {
                                break 'running
                            }
                        }
                    } else {
                        println!("{}: command unknown", buffer);
                    }
                },
                Err(error) => {
                    println!("error: {}", error)
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
        let cmd_split: Vec<&str> = cmd.split(" ").collect();
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
                    Some(
                        Command::ReadMemory(
                            convert_hex_addr(cmd_split[1]),
                            cmd_split[2].parse::<C8Addr>().unwrap()
                        )
                    )
                } else {
                    println!("usage: read-mem addr count");
                    None
                }
            },
            "add-bp" | "ab" => {
                if cmd_split.len() == 2 {
                    Some(Command::AddBreakpoint(convert_hex_addr(cmd_split[1])))
                } else {
                    println!("usage: add-bp addr");
                    None
                }
            },
            "rem-bp" | "rb" => {
                if cmd_split.len() == 2 {
                    Some(Command::RemoveBreakpoint(convert_hex_addr(cmd_split[1])))                    
                } else {
                    println!("usage: rem-bp addr");
                    None
                }
            },
            "list-bp" | "lb" => {
                Some(Command::ListBreakpoints)
            },
            _ => None
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
        println!("  next|n          - Step next");
        println!("  quit|q          - Quit program");
        println!("  help|h          - Show this help");
    }
}