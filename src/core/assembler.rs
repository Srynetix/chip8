//! Assembler.

use std::fs::File;
use std::io::Read;
use std::path::Path;

use regex::Regex;

use crate::core::error::CResult;
use crate::core::opcodes::{BadInstruction, OpCode};
use crate::core::types::{convert_hex_addr, convert_hex_byte, C8Addr, C8Byte, C8RegIdx};
use crate::peripherals::cartridge::{Cartridge, CARTRIDGE_MAX_SIZE};

/// Assembler.
#[derive(Debug, PartialEq)]
pub struct Assembler {
    contents: String,
}

/// Instruction.
#[derive(Debug, PartialEq)]
pub struct Instruction {
    line: Option<C8Addr>,
    opcode: Option<C8Addr>,
    words: String,
    comment: Option<String>,
}

/// Argument token.
#[derive(Debug, PartialEq)]
pub enum ArgToken {
    /// Register.
    Register(C8Byte),
    /// Byte.
    Byte(C8Byte),
    /// Address.
    Address(C8Addr),
    /// Key.
    Key,
    /// Delay timer.
    DelayTimer,
    /// Sound timer.
    SoundTimer,
    /// I pointer.
    IPointer,
    /// I value.
    IValue,
    /// BCD.
    BCD,
    /// Sprite.
    Sprite,
}

impl ArgToken {
    /// To register.
    ///
    /// # Returns
    ///
    /// * Register result.
    ///
    pub fn to_register(&self) -> CResult<C8RegIdx> {
        if let ArgToken::Register(x) = self {
            Ok(*x)
        } else {
            Err(Box::new(BadInstruction("should be a register".to_owned())))
        }
    }

    /// To byte.
    ///
    /// # Returns
    ///
    /// * Byte result.
    ///
    pub fn to_byte(&self) -> CResult<C8Byte> {
        if let ArgToken::Byte(x) = self {
            Ok(*x)
        } else {
            Err(Box::new(BadInstruction("should be a byte".to_owned())))
        }
    }

    /// To address.
    ///
    /// # Returns
    ///
    /// * Address result.
    ///
    pub fn to_address(&self) -> CResult<C8Addr> {
        if let ArgToken::Address(x) = self {
            Ok(*x)
        } else {
            Err(Box::new(BadInstruction("should be an address".to_owned())))
        }
    }
}

fn parse_arg_token(arg: &str) -> CResult<ArgToken> {
    if arg.len() == 4 {
        // Address.
        let addr = convert_hex_addr(arg).unwrap();
        Ok(ArgToken::Address(addr))
    } else if arg.len() == 3 {
        // I value.
        if arg.get(0..3).unwrap() == "[I]" {
            Ok(ArgToken::IValue)
        } else {
            Err(Box::new(BadInstruction("bad instruction".to_owned())))
        }
    } else if arg.len() == 2 {
        if arg.get(0..2).unwrap() == "DT" {
            // Delay timer.
            Ok(ArgToken::DelayTimer)
        } else if arg.get(0..2).unwrap() == "ST" {
            // Sound timer.
            Ok(ArgToken::SoundTimer)
        } else if arg.get(0..1).unwrap() == "V" {
            // Register.
            let reg = convert_hex_byte(arg.get(1..2).unwrap()).unwrap();
            Ok(ArgToken::Register(reg))
        } else {
            // Byte.
            let byte = convert_hex_byte(arg).unwrap();
            Ok(ArgToken::Byte(byte))
        }
    } else if arg.len() == 1 {
        if arg.get(0..1).unwrap() == "I" {
            // I pointer.
            Ok(ArgToken::IPointer)
        } else if arg.get(0..1).unwrap() == "F" {
            // Sprite.
            Ok(ArgToken::Sprite)
        } else if arg.get(0..1).unwrap() == "B" {
            // BCD.
            Ok(ArgToken::BCD)
        } else if arg.get(0..1).unwrap() == "K" {
            // Key.
            Ok(ArgToken::Key)
        } else {
            // Byte.
            let byte = convert_hex_byte(arg).unwrap();
            Ok(ArgToken::Byte(byte))
        }
    } else {
        Err(Box::new(BadInstruction("bad instruction".to_owned())))
    }
}

fn parse_1_arg_token(args: Vec<&str>) -> CResult<ArgToken> {
    let arg = args
        .get(0)
        .ok_or_else(|| BadInstruction("missing argument".to_owned()))?;
    parse_arg_token(arg)
}

fn parse_2_arg_token(args: Vec<&str>) -> CResult<(ArgToken, ArgToken)> {
    let arg1 = args
        .get(0)
        .ok_or_else(|| BadInstruction("missing first argument".to_owned()))?;
    let arg2 = args
        .get(1)
        .ok_or_else(|| BadInstruction("missing second argument".to_owned()))?;

    let v1 = parse_arg_token(arg1)?;
    let v2 = parse_arg_token(arg2)?;

    Ok((v1, v2))
}

fn parse_3_arg_token(args: Vec<&str>) -> CResult<(ArgToken, ArgToken, ArgToken)> {
    let arg1 = args
        .get(0)
        .ok_or_else(|| BadInstruction("missing first argument".to_owned()))?;
    let arg2 = args
        .get(1)
        .ok_or_else(|| BadInstruction("missing second argument".to_owned()))?;
    let arg3 = args
        .get(2)
        .ok_or_else(|| BadInstruction("missing third argument".to_owned()))?;

    let v1 = parse_arg_token(arg1)?;
    let v2 = parse_arg_token(arg2)?;
    let v3 = parse_arg_token(arg3)?;

    Ok((v1, v2, v3))
}

/// Words to opcode.
///
/// # Arguments
///
/// * `words` - Words.
///
/// # Returns
///
/// * Opcode result.
///
pub fn words_to_opcode(words: &str) -> CResult<OpCode> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<opcode>[A-Z]+)( (?P<args>.*))?").unwrap();
    }

    let caps: Vec<_> = RE.captures_iter(words).collect();
    if caps.is_empty() {
        return Err(Box::new(BadInstruction("instruction is empty".to_owned())));
    }

    let cap = &caps[0];
    let word = cap.name("opcode").unwrap().as_str();
    let args = cap
        .name("args")
        .map(|x| x.as_str().split(',').map(str::trim).collect::<Vec<_>>())
        .unwrap_or_else(|| vec![]);

    let opcode = match word {
        "SYS" => {
            let arg = parse_1_arg_token(args)?;
            OpCode::SYS(arg.to_address()?)
        }
        "CLS" => OpCode::CLS,
        "RET" => OpCode::RET,
        "JP" => {
            if args.len() == 1 {
                let arg = parse_1_arg_token(args)?;
                OpCode::JP(arg.to_address()?)
            } else {
                let (_, arg) = parse_2_arg_token(args)?;
                OpCode::JP0(arg.to_address()?)
            }
        }
        "CALL" => {
            let arg = parse_1_arg_token(args)?;
            OpCode::CALL(arg.to_address()?)
        }
        "SE" => {
            let (arg1, arg2) = parse_2_arg_token(args)?;

            if let ArgToken::Byte(x) = arg2 {
                OpCode::SEByte(arg1.to_register()?, x)
            } else {
                OpCode::SE(arg1.to_register()?, arg2.to_register()?)
            }
        }
        "SNE" => {
            let (arg1, arg2) = parse_2_arg_token(args)?;

            if let ArgToken::Byte(x) = arg2 {
                OpCode::SNEByte(arg1.to_register()?, x)
            } else {
                OpCode::SNE(arg1.to_register()?, arg2.to_register()?)
            }
        }
        "ADD" => {
            let (arg1, arg2) = parse_2_arg_token(args)?;

            if let ArgToken::IPointer = arg1 {
                OpCode::ADDI(arg2.to_register()?)
            } else if let ArgToken::Byte(x) = arg2 {
                OpCode::ADDByte(arg1.to_register()?, x)
            } else {
                OpCode::ADD(arg1.to_register()?, arg2.to_register()?)
            }
        }
        "LD" => {
            let (arg1, arg2) = parse_2_arg_token(args)?;

            if let ArgToken::Sprite = arg1 {
                OpCode::LDSprite(arg2.to_register()?)
            } else if let ArgToken::BCD = arg1 {
                OpCode::LDBCD(arg2.to_register()?)
            } else if let ArgToken::IPointer = arg1 {
                OpCode::LDI(arg2.to_address()?)
            } else if let ArgToken::IValue = arg1 {
                OpCode::LDS(arg2.to_register()?)
            } else if let ArgToken::DelayTimer = arg2 {
                OpCode::LDGetDelayTimer(arg1.to_register()?)
            } else if let ArgToken::IValue = arg2 {
                OpCode::LDR(arg1.to_register()?)
            } else if let ArgToken::Byte(x) = arg2 {
                OpCode::LDByte(arg1.to_register()?, x)
            } else if let ArgToken::Key = arg2 {
                OpCode::LDGetKey(arg1.to_register()?)
            } else if let ArgToken::DelayTimer = arg1 {
                OpCode::LDSetDelayTimer(arg2.to_register()?)
            } else if let ArgToken::SoundTimer = arg1 {
                OpCode::LDSetSoundTimer(arg2.to_register()?)
            } else {
                OpCode::LD(arg1.to_register()?, arg2.to_register()?)
            }
        }
        "OR" => {
            let (arg1, arg2) = parse_2_arg_token(args)?;
            OpCode::OR(arg1.to_register()?, arg2.to_register()?)
        }
        "AND" => {
            let (arg1, arg2) = parse_2_arg_token(args)?;
            OpCode::AND(arg1.to_register()?, arg2.to_register()?)
        }
        "XOR" => {
            let (arg1, arg2) = parse_2_arg_token(args)?;
            OpCode::XOR(arg1.to_register()?, arg2.to_register()?)
        }
        "SUB" => {
            let (arg1, arg2) = parse_2_arg_token(args)?;
            OpCode::SUB(arg1.to_register()?, arg2.to_register()?)
        }
        "SUBN" => {
            let (arg1, arg2) = parse_2_arg_token(args)?;
            OpCode::SUBN(arg1.to_register()?, arg2.to_register()?)
        }
        "SHL" => {
            let arg1 = parse_1_arg_token(args)?;
            OpCode::SHL(arg1.to_register()?, 0x0)
        }
        "SHR" => {
            let arg1 = parse_1_arg_token(args)?;
            OpCode::SHR(arg1.to_register()?, 0x0)
        }
        "LDI" => {
            let arg1 = parse_1_arg_token(args)?;
            OpCode::LDI(arg1.to_address()?)
        }
        "RND" => {
            let (arg1, arg2) = parse_2_arg_token(args)?;
            OpCode::RND(arg1.to_register()?, arg2.to_byte()?)
        }
        "DRW" => {
            let (arg1, arg2, arg3) = parse_3_arg_token(args)?;
            OpCode::DRW(arg1.to_register()?, arg2.to_register()?, arg3.to_byte()?)
        }
        "SKP" => {
            let arg = parse_1_arg_token(args)?;
            OpCode::SKP(arg.to_register()?)
        }
        "SKNP" => {
            let arg = parse_1_arg_token(args)?;
            OpCode::SKNP(arg.to_register()?)
        }
        "SCRD" => {
            let arg = parse_1_arg_token(args)?;
            OpCode::SCRD(arg.to_byte()?)
        }
        "SCRR" => OpCode::SCRR,
        "SCRL" => OpCode::SCRL,
        "EXIT" => OpCode::EXIT,
        "LOW" => OpCode::LOW,
        "HIGH" => OpCode::HIGH,
        "DRWX" => {
            let (arg1, arg2) = parse_2_arg_token(args)?;
            OpCode::DRWX(arg1.to_register()?, arg2.to_register()?)
        }
        "LDX" => {
            let (arg1, arg2) = parse_2_arg_token(args)?;
            if let ArgToken::Sprite = arg1 {
                // LDXSprite.
                OpCode::LDXSprite(arg2.to_register()?)
            } else if let ArgToken::IValue = arg1 {
                // LDXS.
                OpCode::LDXS(arg2.to_register()?)
            } else if let ArgToken::IValue = arg2 {
                // LDXR.
                OpCode::LDXR(arg1.to_register()?)
            } else {
                return Err(Box::new(BadInstruction(format!(
                    "unknown LDX instruction: {}",
                    words
                ))));
            }
        }
        "EMPTY" => OpCode::EMPTY,
        "DATA" => {
            let arg = parse_1_arg_token(args)?;
            OpCode::DATA(arg.to_address()?)
        }
        _ => {
            return Err(Box::new(BadInstruction(format!(
                "unknown instruction: {}",
                words
            ))));
        }
    };

    Ok(opcode)
}

fn convert_addr(base: C8Addr, addr: C8Addr) -> C8Addr {
    base + addr
}

fn convert_reg_byte(base: C8Addr, reg: C8RegIdx, byte: C8Byte) -> C8Addr {
    base + C8Addr::from(reg).wrapping_shl(8) + C8Addr::from(byte)
}

fn convert_reg1_reg2(base: C8Addr, reg1: C8RegIdx, reg2: C8RegIdx) -> C8Addr {
    base + C8Addr::from(reg1).wrapping_shl(8) + C8Addr::from(reg2).wrapping_shl(4)
}

fn convert_reg1_reg2_byte(base: C8Addr, reg1: C8RegIdx, reg2: C8RegIdx, byte: C8Byte) -> C8Addr {
    base + C8Addr::from(reg1).wrapping_shl(8)
        + C8Addr::from(reg2).wrapping_shl(4)
        + C8Addr::from(byte)
}

fn convert_reg(base: C8Addr, reg: C8RegIdx) -> C8Addr {
    base + C8Addr::from(reg).wrapping_shl(8)
}

/// Convert opcode enum to address.
///
/// # Arguments
///
/// * `opcode` - Opcode enum.
///
/// # Returns
///
/// * Address.
///
pub fn opcode_enum_to_addr(opcode: OpCode) -> C8Addr {
    match opcode {
        OpCode::SYS(addr) => convert_addr(0x0000, addr),
        OpCode::CLS => 0x00E0,
        OpCode::RET => 0x00EE,
        OpCode::JP(addr) => convert_addr(0x1000, addr),
        OpCode::CALL(addr) => convert_addr(0x2000, addr),
        OpCode::SEByte(reg, byte) => convert_reg_byte(0x3000, reg, byte),
        OpCode::SNEByte(reg, byte) => convert_reg_byte(0x4000, reg, byte),
        OpCode::SE(reg1, reg2) => convert_reg1_reg2(0x5000, reg1, reg2),
        OpCode::LDByte(reg, byte) => convert_reg_byte(0x6000, reg, byte),
        OpCode::ADDByte(reg, byte) => convert_reg_byte(0x7000, reg, byte),
        OpCode::LD(reg1, reg2) => convert_reg1_reg2(0x8000, reg1, reg2),
        OpCode::OR(reg1, reg2) => convert_reg1_reg2(0x8001, reg1, reg2),
        OpCode::AND(reg1, reg2) => convert_reg1_reg2(0x8002, reg1, reg2),
        OpCode::XOR(reg1, reg2) => convert_reg1_reg2(0x8003, reg1, reg2),
        OpCode::ADD(reg1, reg2) => convert_reg1_reg2(0x8004, reg1, reg2),
        OpCode::SUB(reg1, reg2) => convert_reg1_reg2(0x8005, reg1, reg2),
        OpCode::SHR(reg1, reg2) => convert_reg1_reg2(0x8006, reg1, reg2),
        OpCode::SUBN(reg1, reg2) => convert_reg1_reg2(0x8007, reg1, reg2),
        OpCode::SHL(reg1, reg2) => convert_reg1_reg2(0x800E, reg1, reg2),
        OpCode::SNE(reg1, reg2) => convert_reg1_reg2(0x9000, reg1, reg2),
        OpCode::LDI(addr) => convert_addr(0xA000, addr),
        OpCode::JP0(addr) => convert_addr(0xB000, addr),
        OpCode::RND(reg, byte) => convert_reg_byte(0xC000, reg, byte),
        OpCode::DRW(reg1, reg2, byte) => convert_reg1_reg2_byte(0xD000, reg1, reg2, byte),
        OpCode::SKP(reg) => convert_reg(0xE09E, reg),
        OpCode::SKNP(reg) => convert_reg(0xE0A1, reg),
        OpCode::LDGetDelayTimer(reg) => convert_reg(0xF007, reg),
        OpCode::LDGetKey(reg) => convert_reg(0xF00A, reg),
        OpCode::LDSetDelayTimer(reg) => convert_reg(0xF015, reg),
        OpCode::LDSetSoundTimer(reg) => convert_reg(0xF018, reg),
        OpCode::ADDI(reg) => convert_reg(0xF01E, reg),
        OpCode::LDSprite(reg) => convert_reg(0xF029, reg),
        OpCode::LDBCD(reg) => convert_reg(0xF033, reg),
        OpCode::LDS(reg) => convert_reg(0xF055, reg),
        OpCode::LDR(reg) => convert_reg(0xF065, reg),
        OpCode::SCRD(reg) => 0x00C0 + C8Addr::from(reg),
        OpCode::SCRR => 0x00FB,
        OpCode::SCRL => 0x00FC,
        OpCode::EXIT => 0x00FD,
        OpCode::LOW => 0x00FE,
        OpCode::HIGH => 0x00FF,
        OpCode::DRWX(reg1, reg2) => convert_reg1_reg2(0xD000, reg1, reg2),
        OpCode::LDXSprite(reg) => convert_reg(0xF030, reg),
        OpCode::LDXS(reg) => convert_reg(0xF075, reg),
        OpCode::LDXR(reg) => convert_reg(0xF085, reg),
        OpCode::EMPTY => 0x0000,
        OpCode::DATA(addr) => addr,
    }
}

/// Resolve instruction.
///
/// # Arguments
///
/// * `words` - Words.
///
/// # Returns
///
/// * Address result.
///
pub fn resolve_instruction(words: &str) -> CResult<C8Addr> {
    let opcode_enum = words_to_opcode(words)?;
    Ok(opcode_enum_to_addr(opcode_enum))
}

impl Instruction {
    /// Resolve instruction.
    ///
    /// # Arguments
    ///
    /// * `words` - Words.
    ///
    /// # Returns
    ///
    /// * Address result.
    ///
    pub fn resolve(&self) -> CResult<C8Addr> {
        resolve_instruction(&self.words)
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self {
            contents: "".to_owned(),
        }
    }
}

impl Assembler {
    /// Creates new assembler.
    ///
    /// # Returns
    ///
    /// * Assembler instance.
    ///
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates new assembler from path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path.
    ///
    /// # Returns
    ///
    /// * Assembler result.
    ///
    pub fn from_path<P: AsRef<Path>>(path: P) -> CResult<Self> {
        let mut file = File::open(path.as_ref())?;

        debug!("reading assembler code from {:?}", path.as_ref());
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(Self::from_string(&contents))
    }

    /// Creates new assembler from string contents.
    ///
    /// # Arguments
    ///
    /// * `contents` - String contents.
    ///
    /// # Returns
    ///
    /// * Assembler result.
    ///
    pub fn from_string(contents: &str) -> Self {
        Self {
            contents: contents.to_owned(),
        }
    }

    /// Assemble line to address.
    ///
    /// If the line is not an opcode, None will be returned.
    ///
    /// # Arguments
    ///
    /// * `line` - Line
    ///
    /// # Returns
    ///
    /// * Opcode option.
    ///
    pub fn assemble_line_from_str(&self, line: &str) -> Option<Instruction> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"((?P<line>[0-9A-Z]{4})\|)?([ *]?\((?P<opcode>[0-9A-Z]{4})\) )? ?((?P<instr>[A-Z0-9, \[\]]+))?(;(?P<comment>.*))?").unwrap();
        }

        let caps: Vec<_> = RE.captures_iter(line).collect();
        if caps.is_empty() {
            return None;
        }

        // Get capture.
        let cap = &caps[0];
        let line = cap
            .name("line")
            .map(|c| convert_hex_addr(c.as_str()).unwrap());
        let opcode = cap
            .name("opcode")
            .map(|c| convert_hex_addr(c.as_str()).unwrap());
        let words = cap.name("instr").map(|c| c.as_str().trim().to_owned())?;
        let comment = cap.name("comment").map(|c| c.as_str().trim().to_owned());

        Some(Instruction {
            line,
            opcode,
            words,
            comment,
        })
    }

    /// Assemble cartridge data.
    ///
    /// # Arguments
    ///
    /// * `contents` - Contents.
    ///
    /// # Returns
    ///
    /// * Byte vector result.
    ///
    pub fn assemble_data(&self) -> CResult<Vec<C8Byte>> {
        // Generate instructions.
        debug!("assembling instructions ...");
        let mut data: Vec<C8Byte> = Vec::with_capacity(CARTRIDGE_MAX_SIZE);
        for line in self.contents.split('\n') {
            let instruction = self.assemble_line_from_str(line);
            if let Some(x) = instruction {
                let code = x.resolve()?;
                let b1 = ((0xFF00 & code) >> 8) as C8Byte;
                let b2 = (0x00FF & code) as C8Byte;
                data.push(b1);
                data.push(b2);
            }
        }
        debug!("{} instructions assembled", data.len());

        Ok(data)
    }

    /// Assemble cartridge from a string.
    ///
    /// # Arguments
    ///
    /// * `contents` - Contents.
    ///
    /// # Returns
    ///
    /// * Cartridge result.
    ///
    pub fn assemble_cartridge(&self) -> CResult<Cartridge> {
        // Generate data.
        let data = self.assemble_data()?;
        let mut cartridge = Cartridge::new_empty();
        cartridge.set_data(data);

        Ok(cartridge)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assemble_from_str() {
        let example = "JP 020E\nJP 010A";
        let assembler = Assembler::from_string(example);
        let data = assembler.assemble_data().unwrap();

        assert_eq!(data, vec![0x12, 0x0E, 0x11, 0x0A]);
    }

    #[test]
    fn test_assemble_line_from_str() {
        let full_example = r#"0200| (120E)  JP 020E           ; jumping to address 020E"#;
        let lite_example = r#"0200| JP 020E            ; jumping to address 020E"#;
        let small_example = r#"JP 020E     ;   test"#;
        let minimal_example = r#"JP 020E"#;
        let comment_example = r#"; toto"#;
        let comment2_example = r#";   toto"#;

        let assembler = Assembler::new();
        assert_eq!(
            assembler.assemble_line_from_str(full_example),
            Some(Instruction {
                line: Some(0x0200),
                opcode: Some(0x120E),
                words: "JP 020E".to_owned(),
                comment: Some("jumping to address 020E".to_owned())
            })
        );
        assert_eq!(
            assembler.assemble_line_from_str(lite_example),
            Some(Instruction {
                line: Some(0x0200),
                opcode: None,
                words: "JP 020E".to_owned(),
                comment: Some("jumping to address 020E".to_owned())
            })
        );
        assert_eq!(
            assembler.assemble_line_from_str(small_example),
            Some(Instruction {
                line: None,
                opcode: None,
                words: "JP 020E".to_owned(),
                comment: Some("test".to_owned())
            })
        );
        assert_eq!(
            assembler.assemble_line_from_str(minimal_example),
            Some(Instruction {
                line: None,
                opcode: None,
                words: "JP 020E".to_owned(),
                comment: None
            })
        );
        assert_eq!(assembler.assemble_line_from_str(comment_example), None);
        assert_eq!(assembler.assemble_line_from_str(comment2_example), None);

        let test_str = "0254| (F065)  LD V0, [I]           ; read registers V0 through V0 from memory starting at location I";
        assert_eq!(
            assembler.assemble_line_from_str(test_str),
            Some(Instruction {
                line: Some(0x0254),
                opcode: Some(0xF065),
                words: "LD V0, [I]".to_owned(),
                comment: Some(
                    "read registers V0 through V0 from memory starting at location I".to_owned()
                )
            })
        );

        assert_eq!(
            assembler.assemble_line_from_str("0000|*(F065)  LD V0, [I] ; toto"),
            Some(Instruction {
                line: Some(0x0000),
                opcode: Some(0xF065),
                words: "LD V0, [I]".to_owned(),
                comment: Some("toto".to_owned())
            })
        );
    }

    #[test]
    fn test_instruction_resolution() {
        assert_eq!(resolve_instruction("JP 020E").unwrap(), 0x120E);
        assert!(resolve_instruction("JPP 020E").is_err());

        let inst = Instruction {
            line: None,
            opcode: None,
            words: "JP 020E".to_owned(),
            comment: None,
        };

        assert_eq!(inst.resolve().unwrap(), 0x120E);

        let inst = Instruction {
            line: None,
            opcode: None,
            words: "SCRD 1".to_owned(),
            comment: None,
        };
        assert_eq!(inst.resolve().unwrap(), 0x00C1);
    }

    #[test]
    fn test_words_to_opcode() {
        assert!(words_to_opcode("SYS").is_err());
        assert_eq!(words_to_opcode("SYS 0201").unwrap(), OpCode::SYS(0x0201));
        assert_eq!(words_to_opcode("CLS").unwrap(), OpCode::CLS);
        assert_eq!(words_to_opcode("RET").unwrap(), OpCode::RET);
        assert_eq!(words_to_opcode("CALL 0201").unwrap(), OpCode::CALL(0x0201));

        assert_eq!(words_to_opcode("SE V1, V2").unwrap(), OpCode::SE(0x1, 0x2));
        assert_eq!(
            words_to_opcode("SE V1, 56").unwrap(),
            OpCode::SEByte(0x1, 0x56)
        );
        assert_eq!(
            words_to_opcode("SNE V1, V2").unwrap(),
            OpCode::SNE(0x1, 0x2)
        );
        assert_eq!(
            words_to_opcode("SNE V1, 56").unwrap(),
            OpCode::SNEByte(0x1, 0x56)
        );
        assert_eq!(words_to_opcode("LD V1, V2").unwrap(), OpCode::LD(0x1, 0x2));
        assert_eq!(
            words_to_opcode("LD V1, 56").unwrap(),
            OpCode::LDByte(0x1, 0x56)
        );
        assert_eq!(
            words_to_opcode("ADD V1, V2").unwrap(),
            OpCode::ADD(0x1, 0x2)
        );
        assert_eq!(
            words_to_opcode("ADD V1, 56").unwrap(),
            OpCode::ADDByte(0x1, 0x56)
        );
        assert_eq!(
            words_to_opcode("SUB V1, V2").unwrap(),
            OpCode::SUB(0x1, 0x2)
        );
        assert_eq!(
            words_to_opcode("SUBN V1, V2").unwrap(),
            OpCode::SUBN(0x1, 0x2)
        );
        assert_eq!(words_to_opcode("OR V1, V2").unwrap(), OpCode::OR(0x1, 0x2));
        assert_eq!(
            words_to_opcode("AND V1, V2").unwrap(),
            OpCode::AND(0x1, 0x2)
        );
        assert_eq!(
            words_to_opcode("XOR V1, V2").unwrap(),
            OpCode::XOR(0x1, 0x2)
        );
        assert_eq!(words_to_opcode("SHR V1").unwrap(), OpCode::SHR(0x1, 0x0));
        assert_eq!(words_to_opcode("SHL V1").unwrap(), OpCode::SHL(0x1, 0x0));
        assert_eq!(
            words_to_opcode("SNE V1, V2").unwrap(),
            OpCode::SNE(0x1, 0x2)
        );
        assert_eq!(words_to_opcode("LDI 0202").unwrap(), OpCode::LDI(0x0202));
        assert_eq!(words_to_opcode("JP V0, 0202").unwrap(), OpCode::JP0(0x0202));

        assert_eq!(
            words_to_opcode("RND V1, 56").unwrap(),
            OpCode::RND(0x1, 0x56)
        );
        assert_eq!(
            words_to_opcode("DRW V1, V2, 56").unwrap(),
            OpCode::DRW(0x1, 0x2, 0x56)
        );
        assert_eq!(words_to_opcode("SKP V1").unwrap(), OpCode::SKP(0x1));
        assert_eq!(words_to_opcode("SKNP V1").unwrap(), OpCode::SKNP(0x1));
        assert_eq!(
            words_to_opcode("LD V1, DT").unwrap(),
            OpCode::LDGetDelayTimer(0x1)
        );
        assert_eq!(words_to_opcode("LD V1, K").unwrap(), OpCode::LDGetKey(0x1));
        assert_eq!(
            words_to_opcode("LD DT, V1").unwrap(),
            OpCode::LDSetDelayTimer(0x1)
        );
        assert_eq!(
            words_to_opcode("LD ST, V1").unwrap(),
            OpCode::LDSetSoundTimer(0x1)
        );

        assert_eq!(words_to_opcode("ADD I, V1").unwrap(), OpCode::ADDI(0x1));
        assert_eq!(words_to_opcode("LD F, V1").unwrap(), OpCode::LDSprite(0x1));
        assert_eq!(words_to_opcode("LD B, V1").unwrap(), OpCode::LDBCD(0x1));
        assert_eq!(words_to_opcode("LD [I], V1").unwrap(), OpCode::LDS(0x1));
        assert_eq!(words_to_opcode("LD V1, [I]").unwrap(), OpCode::LDR(0x1));

        assert_eq!(words_to_opcode("SCRD 56").unwrap(), OpCode::SCRD(0x56));
        assert_eq!(words_to_opcode("SCRR").unwrap(), OpCode::SCRR);
        assert_eq!(words_to_opcode("SCRL").unwrap(), OpCode::SCRL);
        assert_eq!(words_to_opcode("EXIT").unwrap(), OpCode::EXIT);
        assert_eq!(words_to_opcode("LOW").unwrap(), OpCode::LOW);
        assert_eq!(words_to_opcode("HIGH").unwrap(), OpCode::HIGH);
        assert_eq!(
            words_to_opcode("DRWX V1, V2").unwrap(),
            OpCode::DRWX(0x1, 0x2)
        );
        assert_eq!(
            words_to_opcode("LDX F, V1").unwrap(),
            OpCode::LDXSprite(0x1)
        );
        assert_eq!(words_to_opcode("LDX [I], V1").unwrap(), OpCode::LDXS(0x1));
        assert_eq!(words_to_opcode("LDX V1, [I]").unwrap(), OpCode::LDXR(0x1));

        assert_eq!(words_to_opcode("EMPTY").unwrap(), OpCode::EMPTY);
        assert_eq!(words_to_opcode("DATA 0202").unwrap(), OpCode::DATA(0x0202));
    }

    #[test]
    fn test_opcode_enum_to_addr() {
        assert_eq!(opcode_enum_to_addr(OpCode::SYS(0x020E)), 0x020E);
        assert_eq!(opcode_enum_to_addr(OpCode::CLS), 0x00E0);
        assert_eq!(opcode_enum_to_addr(OpCode::RET), 0x00EE);
        assert_eq!(opcode_enum_to_addr(OpCode::JP(0x020E)), 0x120E);
        assert_eq!(opcode_enum_to_addr(OpCode::CALL(0x020E)), 0x220E);
        assert_eq!(opcode_enum_to_addr(OpCode::SEByte(0x1, 0x56)), 0x3156);
        assert_eq!(opcode_enum_to_addr(OpCode::SNEByte(0x1, 0x56)), 0x4156);
        assert_eq!(opcode_enum_to_addr(OpCode::SE(0x1, 0x2)), 0x5120);
        assert_eq!(opcode_enum_to_addr(OpCode::LDByte(0x1, 0x56)), 0x6156);
        assert_eq!(opcode_enum_to_addr(OpCode::ADDByte(0x1, 0x56)), 0x7156);
        assert_eq!(opcode_enum_to_addr(OpCode::LD(0x1, 0x2)), 0x8120);
        assert_eq!(opcode_enum_to_addr(OpCode::OR(0x1, 0x2)), 0x8121);
        assert_eq!(opcode_enum_to_addr(OpCode::AND(0x1, 0x2)), 0x8122);
        assert_eq!(opcode_enum_to_addr(OpCode::XOR(0x1, 0x2)), 0x8123);
        assert_eq!(opcode_enum_to_addr(OpCode::ADD(0x1, 0x2)), 0x8124);
        assert_eq!(opcode_enum_to_addr(OpCode::SUB(0x1, 0x2)), 0x8125);
        assert_eq!(opcode_enum_to_addr(OpCode::SHR(0x1, 0x2)), 0x8126);
        assert_eq!(opcode_enum_to_addr(OpCode::SUBN(0x1, 0x2)), 0x8127);
        assert_eq!(opcode_enum_to_addr(OpCode::SHL(0x1, 0x2)), 0x812E);
        assert_eq!(opcode_enum_to_addr(OpCode::SNE(0x1, 0x2)), 0x9120);
        assert_eq!(opcode_enum_to_addr(OpCode::LDI(0x020E)), 0xA20E);
        assert_eq!(opcode_enum_to_addr(OpCode::JP0(0x020E)), 0xB20E);
        assert_eq!(opcode_enum_to_addr(OpCode::RND(0x1, 0x56)), 0xC156);
        assert_eq!(opcode_enum_to_addr(OpCode::DRW(0x1, 0x2, 0x8)), 0xD128);
        assert_eq!(opcode_enum_to_addr(OpCode::SKP(0x1)), 0xE19E);
        assert_eq!(opcode_enum_to_addr(OpCode::SKNP(0x1)), 0xE1A1);
        assert_eq!(opcode_enum_to_addr(OpCode::LDGetDelayTimer(0x1)), 0xF107);
        assert_eq!(opcode_enum_to_addr(OpCode::LDGetKey(0x1)), 0xF10A);
        assert_eq!(opcode_enum_to_addr(OpCode::LDSetDelayTimer(0x1)), 0xF115);
        assert_eq!(opcode_enum_to_addr(OpCode::LDSetSoundTimer(0x1)), 0xF118);
        assert_eq!(opcode_enum_to_addr(OpCode::ADDI(0x1)), 0xF11E);
        assert_eq!(opcode_enum_to_addr(OpCode::LDSprite(0x1)), 0xF129);
        assert_eq!(opcode_enum_to_addr(OpCode::LDBCD(0x1)), 0xF133);
        assert_eq!(opcode_enum_to_addr(OpCode::LDS(0x1)), 0xF155);
        assert_eq!(opcode_enum_to_addr(OpCode::LDR(0x1)), 0xF165);
        assert_eq!(opcode_enum_to_addr(OpCode::SCRD(0x1)), 0x00C1);
        assert_eq!(opcode_enum_to_addr(OpCode::SCRR), 0x00FB);
        assert_eq!(opcode_enum_to_addr(OpCode::SCRL), 0x00FC);
        assert_eq!(opcode_enum_to_addr(OpCode::EXIT), 0x00FD);
        assert_eq!(opcode_enum_to_addr(OpCode::LOW), 0x00FE);
        assert_eq!(opcode_enum_to_addr(OpCode::HIGH), 0x00FF);
        assert_eq!(opcode_enum_to_addr(OpCode::DRWX(0x1, 0x2)), 0xD120);
        assert_eq!(opcode_enum_to_addr(OpCode::LDXSprite(0x1)), 0xF130);
        assert_eq!(opcode_enum_to_addr(OpCode::LDXS(0x1)), 0xF175);
        assert_eq!(opcode_enum_to_addr(OpCode::LDXR(0x1)), 0xF185);
        assert_eq!(opcode_enum_to_addr(OpCode::EMPTY), 0x0000);
        assert_eq!(opcode_enum_to_addr(OpCode::DATA(0x9999)), 0x9999);
    }
}
