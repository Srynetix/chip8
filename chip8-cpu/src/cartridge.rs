//! CHIP-8 cartridge

use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::error::Error;
use std::fmt;

use std::env;
use std::path::{Path, PathBuf};

use super::types::{C8Byte, C8Addr};
use super::opcodes::{get_opcode_enum, get_opcode_str, extract_opcode_from_array};
use super::memory::{INITIAL_MEMORY_POINTER};

/// Cartridge max size
const CARTRIDGE_MAX_SIZE: usize = 4096 - 512;

/// Available extensions
const AVAILABLE_EXTENSIONS: [&str; 3] = ["", "ch8", "CH8"];

/// CHIP-8 cartridge type
pub struct Cartridge {
    title: String,
    data: Vec<C8Byte>
}

/// Missing Cartridge error
#[derive(Debug)]
pub struct MissingCartridgeError(String);

impl Error for MissingCartridgeError {
    fn description(&self) -> &str {
        "Missing cartridge"
    }
}

impl fmt::Display for MissingCartridgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Game cartridge is not found: {}", self.0)
    }
}

impl Cartridge {

    /// Get game path.
    /// 
    /// Automatically add extension if not in name.
    /// Supported extensions are: ch8, CH8
    /// 
    /// # Arguments
    /// 
    /// * `name` - Game name
    /// 
    fn get_game_path(name: &str) -> Result<String, Box<Error>> {
        // Concat games directory to path
        let mut game_path = Cartridge::get_games_directory();
        game_path.push(name);

        for ext in &AVAILABLE_EXTENSIONS {
            game_path.set_extension(ext);
            debug!("Searching for game {:?}...", game_path);

            if game_path.exists() {
                return Ok(String::from(game_path.to_str().unwrap()));
            }
        }

        Err(Box::new(MissingCartridgeError(name.to_string())))
    }

    /// Get cartridge title
    pub fn get_title(&self) -> &str {
        &self.title
    }

    /// Load cartridge from path
    /// 
    /// # Arguments
    /// 
    /// * `path` - Path to file
    /// 
    pub fn load_from_games_directory(path: &str) -> Result<Cartridge, Box<Error>> {
        let game_path = Cartridge::get_game_path(path)?;
        let mut file = File::open(game_path)?;

        let mut contents = Vec::with_capacity(CARTRIDGE_MAX_SIZE);
        file.read_to_end(&mut contents)?;

        Ok(
            Cartridge {
                title: path.to_string(),
                data: contents
            }
        )
    }

    /// Get games directory
    fn get_games_directory() -> PathBuf {
        let cargo_path = match env::var("CARGO_MANIFEST_DIR") {
            Ok(path) => path,
            Err(_) => ".".to_string()
        };

        Path::new(&cargo_path).join("games")
    }

    /// Get internal data
    pub fn get_data(&self) -> &[C8Byte] {
        &self.data
    }

    /// Disassemble cartridge
    /// 
    /// Returns a tuple (code, assembly, verbose)
    /// 
    pub fn disassemble(&self) -> (Vec<C8Addr>, Vec<String>, Vec<String>) {
        let mut code_output = Vec::with_capacity(CARTRIDGE_MAX_SIZE / 2);
        let mut assembly_output = Vec::with_capacity(CARTRIDGE_MAX_SIZE / 2);
        let mut verbose_output = Vec::with_capacity(CARTRIDGE_MAX_SIZE / 2);
        let mut ptr = 0;

        while ptr < self.data.len() {
            let opcode_value = extract_opcode_from_array(&self.data, ptr);
            let opcode_enum = get_opcode_enum(opcode_value);

            let (assembly, verbose) = get_opcode_str(&opcode_enum);
            code_output.push(opcode_value);
            assembly_output.push(assembly);
            verbose_output.push(verbose);

            ptr += 2;
        }

        (code_output, assembly_output, verbose_output)
    }

    /// Print disassembly
    /// 
    /// # Arguments
    /// 
    /// * `output_file` - Output file
    /// 
    pub fn print_disassembly(&self, output_file: &str) {
        let (code, assembly, verbose) = self.disassemble();
        let mut ptr_value = INITIAL_MEMORY_POINTER;
            
        if output_file == "-" {
            println!("> Disassembly:");
            for i in 0..assembly.len() {
                println!("{:04X}| ({:04X})  {:20} ; {}", ptr_value, code[i], assembly[i], verbose[i]);
                ptr_value += 2;
            }
        } else {
            println!("> Disassembly dumped to file {}", output_file);
            let mut file_handle = OpenOptions::new()
                                    .create(true)
                                    .write(true)
                                    .open(output_file)
                                    .unwrap();

            for i in 0..assembly.len() {
                writeln!(file_handle, "{:04X}| ({:04X})  {:20} ; {}", ptr_value, code[i], assembly[i], verbose[i]).unwrap();                
                ptr_value += 2;
            }   
        }

        
    }
}