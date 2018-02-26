//! CHIP-8 CPU opcodes

use std::collections::HashMap;

use super::types::{C8Byte, C8Short, C8RegIdx, C8Addr};

/// CHIP-8 opcode flag/mask
type OpCodeFlagMask = (C8Short, C8Short);

/// CHIP-8 opcode enum
#[derive(Debug)]
pub enum OpCode {
    
    /// 0nnn - SYS addr
    /// * Jump to a machine code routine at nnn
    /// 
    /// | This instruction is only used on the old computers on which Chip-8 was
    /// | originally implemented.
    /// | It is ignored by modern interpreters.

    SYS(C8Addr),

    /// 00E0 - CLS
    /// * Clear the display
    CLS,

    /// 00EE - RET
    /// * Return from a subroutine
    /// 
    /// | The interpreter sets the program counter to the address at the top
    /// | of the stack, then subtracts 1 from the stack pointer.
    RET,
    
    /// 1nnn - JP addr
    /// * Jump to location nnn
    /// 
    /// | The interpreter sets the program counter to nnn.
    JP(C8Addr),

    /// 2nnn - CALL addr
    /// * Call subroutine at nnn
    /// 
    /// | The interpreter increments the stack pointer,
    /// | then puts the current PC on the top of the stack.
    /// | The PC is then set to nnn.
    CALL(C8Addr),

    /// 3xkk - SE Vx, byte
    /// * Skip next instruction if Vx = kk
    /// 
    /// | The interpreter compares register Vx to kk, and if they are equal,
    /// | increments the program counter by 2.
    SEByte(C8RegIdx, C8Byte),

    /// 4xkk - SNE Vx, byte
    /// * Skip next instruction if Vx != kk
    /// 
    /// | The interpreter compares register Vx to kk, and if they are not equal,
    /// | increments the program counter by 2.
    SNEByte(C8RegIdx, C8Byte),

    /// 5xy0 - SE Vx, Vy
    /// * Skip next instruction if Vx = Vy
    /// 
    /// | The interpreter compares register Vx to register Vy, and if they are
    /// | equal, increments the program counter by 2.
    SE(C8RegIdx, C8RegIdx),

    /// 6xkk - LD Vx, byte
    /// * Set Vx = kk
    /// 
    /// | The interpreter puts the value kk into register Vx.
    LDByte(C8RegIdx, C8Byte),

    /// 7xkk - ADD Vx, byte
    /// * Set Vx = Vx + kk
    /// 
    /// | Adds the value kk to the value of register Vx, then stores the
    /// | result in Vx. 
    ADDByte(C8RegIdx, C8Byte),

    /// 8xy0 - LD Vx, Vy
    /// * Set Vx = Vy
    /// 
    /// | Stores the value of register Vy in register Vx.
    LD(C8RegIdx, C8RegIdx),

    /// 8xy1 - OR Vx, Vy
    /// * Set Vx = Vx OR Vy
    /// 
    /// | Performs a bitwise OR on the values of Vx and Vy, then stores the
    /// | result in Vx.
    /// | A bitwise OR compares the corrseponding bits from two values, and
    /// | if either bit is 1, then the same bit in the result is also 1.
    /// | Otherwise, it is 0. 
    OR(C8RegIdx, C8RegIdx),

    /// 8xy2 - AND Vx, Vy
    /// * Set Vx = Vx AND Vy
    ///
    /// | Performs a bitwise AND on the values of Vx and Vy, then stores the
    /// | result in Vx.
    /// | A bitwise AND compares the corrseponding bits from two values, and if
    /// | both bits are 1, then the same bit in the result is also 1.
    /// | Otherwise, it is 0. 
    AND(C8RegIdx, C8RegIdx),

    /// 8xy3 - XOR Vx, Vy
    /// * Set Vx = Vx XOR Vy
    /// 
    /// | Performs a bitwise exclusive OR on the values of Vx and Vy, then
    /// | stores the result in Vx. An exclusive OR compares the corrseponding bits
    /// | from two values, and if the bits are not both the same, then the
    /// | corresponding bit in the result is set to 1.
    /// | Otherwise, it is 0. 
    XOR(C8RegIdx, C8RegIdx),

    /// 8xy4 - ADD Vx, Vy
    /// * Set Vx = Vx + Vy, set VF = carry
    /// 
    /// | The values of Vx and Vy are added together.
    /// | If the result is greater than 8 bits (i.e., > 255,) VF is set to 1,
    /// | otherwise 0.
    /// | Only the lowest 8 bits of the result are kept, and stored in Vx.
    ADD(C8RegIdx, C8RegIdx),

    /// 8xy5 - SUB Vx, Vy
    /// * Set Vx = Vx - Vy, set VF = NOT borrow
    /// 
    /// | If Vx > Vy, then VF is set to 1, otherwise 0.
    /// | Then Vy is subtracted from Vx, and the results stored in Vx.
    SUB(C8RegIdx, C8RegIdx),

    /// 8xy6 - SHR Vx {, Vy}
    /// * Set Vx = Vx SHR 1
    /// 
    /// | If the least-significant bit of Vx is 1, then VF is set to 1,
    /// | otherwise 0.
    /// | Then Vx is divided by 2.
    SHR(C8RegIdx, C8RegIdx),

    /// 8xy7 - SUBN Vx, Vy
    /// * Set Vx = Vy - Vx, set VF = NOT borrow
    /// 
    /// | If Vy > Vx, then VF is set to 1, otherwise 0.
    /// | Then Vx is subtracted from Vy, and the results stored in Vx.
    SUBN(C8RegIdx, C8RegIdx),

    /// 8xyE - SHL Vx {, Vy}
    /// * Set Vx = Vx SHL 1
    /// 
    /// | If the most-significant bit of Vx is 1, then VF is set to 1,
    /// | otherwise to 0.
    /// | Then Vx is multiplied by 2.
    SHL(C8RegIdx, C8RegIdx),

    /// 9xy0 - SNE Vx, Vy
    /// * Skip next instruction if Vx != Vy
    /// 
    /// | The values of Vx and Vy are compared, and if they are not equal,
    /// | the program counter is increased by 2.
    SNE(C8RegIdx, C8RegIdx),

    /// Annn - LD I, addr
    /// * Set I = nnn
    /// 
    /// | The value of register I is set to nnn.
    LDI(C8Addr),

    /// Bnnn - JP V0, addr
    /// * Jump to location nnn + V0
    /// 
    /// | The program counter is set to nnn plus the value of V0.
    JP0(C8Addr),

    /// Cxkk - RND Vx, byte
    /// * Set Vx = random byte AND kk
    /// 
    /// | The interpreter generates a random number from 0 to 255,
    /// | which is then ANDed with the value kk.
    /// | The results are stored in Vx.
    /// | See instruction 8xy2 for more information on AND.
    RND(C8RegIdx, C8Byte),

    /// Dxyn - DRW Vx, Vy, nibble
    /// * Display n-byte sprite starting at memory location I at (Vx, Vy),
    ///   set VF = collision
    /// 
    /// | The interpreter reads n bytes from memory, starting at the address
    /// | stored in I.
    /// | These bytes are then displayed as sprites on screen at coordinates
    /// | (Vx, Vy).
    /// | Sprites are XORed onto the existing screen.
    /// | If this causes any pixels to be erased, VF is set to 1, otherwise it
    /// | is set to 0.
    /// | If the sprite is positioned so part of it is outside the coordinates
    /// | of the display, it wraps around to the opposite side of the screen.
    /// | See instruction 8xy3 for more information on XOR.
    DRW(C8RegIdx, C8RegIdx, C8Byte),

    /// Ex9E - SKP Vx
    /// * Skip next instruction if key with the value of Vx is pressed
    /// 
    /// | Checks the keyboard, and if the key corresponding to the value of Vx
    /// | is currently in the down position, PC is increased by 2.
    SKP(C8RegIdx),

    /// ExA1 - SKNP Vx
    /// * Skip next instruction if key with the value of Vx is not pressed
    /// 
    /// | Checks the keyboard, and if the key corresponding to the value of Vx
    /// | is currently in the up position, PC is increased by 2.
    SKNP(C8RegIdx),

    /// Fx07 - LD Vx, DT
    /// * Set Vx = delay timer value
    /// 
    /// | The value of DT is placed into Vx.
    LDGetDelayTimer(C8RegIdx),

    /// Fx0A - LD Vx, K
    /// * Wait for a key press, store the value of the key in Vx.
    /// 
    /// | All execution stops until a key is pressed, then the value of that
    /// | key is stored in Vx.
    LDGetKey(C8RegIdx),

    /// Fx15 - LD DT, Vx
    /// * Set delay timer = Vx.
    /// 
    /// | DT is set equal to the value of Vx.
    LDSetDelayTimer(C8RegIdx),

    /// Fx18 - LD ST, Vx
    /// * Set sound timer = Vx.
    /// 
    /// | ST is set equal to the value of Vx.
    LDSetSoundTimer(C8RegIdx),

    /// Fx1E - ADD I, Vx
    /// * Set I = I + Vx
    /// 
    /// | The values of I and Vx are added, and the results are stored in I.
    ADDI(C8RegIdx),

    /// Fx29 - LD F, Vx
    /// * Set I = location of sprite for digit Vx.
    /// 
    /// | The value of I is set to the location for the hexadecimal sprite
    /// | corresponding to the value of Vx.
    LDSprite(C8RegIdx),

    /// Fx33 - LD B, Vx
    /// * Store BCD representation of Vx in memory locations I, I+1, and I+2.
    /// 
    /// | The interpreter takes the decimal value of Vx, and places the hundreds
    /// | digit in memory at location in I, the tens digit at location I+1, and
    /// | the ones digit at location I+2.
    LDBCD(C8RegIdx),

    /// Fx55 - LD [I], Vx
    /// * Store registers V0 through Vx in memory starting at location I.
    /// 
    /// | The interpreter copies the values of registers V0 through Vx into
    /// | memory, starting at the address in I.
    LDS(C8RegIdx),

    /// Fx65 - LD Vx, [I]
    /// * Read registers V0 through Vx from memory starting at location I.
    /// 
    /// | The interpreter reads values from memory starting at location I
    /// | into registers V0 through Vx.
    LDR(C8RegIdx),

    /// 0000 - NOP
    /// * Empty
    NOP
}

lazy_static! {
    static ref OPCODE_FLAG_MASKS: HashMap<C8Short, OpCodeFlagMask> = {
        let mut m = HashMap::new();
        m.insert(0, (0x0FFF, 0x0000));      // 0nnn
        m.insert(1, (0x00E0, 0xFFFF));      // 00E0
        m.insert(2, (0x00EE, 0xFFFF));      // 00EE
        m.insert(3, (0x1000, 0xF000));      // 1nnn
        m.insert(4, (0x2000, 0xF000));      // 2nnn
        m.insert(5, (0x3000, 0xF000));      // 3xkk
        m.insert(6, (0x4000, 0xF000));      // 4xkk
        m.insert(7, (0x5000, 0xF00F));      // 5xy0
        m.insert(8, (0x6000, 0xF000));      // 6xkk
        m.insert(9, (0x7000, 0xF000));      // 7xkk
        m.insert(10, (0x8000, 0xF00F));     // 8xy0
        m.insert(11, (0x8001, 0xF00F));     // 8xy1
        m.insert(12, (0x8002, 0xF00F));     // 8xy2
        m.insert(13, (0x8003, 0xF00F));     // 8xy3
        m.insert(14, (0x8004, 0xF00F));     // 8xy4
        m.insert(15, (0x8005, 0xF00F));     // 8xy5
        m.insert(16, (0x8006, 0xF00F));     // 8xy6
        m.insert(17, (0x8007, 0xF00F));     // 8xy7
        m.insert(18, (0x800E, 0xF00F));     // 8xyE
        m.insert(19, (0x9000, 0xF00F));     // 9xy0
        m.insert(20, (0xA000, 0xF000));     // Annn
        m.insert(21, (0xB000, 0xF000));     // Bnnn
        m.insert(22, (0xC000, 0xF000));     // Cxkk
        m.insert(23, (0xD000, 0xF000));     // Dxyn
        m.insert(24, (0xE09E, 0xF0FF));     // Ex9E
        m.insert(25, (0xE0A1, 0xF0FF));     // ExA1
        m.insert(26, (0xF007, 0xF0FF));     // Fx07
        m.insert(27, (0xF007, 0xF0FF));     // Fx0A
        m.insert(28, (0xF015, 0xF0FF));     // Fx15
        m.insert(29, (0xF018, 0xF0FF));     // Fx18
        m.insert(30, (0xF01E, 0xF0FF));     // Fx1E
        m.insert(31, (0xF029, 0xF0FF));     // Fx29
        m.insert(32, (0xF033, 0xF0FF));     // Fx33
        m.insert(33, (0xF055, 0xF0FF));     // Fx55
        m.insert(34, (0xF065, 0xF0FF));     // Fx65
        m.insert(35, (0x0000, 0xFFFF));     // 0000

        m
    };
}

/// Extract opcode ID
/// 
/// # Arguments
/// 
/// * `opcode` - Opcode value
/// 
fn extract_opcode_id(opcode: C8Short) -> Option<C8Short> {
    let mut extracted_key = None;

    for (key, flag_mask) in OPCODE_FLAG_MASKS.iter() {
        let flag = flag_mask.0;
        let mask = flag_mask.1;

        if mask & opcode == flag {
            extracted_key = Some(*key);
        }
    }

    extracted_key
}

/// Get opcode enum
/// 
/// # Arguments
/// 
/// * `opcode_value` - Opcode value
/// * `action_id` - Action ID
/// 
pub fn get_opcode_enum(opcode: C8Short) -> Option<OpCode> {
    let action_id = extract_opcode_id(opcode);
    if action_id.is_none() {
        println!("`{}` is an unknown opcode", opcode);
        return None;
    }

    let action_id = action_id.unwrap();

    let b3 = ((opcode & 0x0F00) >> 8) as C8Byte; 
    let b2 = ((opcode & 0x00F0) >> 4) as C8Byte; 
    let b1 = (opcode & 0x000F) as C8Byte;

    let addr = ((b3 as C8Addr) << 8) + ((b2 as C8Addr) << 4) + b1 as C8Addr; 
    let kk = (b2 << 4) as C8Byte + b1 as C8Byte;

    let opcode_enum = match action_id {
        0 => OpCode::SYS(addr),
        1 => OpCode::CLS,
        2 => OpCode::RET,
        3 => OpCode::JP(addr),
        4 => OpCode::CALL(addr),
        5 => OpCode::SEByte(b3, kk),
        6 => OpCode::SNEByte(b3, kk),
        7 => OpCode::SE(b3, b2),
        8 => OpCode::LDByte(b3, kk),
        9 => OpCode::ADDByte(b3, kk),
        10 => OpCode::LD(b3, b2),
        11 => OpCode::OR(b3, b2),
        12 => OpCode::AND(b3, b2),
        13 => OpCode::XOR(b3, b2),
        14 => OpCode::ADD(b3, b2),
        15 => OpCode::SUB(b3, b2),
        16 => OpCode::SHR(b3, b2),
        17 => OpCode::SUBN(b3, b2),
        18 => OpCode::SHL(b3, b2),
        19 => OpCode::SNE(b3, b2),
        20 => OpCode::LDI(addr),
        21 => OpCode::JP0(addr),
        22 => OpCode::RND(b3, kk),
        23 => OpCode::DRW(b3, b2, b1),
        24 => OpCode::SKP(b3),
        25 => OpCode::SKNP(b3),
        26 => OpCode::LDGetDelayTimer(b3),
        27 => OpCode::LDGetKey(b3),
        28 => OpCode::LDSetDelayTimer(b3),
        29 => OpCode::LDSetSoundTimer(b3),
        30 => OpCode::ADDI(b3),
        31 => OpCode::LDSprite(b3),
        32 => OpCode::LDBCD(b3),
        33 => OpCode::LDS(b3),
        34 => OpCode::LDR(b3),
        35 => OpCode::NOP,
        _ => {
            println!("OpCode `{}` identified as `{}` but is not implemented", opcode, action_id);
            return None
        }
    };

    Some(opcode_enum)
}

/// Get string output for an opcode.
/// Return a tuple: (assembly, verbose).
/// 
/// # Arguments
/// 
/// * `opcode_enum` - Opcode enum
/// 
pub fn get_opcode_str(opcode_enum: &OpCode) -> (String, String) {
    match *opcode_enum {
        OpCode::SYS(addr) => (format!("SYS {:04X}", addr), format!("Executing system routine at {:04X}", addr)),
        OpCode::CLS => ("CLS".to_string(), "Clearing screen".to_string()),
        OpCode::RET => ("RET".to_string(), "Return from subroutine".to_string()),
        OpCode::JP(addr) => (format!("JP {:04X}", addr), format!("Jumping to address {:04X}", addr)),
        OpCode::CALL(addr) => (format!("CALL {:04X}", addr), format!("Call subroutine at {:04X}", addr)),

        OpCode::SEByte(reg, byte) => (format!("SE V{:X}, {:02X}", reg, byte), format!("Skip next instruction if V{:X} = {:02X}", reg, byte)),
        OpCode::SNEByte(reg, byte) => (format!("SNE V{:X}, {:02X}", reg, byte), format!("Skip next instruction if V{:X} != {:02X}", reg, byte)),
        OpCode::SE(reg1, reg2) => (format!("SE V{:X}, V{:X}", reg1, reg2), format!("Skip next instruction if V{:X} = V{:X}", reg1, reg2)),
        OpCode::LDByte(reg, byte) => (format!("LD V{:X}, {:02X}", reg, byte), format!("Set V{:X} = {:02X}", reg, byte)),
        OpCode::ADDByte(reg, byte) => (format!("ADD V{:X}, {:02X}", reg, byte), format!("Set V{:X} = V{:X} + {:02X}", reg, reg, byte)),
        OpCode::LD(reg1, reg2) => (format!("LD V{:X}, V{:X}", reg1, reg2), format!("Set V{:X} = V{:X}", reg1, reg2)),
        OpCode::OR(reg1, reg2) => (format!("OR V{:X}, V{:X}", reg1, reg2), format!("Set V{:X} = V{:X} OR V{:X}", reg1, reg1, reg2)),
        OpCode::AND(reg1, reg2) => (format!("AND V{:X}, V{:X}", reg1, reg2), format!("Set V{:X} = V{:X} AND V{:X}", reg1, reg1, reg2)),
        OpCode::XOR(reg1, reg2) => (format!("XOR V{:X}, V{:X}", reg1, reg2), format!("Set V{:X} = V{:X} XOR V{:X}", reg1, reg1, reg2)),
        OpCode::ADD(reg1, reg2) => (format!("AND V{:X}, V{:X}", reg1, reg2), format!("Set V{:X} = V{:X} + V{:X}, set VF = carry", reg1, reg1, reg2)),
        OpCode::SUB(reg1, reg2) => (format!("SUB V{:X}, V{:X}", reg1, reg2), format!("Set V{:X} = V{:X} - V{:X}, set VF = NOT borrow", reg1, reg1, reg2)),
        OpCode::SHR(reg, _) => (format!("SHR V{:X}", reg), format!("Set V{:X} = V{:X} SHR 1", reg, reg)),
        OpCode::SUBN(reg1, reg2) => (format!("SUBN V{:X}, V{:X}", reg1, reg2), format!("Set V{:X} = V{:X} - V{:X}, set VF = NOT borrow", reg1, reg2, reg1)),
        OpCode::SHL(reg, _) => (format!("SHL V{:X}", reg), format!("Set V{:X} = V{:X} SHL 1", reg, reg)),
        
        OpCode::SNE(reg1, reg2) => (format!("SNE V{:X}, V{:X}", reg1, reg2), format!("Skip next instruction if V{:X} != V{:X}", reg1, reg2)),
        OpCode::LDI(addr) => (format!("LD I, {:04X}", addr), format!("Set I = {:04X}", addr)),
        OpCode::JP0(addr) => (format!("JP V0, {:04X}", addr), format!("Jump to location {:04X} + V0", addr)),

        OpCode::RND(reg, byte) => (format!("RND V{:X}, {:02X}", reg, byte), format!("Set V{:X} = random byte AND {:02X}", reg, byte)),
        OpCode::DRW(reg1, reg2, byte) => (format!("DRW V{:X}, V{:X}, {:02X}", reg1, reg2, byte), format!("Display sprite {:02X} starting at mem. location I at (V{:X}, V{:X}), set VF = collision", byte, reg1, reg2)),
        
        OpCode::SKP(reg) => (format!("SKP V{:X}", reg), format!("Skip next instruction if key with the value of V{:X} is pressed", reg)),
        OpCode::SKNP(reg) => (format!("SKNP V{:X}", reg), format!("Skip next instruction if key with the value of V{:X} is not pressed", reg)),
        OpCode::LDGetDelayTimer(reg) => (format!("LD V{:X}, DT", reg), format!("Set V{:X} = delay timer value", reg)),
        OpCode::LDGetKey(reg) => (format!("LD V{:X}, K", reg), format!("Wait for a key press, store the value of the key in V{:X}", reg)),
        OpCode::LDSetDelayTimer(reg)
         => (format!("LD DT, V{:X}", reg), format!("Set delay timer = V{:X}", reg)),
        OpCode::LDSetSoundTimer(reg) => (format!("LD ST, V{:X}", reg), format!("Set sound timer = V{:X}", reg)),

        OpCode::ADDI(reg) => (format!("ADD I, V{:X}", reg), format!("Set I = I + V{:X}", reg)),
        OpCode::LDSprite(reg) => (format!("LD F, V{:X}", reg), format!("Set I = location of sprite for digit V{:X}", reg)),
        OpCode::LDBCD(reg) => (format!("LD B, V{:X}", reg), format!("Store BCD representation of V{:X} in memory locations I, I+1 and I+2", reg)),
        OpCode::LDS(reg) => (format!("LD [I], V{:X}", reg), format!("Store registers V0 through V{:X} in memory starting at location I", reg)),
        OpCode::LDR(reg) => (format!("LD V{:X}, [I]", reg), format!("Read registers V0 through V{:X} from memory starting at location I", reg)),

        OpCode::NOP => ("NOP".to_string(), "- Empty".to_string())
    }
}