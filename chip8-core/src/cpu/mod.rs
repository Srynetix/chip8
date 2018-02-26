//! CHIP-8 CPU

use std::fmt;

pub mod memory;
pub mod registers;
pub mod stack;
pub mod timer;
pub mod types;
pub mod opcodes;
pub mod video;
pub mod font;

use rand::random;

use self::opcodes::OpCode;
use self::types::{C8Byte, C8Addr};
use self::font::{FONT_DATA_ADDR, FONT_CHAR_WIDTH, FONT_CHAR_HEIGHT};
use self::video::{VIDEO_MEMORY_WIDTH, VIDEO_MEMORY_HEIGHT};

/// CHIP-8 CPU struct
pub struct CPU {
    /// Memory
    memory: memory::Memory,
    /// Video memory
    video_memory: video::VideoMemory,
    /// Registers
    registers: registers::Registers,
    /// Stack
    stack: stack::Stack,

    /// Syncronization timer
    delay_timer: timer::Timer,
    /// Sound timer
    sound_timer: timer::Timer,

    /// Font
    font: font::Font,
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

            delay_timer: timer::Timer::new(),
            sound_timer: timer::Timer::new(),

            font: font::Font::new_system_font(),
            instruction_count: 0
        }
    }

    /// Load font in memory
    pub fn load_font_in_memory(&mut self) {
        self.memory.write_data_at_offset(FONT_DATA_ADDR, self.font.get_data());
    }

    /// Run CPU
    pub fn run(&mut self) {
        println!("> Run");
    }

    /// Read cartridge data
    pub fn load_cartridge_data(&mut self, cartridge_data: &[u8]) {
        self.memory.reset_pointer();
        self.memory.write_data_at_pointer(cartridge_data);
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
                let r = self.registers.get_register(reg);

                if r == byte {
                    self.memory.advance_pointer();
                }
            },
            OpCode::SNEByte(reg, byte) => {
                // Compare register with byte and then advance pointer
                let r = self.registers.get_register(reg);
                
                if r != byte {
                    self.memory.advance_pointer();
                }
            },
            OpCode::SE(reg1, reg2) => {
                // Compare register values
                let r1 = self.registers.get_register(reg1);
                let r2 = self.registers.get_register(reg2);

                if r1 == r2 {
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
                let (res, overflow) = r1.overflowing_add(r2);

                if overflow {
                    self.registers.set_carry_register(1);
                } else {
                    self.registers.set_carry_register(0);
                }

                self.registers.set_register(reg1, res);
            },
            OpCode::SUB(reg1, reg2) => {
                // SUB between two registers
                let r1 = self.registers.get_register(reg1);
                let r2 = self.registers.get_register(reg2);
                let (res, overflow) = r1.overflowing_sub(r2);

                if overflow {
                    self.registers.set_carry_register(0);
                } else {
                    self.registers.set_carry_register(1);
                }

                self.registers.set_register(reg1, res);
            },
            OpCode::SHR(reg, _) => {
                // Shift right registry
                let r = self.registers.get_register(reg);

                if r & 1 == 1 {
                    self.registers.set_carry_register(1);
                } else {
                    self.registers.set_carry_register(0);
                }

                self.registers.set_register(reg, r >> 1);
            },
            OpCode::SUBN(reg1, reg2) => {
                // SUBN between two registers
                let r1 = self.registers.get_register(reg1);
                let r2 = self.registers.get_register(reg2);
                let (res, overflow) = r2.overflowing_sub(r1);

                if overflow {
                    self.registers.set_carry_register(0);
                } else {
                    self.registers.set_carry_register(1);
                }

                self.registers.set_register(reg1, res);
            },
            OpCode::SHL(reg, _) => {
                // Shift left registry
                let r = self.registers.get_register(reg);
                let msb = 1 << 7;

                if r & msb == msb {
                    self.registers.set_carry_register(1);
                } else {
                    self.registers.set_carry_register(0);
                }

                self.registers.set_register(reg, r << 1);
            },
            OpCode::SNE(reg1, reg2) => {
                // Skip if registers are not equal
                let r1 = self.registers.get_register(reg1);
                let r2 = self.registers.get_register(reg2);

                if r1 != r2 {
                    self.memory.advance_pointer();
                }
            },
            OpCode::LDI(addr) => {
                // Set I to addr
                self.registers.set_i_register(addr);
            },
            OpCode::JP0(addr) => {
                // Set pointer to address + V0
                let v0 = self.registers.get_register(0);
                self.memory.set_pointer(addr + (v0 as C8Addr));
                advance_pointer = false;                
            },
            OpCode::RND(reg, byte) => {
                // Set random value AND byte in register
                let rand_value = random::<C8Byte>() & byte;
                self.registers.set_register(reg, rand_value);
            },
            OpCode::DRW(reg1, reg2, byte) => {
                // Draw sprite
                let r1 = self.registers.get_register(reg1);
                let r2 = self.registers.get_register(reg2);
                let ri = self.registers.get_i_register();

                self.registers.set_carry_register(0);

                for i in 0..r1 {
                    let code = self.memory.read_byte_at_offset(ri + i as C8Addr);
                    
                    let y = (r2 + i) as usize % VIDEO_MEMORY_HEIGHT;
                    let mut shift = FONT_CHAR_HEIGHT - 1;

                    for j in 0..FONT_CHAR_WIDTH {
                        let x = (byte as usize + j) % VIDEO_MEMORY_WIDTH;

                        if code & (0x1 << shift) != 0 {
                            if self.video_memory.toggle_pixel_xy(x, y) {
                                self.registers.set_carry_register(1);
                            }
                        } 

                        shift -= 1;
                    } 
                }
            },
            OpCode::SKP(_reg) => {
                // Skip next instruction if key is pressed
                // TODO
            },
            OpCode::SKNP(_reg) => {
                // Skip next instruction if key is not pressed
                // TODO
            },
            OpCode::LDGetDelayTimer(reg) => {
                // Get delay timer and set register
                let dt = self.delay_timer.get_value();

                self.registers.set_register(reg, dt);
            },
            OpCode::LDGetKey(_reg) => {
                // Wait for key press and stores it in register
                // TODO
            },
            OpCode::LDSetDelayTimer(reg) => {
                // Set delay timer value from registry
                let r = self.registers.get_register(reg);
                self.delay_timer.reset(r);
            },
            OpCode::LDSetSoundTimer(reg) => {
                // Set sound timer value from registry
                let r = self.registers.get_register(reg);
                self.sound_timer.reset(r);
            },
            OpCode::ADDI(reg) => {
                // Add register value to I
                let i = self.registers.get_i_register();
                let r = self.registers.get_register(reg);

                self.registers.set_i_register(i + (r as C8Addr));
            },
            OpCode::LDSprite(reg) => {
                // Set I = location of sprite for reg
                let r = self.registers.get_register(reg) as C8Addr;
                let sprite_addr = (FONT_DATA_ADDR * FONT_CHAR_HEIGHT as C8Addr) * r;

                self.registers.set_i_register(sprite_addr);
            },
            OpCode::LDBCD(reg) => {
                // Store BCD repr of reg in I, I+1, I+2
                let r = self.registers.get_register(reg);
                let i = self.registers.get_i_register();

                let n3 = r / 100;
                let n2 = (r % 100) / 10;
                let n1 = r % 10;

                self.memory.write_data_at_offset(i, &[n3, n2, n1]);
            },
            OpCode::LDS(reg) => {
                // Store registers V0 through reg in memory starting at I
                let ri = self.registers.get_i_register();

                for ridx in 0..reg {
                    let r = self.registers.get_register(ridx);                    
                    self.memory.write_byte_at_offset(ri + ridx as C8Addr, r);
                }
            },
            OpCode::LDR(reg) => {
                // Read registers V0 through reg from memory starting at I
                let ri = self.registers.get_i_register();
                
                for ridx in 0..reg {
                    let byte = self.memory.read_byte_at_offset(ri + ridx as C8Addr);
                    self.registers.set_register(ridx, byte);
                }
            },
            OpCode::NOP => {
                // Nothing
            }
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

        write!(f, "  delay_timer: {:?}\n", self.delay_timer)?;
        write!(f, "  sound_timer: {:?}\n", self.sound_timer)?;

        write!(f, "}}\n")
    }
}