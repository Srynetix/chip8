//! CHIP-8 CPU

use std::fmt;

use rand::random;

use super::breakpoints::Breakpoints;
use super::cartridge::Cartridge;
use super::font::{Font, FONT_CHAR_HEIGHT, FONT_DATA_ADDR};
use super::opcodes::OpCode;
use super::peripherals::Peripherals;
use super::registers::Registers;
use super::savestate::SaveState;
use super::screen::ScreenMode;
use super::stack::Stack;
use super::timer::Timer;
use super::types::{C8Addr, C8Byte};

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
    pub tracefile: Option<String>,

    /// Save state
    pub savestate: Option<SaveState>,

    /// SCHIP mode
    pub schip_mode: bool,
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

            tracefile: None,
            savestate: None,
            schip_mode: false,
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
        self.peripherals
            .memory
            .write_data_at_offset(FONT_DATA_ADDR, self.font.get_data());
    }

    /// Load savestate
    ///
    /// # Arguments
    ///
    /// * `state` - Save state
    ///
    pub fn load_savestate(&mut self, state: SaveState) {
        self.instruction_count = state.instruction_count;
        self.peripherals.input.load_from_save(state.input_data);
        self.peripherals.memory.load_from_save(state.memory);
        self.peripherals.screen.load_from_save(state.screen_data);
        self.registers.load_from_save(state.registers);
        self.stack.load_from_save(state.stack);
        self.delay_timer.load_from_save(state.delay_timer);
        self.sound_timer.load_from_save(state.sound_timer);
    }

    /// Read cartridge data
    ///
    /// # Arguments
    ///
    /// * `cartridge` - Cartridge
    ///
    pub fn load_cartridge_data(&mut self, cartridge: &Cartridge) {
        self.peripherals.memory.reset_pointer();
        self.peripherals
            .memory
            .write_data_at_pointer(cartridge.get_data());
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

    /// Execute instruction
    ///
    /// # Arguments
    ///
    /// * `opcode` - Execute instruction
    ///
    pub fn execute_instruction(&mut self, opcode: &OpCode) -> bool {
        let mut advance_pointer = true;

        match *opcode {
            OpCode::SYS(_addr) => {
                // Do nothing
            }
            OpCode::CLS => {
                // Clear screen
                self.peripherals.screen.clear_screen();
            }
            OpCode::RET => {
                // Get last stored address
                if self.stack.empty() {
                    println!("END !");
                    return true;
                }

                let addr = self.stack.pop();
                self.peripherals.memory.set_pointer(addr);
            }
            OpCode::JP(addr) => {
                // Set pointer to address
                self.peripherals.memory.set_pointer(addr);
                advance_pointer = false;
            }
            OpCode::CALL(addr) => {
                // Store current address and set pointer
                self.stack.push(self.peripherals.memory.get_pointer());
                self.peripherals.memory.set_pointer(addr);
                advance_pointer = false;
            }
            OpCode::SEByte(reg, byte) => {
                // Compare register with byte and then advance pointer
                let r = self.registers.get_register(reg);

                if r == byte {
                    self.peripherals.memory.advance_pointer();
                }
            }
            OpCode::SNEByte(reg, byte) => {
                // Compare register with byte and then advance pointer
                let r = self.registers.get_register(reg);

                if r != byte {
                    self.peripherals.memory.advance_pointer();
                }
            }
            OpCode::SE(reg1, reg2) => {
                // Compare register values
                let r1 = self.registers.get_register(reg1);
                let r2 = self.registers.get_register(reg2);

                if r1 == r2 {
                    self.peripherals.memory.advance_pointer();
                }
            }
            OpCode::LDByte(reg, byte) => {
                // Puts byte in register
                self.registers.set_register(reg, byte);
            }
            OpCode::ADDByte(reg, byte) => {
                // Add byte in register
                let r = self.registers.get_register(reg);
                let res = r.wrapping_add(byte);

                self.registers.set_register(reg, res);
            }
            OpCode::LD(reg1, reg2) => {
                // Load register value in another
                let r = self.registers.get_register(reg2);

                self.registers.set_register(reg1, r);
            }
            OpCode::OR(reg1, reg2) => {
                // OR between two registers
                let r1 = self.registers.get_register(reg1);
                let r2 = self.registers.get_register(reg2);

                self.registers.set_register(reg1, r1 | r2);
            }
            OpCode::AND(reg1, reg2) => {
                // AND between two registers
                let r1 = self.registers.get_register(reg1);
                let r2 = self.registers.get_register(reg2);

                self.registers.set_register(reg1, r1 & r2)
            }
            OpCode::XOR(reg1, reg2) => {
                // XOR between two registers
                let r1 = self.registers.get_register(reg1);
                let r2 = self.registers.get_register(reg2);

                self.registers.set_register(reg1, r1 ^ r2);
            }
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
            }
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
            }
            OpCode::SHR(reg, _) => {
                // Shift right registry
                let r = self.registers.get_register(reg);

                if r & 1 == 1 {
                    self.registers.set_carry_register(1);
                } else {
                    self.registers.set_carry_register(0);
                }

                self.registers.set_register(reg, r >> 1);
            }
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
            }
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
            }
            OpCode::SNE(reg1, reg2) => {
                // Skip if registers are not equal
                let r1 = self.registers.get_register(reg1);
                let r2 = self.registers.get_register(reg2);

                if r1 != r2 {
                    self.peripherals.memory.advance_pointer();
                }
            }
            OpCode::LDI(addr) => {
                // Set I to addr
                self.registers.set_i_register(addr);
            }
            OpCode::JP0(addr) => {
                // Set pointer to address + V0
                let v0 = self.registers.get_register(0);
                self.peripherals
                    .memory
                    .set_pointer(addr + (C8Addr::from(v0)));
                advance_pointer = false;
            }
            OpCode::RND(reg, byte) => {
                // Set random value AND byte in register
                let rand_value = random::<C8Byte>() & byte;
                self.registers.set_register(reg, rand_value);
            }
            OpCode::DRW(reg1, reg2, byte) => {
                // Draw sprite
                let r1 = self.registers.get_register(reg1);
                let r2 = self.registers.get_register(reg2);
                let ri = self.registers.get_i_register();
                let sprite_data = self
                    .peripherals
                    .memory
                    .read_data_at_offset(ri, C8Addr::from(byte));

                let collision = self.peripherals.screen.draw_sprite(r1, r2, sprite_data);
                self.registers.set_carry_register(collision as C8Byte);
            }
            OpCode::SKP(reg) => {
                // Skip next instruction if key is pressed
                let r = self.registers.get_register(reg);
                let is = self.peripherals.input.get(r);

                if is == 1 {
                    self.peripherals.memory.advance_pointer();
                }
            }
            OpCode::SKNP(reg) => {
                // Skip next instruction if key is not pressed
                let r = self.registers.get_register(reg);
                let is = self.peripherals.input.get(r);

                if is == 0 {
                    self.peripherals.memory.advance_pointer();
                }
            }
            OpCode::LDGetDelayTimer(reg) => {
                // Get delay timer and set register
                let dt = self.delay_timer.get_value();

                self.registers.set_register(reg, dt);
            }
            OpCode::LDGetKey(reg) => {
                let key = self.peripherals.input.wait_for_input();
                self.registers.set_register(reg, key);
            }
            OpCode::LDSetDelayTimer(reg) => {
                // Set delay timer value from registry
                let r = self.registers.get_register(reg);
                self.delay_timer.reset(r);
            }
            OpCode::LDSetSoundTimer(reg) => {
                // Set sound timer value from registry
                let r = self.registers.get_register(reg);
                self.sound_timer.reset(r);
            }
            OpCode::ADDI(reg) => {
                // Add register value to I
                let i = self.registers.get_i_register();
                let r = self.registers.get_register(reg);

                self.registers.set_i_register(i + C8Addr::from(r));
            }
            OpCode::LDSprite(reg) => {
                // Set I = location of sprite for reg
                let r = C8Addr::from(self.registers.get_register(reg));
                let sprite_addr = FONT_DATA_ADDR + (FONT_CHAR_HEIGHT as C8Addr * r);

                self.registers.set_i_register(sprite_addr);
            }
            OpCode::LDBCD(reg) => {
                // Store BCD repr of reg in I, I+1, I+2
                let r = self.registers.get_register(reg);
                let i = self.registers.get_i_register();

                let n3 = r / 100;
                let n2 = (r % 100) / 10;
                let n1 = r % 10;

                self.peripherals
                    .memory
                    .write_data_at_offset(i, &[n3, n2, n1]);
            }
            OpCode::LDS(reg) => {
                // Store registers V0 through reg in memory starting at I
                let ri = self.registers.get_i_register();

                for ridx in 0..=reg {
                    let r = self.registers.get_register(ridx);
                    self.peripherals
                        .memory
                        .write_byte_at_offset(ri + C8Addr::from(ridx), r);
                }
            }
            OpCode::LDR(reg) => {
                // Read registers V0 through reg from memory starting at I
                let ri = self.registers.get_i_register();

                for ridx in 0..=reg {
                    let byte = self
                        .peripherals
                        .memory
                        .read_byte_at_offset(ri + C8Addr::from(ridx));
                    self.registers.set_register(ridx, byte);
                }
            }

            // S-CHIP
            OpCode::SCRD(_b) => {
                println!("Executing SCRD");
            }
            OpCode::SCRR => {
                println!("Executing SCRR");
            }
            OpCode::SCRL => {
                println!("Executing SCRL");
            }
            OpCode::EXIT => {
                println!("Executing EXIT");
            }
            OpCode::LOW => {
                self.peripherals
                    .screen
                    .reload_screen_for_mode(ScreenMode::Standard);
            }
            OpCode::HIGH => {
                self.peripherals
                    .screen
                    .reload_screen_for_mode(ScreenMode::Extended);
            }
            OpCode::DRWX(_reg1, _reg2) => {
                println!("Executing DRWX");
            }
            OpCode::LDXSprite(_reg) => {
                println!("Executing LDXSprite");
            }
            OpCode::LDXS(_reg) => {
                println!("Executing LDXS");
            }
            OpCode::LDXR(_reg) => {
                println!("Executing LDXR");
            }

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

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CPU {{")?;

        writeln!(f, "  memory: {{")?;
        write!(f, "{:?}", self.peripherals.memory)?;
        writeln!(f, "  }},")?;

        writeln!(f, "  screen: {{")?;
        write!(f, "{:?}", self.peripherals.screen)?;
        writeln!(f, "  }},")?;

        writeln!(f, "  registers: {{")?;
        write!(f, "{:?}", self.registers)?;
        writeln!(f, "  }},")?;

        writeln!(f, "  stack: {{")?;
        write!(f, "{:?}", self.stack)?;
        writeln!(f, "  }},")?;

        writeln!(f, "  input: {{")?;
        write!(f, "{:?}", self.peripherals.input)?;
        writeln!(f, "  }},")?;

        writeln!(f, "  delay_timer: {:?},", self.delay_timer)?;
        writeln!(f, "  sound_timer: {:?}", self.sound_timer)?;

        writeln!(f, "}}")
    }
}
