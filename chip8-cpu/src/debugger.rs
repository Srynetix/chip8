//! CHIP-8 debugger

use std::io::{self, Write};

use chip8_core::types::{C8Addr};
use super::cpu::CPU;
use super::opcodes::{get_opcode_enum, get_opcode_str};

/// Debugger
pub struct Debugger<'a> {
    addr: C8Addr,
    cpu: &'a CPU
}

/// Debugger command
#[derive(Copy, Clone)]
pub enum Command {
    /// Quit
    Quit,
    /// Continue
    Continue,
    /// Show line
    Show,
    /// Dump CPU
    Dump,
    /// Next instruction
    Next,
    /// Show help
    Help
}

impl<'a> Debugger<'a> {
    /// Create
    pub fn new(cpu: &'a CPU, addr: C8Addr) -> Debugger<'a> {
        Debugger {
            addr,
            cpu
        }
    }

    /// Set address
    pub fn set_address(&mut self, addr: C8Addr) {
        self.addr = addr;
    }

    /// Run
    pub fn run(&self) -> Option<Command> {
        println!("Debugger on address {:04X}.", self.addr);
        
        let opcode = self.cpu.peripherals.memory.read_opcode_at_address(self.addr);
        let opcode_enum = get_opcode_enum(opcode);
        let (opcode_asm, opcode_txt) = get_opcode_str(&opcode_enum);
        println!("  - {:20} ; {}", opcode_asm, opcode_txt);

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

                    if let Some(command) = self.read_command(&buffer) {
                        last_command = Some(command);

                        match command {
                            Command::Quit => {
                                break 'running;
                            },
                            Command::Continue => {
                                break 'running
                            }
                            Command::Dump => {
                                self.cpu.show_debug()
                            },
                            Command::Show => {
                                println!("  - {:20} ; {}", opcode_asm, opcode_txt);
                            },
                            Command::Next => {
                                break 'running
                            },
                            Command::Help => {
                                self.show_help()
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
    fn read_command(&self, cmd: &str) -> Option<Command> {
        match &(cmd.to_lowercase())[..] {
            "quit" | "q" => Some(Command::Quit),
            "continue" | "c" => Some(Command::Continue),
            "dump" | "d" => Some(Command::Dump),
            "show" | "s" => Some(Command::Show),
            "next" | "n" => Some(Command::Next),
            "help" | "h" => Some(Command::Help),
            _ => None
        }
    }

    /// Show help
    fn show_help(&self) {
        println!("Available commands: ");
        println!("  continue|c  - Continue");
        println!("  dump|d      - Dump CPU");
        println!("  show|s      - Show line");
        println!("  next|n      - Step");
        println!("  quit|q      - Quit program");
        println!("  help|h      - Show this help");
    }
}