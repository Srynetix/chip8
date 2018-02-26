//! CHIP-8 CPU

use std::fmt;

pub mod memory;
pub mod registers;
pub mod stack;
pub mod timer;
pub mod types;
pub mod opcodes;
pub mod video;

use self::opcodes::OpCode;
use self::types::{C8Short};

/// CHIP-8 CPU struct
pub struct CPU {
    /// Memory
    pub memory: memory::Memory,
    /// Video memory
    pub video_memory: video::VideoMemory,
    /// Registers
    pub registers: registers::Registers,
    /// Stack
    pub stack: stack::Stack,

    /// Syncronization timer
    pub sync_timer: timer::Timer,
    /// Sound timer
    pub sound_timer: timer::Timer,

    /// Instruction count
    instruction_count: usize
}

impl CPU {

    /// Create CHIP-8 CPU
    pub fn new() -> Self {
        CPU {
            memory: memory::Memory::new(),
            video_memory: video::VideoMemory::new(),
            registers: registers::Registers::new(),
            stack: stack::Stack::new(),

            sync_timer: timer::Timer::new(),
            sound_timer: timer::Timer::new(),

            instruction_count: 0
        }
    }

    /// Run CPU
    pub fn run() {
        
    }

    /// Read next instruction
    pub fn read_next_instruction(&mut self) {
        let opcode = self.memory.read_opcode();
        println!("{:04X} - Reading opcode 0x{:04X}...", self.instruction_count, opcode);

        let opcode_enum = opcodes::get_opcode_enum(opcode)
                                .expect(&format!("Unknown opcode: {:04X}", opcode));

        let (assembly, verbose) = opcodes::get_opcode_str(&opcode_enum);
        println!("  - {:30} ; {}", assembly, verbose);

        let mut advance_pointer = true;

        match opcode_enum {
            OpCode::SYS(_addr) => {
                // Do nothing
            },
            OpCode::CLS => {
                // Clear screen
                self.video_memory.clear_screen();
            },
            OpCode::RET => {
                // Get last stored address
                let addr = self.stack.pop();
                self.memory.set_pointer(addr);
            },
            OpCode::JP(addr) => {
                // Set pointer to address
                self.memory.set_pointer(addr);
                advance_pointer = false;
            },
            OpCode::CALL(addr) => {
                // Store current address and set pointer
                self.stack.push(self.memory.get_pointer());
                self.memory.set_pointer(addr);
                advance_pointer = false;                
            },
            OpCode::SEByte(reg, byte) => {
                // Compare register with byte and then advance pointer
                if self.registers.get_register(reg) == byte {
                    self.memory.advance_pointer();
                }
            },
            OpCode::SNEByte(reg, byte) => {
                // Compare register with byte and then advance pointer
                if self.registers.get_register(reg) != byte {
                    self.memory.advance_pointer();
                }
            },
            OpCode::SE(reg1, reg2) => {
                // Compare register values
                if self.registers.get_register(reg1) == self.registers.get_register(reg2) {
                    self.memory.advance_pointer();
                }
            },
            OpCode::LDByte(reg, byte) => {
                // Puts byte in register
                self.registers.set_register(reg, byte);
            },
            OpCode::ADDByte(reg, byte) => {
                // Add byte in register
                let r = self.registers.get_register(reg);

                self.registers.set_register(reg, r + byte);
            },
            OpCode::LD(reg1, reg2) => {
                // Load register value in another
                let r = self.registers.get_register(reg2);

                self.registers.set_register(reg1, r);
            },
            OpCode::OR(reg1, reg2) => {
                // OR between two registers
                let r1 = self.registers.get_register(reg1);
                let r2 = self.registers.get_register(reg2);

                self.registers.set_register(reg1, r1 & r2);
            },
            OpCode::AND(reg1, reg2) => {
                // AND between two registers
                let r1 = self.registers.get_register(reg1);
                let r2 = self.registers.get_register(reg2);

                self.registers.set_register(reg1, r1 | r2)
            },
            OpCode::XOR(reg1, reg2) => {
                // XOR between two registers
                let r1 = self.registers.get_register(reg1);
                let r2 = self.registers.get_register(reg2);

                self.registers.set_register(reg1, r1 ^ r2);
            },
            OpCode::ADD(reg1, reg2) => {
                // ADD between two registers
                let r1 = self.registers.get_register(reg1);
                let r2 = self.registers.get_register(reg2);

                if (r1 as C8Short + r2 as C8Short) > 0xFF {
                    self.registers.set_carry_register(1);
                } else {
                    self.registers.set_carry_register(0);
                }

                self.registers.set_register(reg1, r1 + r2);
            },
            OpCode::SUB(reg1, reg2) => {
                // SUB between two registers
                let r1 = self.registers.get_register(reg1);
                let r2 = self.registers.get_register(reg2);

                if r1 < r2 {
                    self.registers.set_carry_register(1);
                } else {
                    self.registers.set_carry_register(0);
                }

                self.registers.set_register(reg1, r1 - r2);
            },
            _ => println!(" - Not implemented")
        };

        if advance_pointer {
            self.memory.advance_pointer();
        }

        self.instruction_count += 1;
    }
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CPU {{\n")?;
        
        write!(f, "  memory: {{\n")?;
        write!(f, "{:?}", self.memory)?;
        write!(f, "  }},\n")?;

        write!(f, "  video memory: {{\n")?;
        write!(f, "{:?}", self.video_memory)?;
        write!(f, "  }},\n")?;
        
        write!(f, "  registers: {{\n")?;
        write!(f, "{:?}", self.registers)?;
        write!(f, "  }}\n")?;
        
        write!(f, "  stack: {{\n")?;
        write!(f, "{:?}", self.stack)?;
        write!(f, "  }}\n")?;

        write!(f, "  sync_timer: {:?}\n", self.sync_timer)?;
        write!(f, "  sound_timer: {:?}\n", self.sound_timer)?;

        write!(f, "}}\n")
    }
}