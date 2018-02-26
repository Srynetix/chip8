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
            OpCode::JP(addr) => {
                // Set pointer to address
                self.memory.set_pointer(addr);
                advance_pointer = false;
            },
            OpCode::CALL(addr) => {
                // Store current address and set pointer
                self.stack.store(self.memory.get_pointer());
                self.memory.set_pointer(addr);
                advance_pointer = false;                
            }
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