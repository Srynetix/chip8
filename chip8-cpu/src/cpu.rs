//! CHIP-8 CPU

use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::process;

use rand::random;
use time;

use super::types::{C8Byte, C8Addr};
use super::opcodes::{self, OpCode};
use super::font::{Font, FONT_DATA_ADDR, FONT_CHAR_HEIGHT};
use super::cartridge::Cartridge;
use super::timer::Timer;
use super::registers::Registers;
use super::stack::Stack;
use super::breakpoints::Breakpoints;
use super::peripherals::Peripherals;
use super::debugger::{Debugger, Command};

const TIMER_FRAME_LIMIT: u64 = 10;
const CPU_FRAME_LIMIT: u64 = 10;

/// CHIP-8 CPU struct
pub struct CPU {
    /// Peripherals
    pub peripherals: Peripherals,
    /// Breakpoints
    pub breakpoints: Breakpoints,

    /// Registers
    pub registers: Registers,
    /// Stack
    pub stack: Stack,

    /// Syncronization timer
    pub delay_timer: Timer,
    /// Sound timer
    pub sound_timer: Timer,

    /// Font
    pub font: Font,
    /// Instruction count
    pub instruction_count: usize,

    /// Tracefile
    pub tracefile: Option<String>
}

impl CPU {

    /// Create CHIP-8 CPU
    pub fn new() -> Self {
        CPU {
            peripherals: Peripherals::new(),
            breakpoints: Breakpoints::new(),

            registers: Registers::new(),
            stack: Stack::new(),

            delay_timer: Timer::new("Delay".to_string()),
            sound_timer: Timer::new("Sound".to_string()),

            font: Font::new_system_font(),
            instruction_count: 0,

            tracefile: None
        }
    }

    /// Set tracefile
    /// 
    /// # Arguments
    /// 
    /// * `tracefile` - Tracefile output
    /// 
    pub fn tracefile(&mut self, tracefile: &str) {
        self.tracefile = Some(tracefile.to_string());
    }

    /// Show debug data
    pub fn show_debug(&self) {
        println!("{:?}", self);
    }

    /// Load font in memory
    pub fn load_font_in_memory(&mut self) {
        self.peripherals.memory.write_data_at_offset(FONT_DATA_ADDR, self.font.get_data());
    }

    /// Get instruction count
    pub fn get_instruction_count(&self) -> usize {
        self.instruction_count
    }

    /// Read cartridge data
    /// 
    /// # Arguments
    /// 
    /// * `cartridge` - Cartridge
    /// 
    pub fn load_cartridge_data(&mut self, cartridge: &Cartridge) {
        self.peripherals.memory.reset_pointer();
        self.peripherals.memory.write_data_at_pointer(cartridge.get_data());
    }

    /// Decrement timers
    pub fn decrement_timers(&mut self) {
        self.delay_timer.decrement();
        self.sound_timer.decrement();
    }
    
    /// Reset CPU
    pub fn reset(&mut self) {
        // Reset peripherals
        self.peripherals.memory.reset();
        self.peripherals.input.reset();
        self.peripherals.screen.reset();       

        // Reset components
        self.registers.reset();
        self.stack.reset();
        self.delay_timer.reset(0);
        self.sound_timer.reset(0);
    }

    /// Run CPU
    /// 
    /// # Arguments
    /// 
    /// * `cartridge` - Cartridge
    /// 
    pub fn run(&mut self, cartridge: &Cartridge) {
        self.load_font_in_memory();
        self.load_cartridge_data(cartridge);

        let mut last_debugger_command: Option<Command> = None;
        let mut break_next_instruction: Option<C8Addr> = None;
        let mut tracefile_handle = match self.tracefile {
            Some(ref path) => Some(
                            OpenOptions::new()
                                .write(true)
                                .create(true)
                                .open(path)
                                .unwrap()
                            ),
            None => None
        };

        let mut cpu_frames = 0;
        let mut timer_frames = 0;

        loop {
            // Check if CPU should stop
            if self.peripherals.input.should_close {
                break;
            }

            // Check if CPU should reset
            if self.peripherals.input.should_reset {
                // Reset CPU
                self.reset();

                // Reload data
                self.load_font_in_memory();
                self.load_cartridge_data(cartridge);

                // Reset vars
                last_debugger_command = None;
                break_next_instruction = None;
                timer_frames = 0;
                cpu_frames = 0;

                // Restart !
                continue;
            }

            // Read next instruction
            let opcode = self.peripherals.memory.read_opcode();
            trace_exec!(tracefile_handle,
                "[{:08X}] {:04X} - Reading opcode 0x{:04X}...",
                self.instruction_count,
                self.peripherals.memory.get_pointer(),
                opcode
            );

            // Check for breakpoints
            if break_next_instruction.is_none() {
                let pointer = self.peripherals.memory.get_pointer();
                if let Some(_) = self.breakpoints.check_breakpoint(pointer) {
                    break_next_instruction = Some(pointer);
                }
            }

            if let Some(addr) = break_next_instruction {
                trace_exec!(tracefile_handle, "{:?}", self);

                'debugger: loop {
                    {
                        let debugger = Debugger::new(&self, addr);
                        last_debugger_command = debugger.run();
                    }

                    if let Some(ref command) = last_debugger_command {
                        match *command {
                            Command::Quit => process::exit(1),
                            Command::AddBreakpoint(addr) => {
                                println!("Adding breakpoint for address 0x{:04X}", addr);                            
                                self.breakpoints.register(addr);                
                            },
                            Command::RemoveBreakpoint(addr) => {
                                println!("Removing breakpoint for address 0x{:04X}", addr);
                                self.breakpoints.unregister(addr);                                                        
                            },
                            _ => break 'debugger
                        }
                    }
                }
            }

            let opcode_enum = opcodes::get_opcode_enum(opcode);
            let (assembly, verbose) = opcodes::get_opcode_str(&opcode_enum);
            trace_exec!(tracefile_handle, "  - {:20} ; {}", assembly, verbose);

            // Update input state
            if cpu_frames >= CPU_FRAME_LIMIT {
                self.peripherals.input.update_state();
             
                if self.execute_instruction(opcode_enum) {
                    // Should exit
                    break;
                }
             
                self.instruction_count += 1;
            }

            // Handle last debugger command
            if let Some(ref command) = last_debugger_command {
                match *command {
                    Command::Continue => {
                        break_next_instruction = None;
                    },
                    Command::Next => {
                        break_next_instruction = Some(self.peripherals.memory.get_pointer());
                    },
                    _ => {}
                }
            }
            
            // Handle timers
            if timer_frames >= TIMER_FRAME_LIMIT {
                self.decrement_timers();
                timer_frames = 0;
            }

            // Update screen
            self.peripherals.screen.fade_pixels();
            self.peripherals.screen.render();

            cpu_frames += 1;
            timer_frames += 1;      
        }
    }

    /// Execute instruction
    /// 
    /// # Arguments
    /// 
    /// * `opcode` - Execute instruction
    /// 
    pub fn execute_instruction(&mut self, opcode: OpCode) -> bool {
        let mut advance_pointer = true;

        match opcode {
            OpCode::SYS(_addr) => {
                // Do nothing
            },
            OpCode::CLS => {
                // Clear screen
                self.peripherals.screen.clear_screen();
            },
            OpCode::RET => {
                // Get last stored address
                if self.stack.empty() {
                    println!("END !");
                    return true
                }

                let addr = self.stack.pop();
                self.peripherals.memory.set_pointer(addr);
            },
            OpCode::JP(addr) => {
                // Set pointer to address
                self.peripherals.memory.set_pointer(addr);
                advance_pointer = false;
            },
            OpCode::CALL(addr) => {
                // Store current address and set pointer
                self.stack.push(self.peripherals.memory.get_pointer());
                self.peripherals.memory.set_pointer(addr);
                advance_pointer = false;                
            },
            OpCode::SEByte(reg, byte) => {
                // Compare register with byte and then advance pointer
                let r = self.registers.get_register(reg);

                if r == byte {
                    self.peripherals.memory.advance_pointer();
                }
            },
            OpCode::SNEByte(reg, byte) => {
                // Compare register with byte and then advance pointer
                let r = self.registers.get_register(reg);
                
                if r != byte {
                    self.peripherals.memory.advance_pointer();
                }
            },
            OpCode::SE(reg1, reg2) => {
                // Compare register values
                let r1 = self.registers.get_register(reg1);
                let r2 = self.registers.get_register(reg2);

                if r1 == r2 {
                    self.peripherals.memory.advance_pointer();
                }
            },
            OpCode::LDByte(reg, byte) => {
                // Puts byte in register
                self.registers.set_register(reg, byte);
            },
            OpCode::ADDByte(reg, byte) => {
                // Add byte in register
                let r = self.registers.get_register(reg);
                let res = r.wrapping_add(byte);

                self.registers.set_register(reg, res);
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

                self.registers.set_register(reg1, r1 | r2);
            },
            OpCode::AND(reg1, reg2) => {
                // AND between two registers
                let r1 = self.registers.get_register(reg1);
                let r2 = self.registers.get_register(reg2);

                self.registers.set_register(reg1, r1 & r2)
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
                let res = r1.wrapping_sub(r2);

                if r1 > r2 {
                    self.registers.set_carry_register(1);
                } else {
                    self.registers.set_carry_register(0);
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
                let res = r2.wrapping_sub(r1);

                if r2 > r1 {
                    self.registers.set_carry_register(1);
                } else {
                    self.registers.set_carry_register(0);
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
                    self.peripherals.memory.advance_pointer();
                }
            },
            OpCode::LDI(addr) => {
                // Set I to addr
                self.registers.set_i_register(addr);
            },
            OpCode::JP0(addr) => {
                // Set pointer to address + V0
                let v0 = self.registers.get_register(0);
                self.peripherals.memory.set_pointer(addr + (v0 as C8Addr));
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
                let sprite_data = self.peripherals.memory.read_data_at_offset(ri, byte as C8Addr);

                let collision = self.peripherals.screen.draw_sprite(r1, r2, sprite_data);
                self.registers.set_carry_register(collision as C8Byte);
            },
            OpCode::SKP(reg) => {
                // Skip next instruction if key is pressed
                let r = self.registers.get_register(reg);
                let is = self.peripherals.input.get(r);

                if is == 1 {
                    self.peripherals.memory.advance_pointer();
                }

            },
            OpCode::SKNP(reg) => {
                // Skip next instruction if key is not pressed
                let r = self.registers.get_register(reg);
                let is = self.peripherals.input.get(r);

                if is == 0 {
                    self.peripherals.memory.advance_pointer();
                }
            },
            OpCode::LDGetDelayTimer(reg) => {
                // Get delay timer and set register
                let dt = self.delay_timer.get_value();

                self.registers.set_register(reg, dt);
            },
            OpCode::LDGetKey(reg) => {
                let key = self.peripherals.input.wait_for_input();
                self.registers.set_register(reg, key);
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
                let sprite_addr = FONT_DATA_ADDR + (FONT_CHAR_HEIGHT as C8Addr * r);

                self.registers.set_i_register(sprite_addr);
            },
            OpCode::LDBCD(reg) => {
                // Store BCD repr of reg in I, I+1, I+2
                let r = self.registers.get_register(reg);
                let i = self.registers.get_i_register();

                let n3 = r / 100;
                let n2 = (r % 100) / 10;
                let n1 = r % 10;

                self.peripherals.memory.write_data_at_offset(i, &[n3, n2, n1]);
            },
            OpCode::LDS(reg) => {
                // Store registers V0 through reg in memory starting at I
                let ri = self.registers.get_i_register();

                for ridx in 0..(reg + 1) {
                    let r = self.registers.get_register(ridx);                    
                    self.peripherals.memory.write_byte_at_offset(ri + ridx as C8Addr, r);
                }
            },
            OpCode::LDR(reg) => {
                // Read registers V0 through reg from memory starting at I
                let ri = self.registers.get_i_register();
                
                for ridx in 0..(reg + 1) {
                    let byte = self.peripherals.memory.read_byte_at_offset(ri + ridx as C8Addr);
                    self.registers.set_register(ridx, byte);
                }
            },
            OpCode::DATA(_) => {
                // Unknown
            }
        };

        if advance_pointer {
            self.peripherals.memory.advance_pointer();
        }

        false
    }
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CPU {{\n")?;
        
        write!(f, "  memory: {{\n")?;
        write!(f, "{:?}", self.peripherals.memory)?;
        write!(f, "  }},\n")?;

        write!(f, "  screen: {{\n")?;
        write!(f, "{:?}", self.peripherals.screen)?;
        write!(f, "  }},\n")?;
        
        write!(f, "  registers: {{\n")?;
        write!(f, "{:?}", self.registers)?;
        write!(f, "  }},\n")?;
        
        write!(f, "  stack: {{\n")?;
        write!(f, "{:?}", self.stack)?;
        write!(f, "  }},\n")?;

        write!(f, "  input: {{\n")?;
        write!(f, "{:?}", self.peripherals.input)?;
        write!(f, "  }},\n")?;

        write!(f, "  delay_timer: {:?},\n", self.delay_timer)?;
        write!(f, "  sound_timer: {:?}\n", self.sound_timer)?;

        write!(f, "}}\n")
    }
}