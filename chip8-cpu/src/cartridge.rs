//! CHIP-8 cartridge

use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

use std::env;
use std::path::{Path, PathBuf};

use super::types::{C8Byte, C8Addr};
use super::opcodes::{get_opcode_enum, get_opcode_str, extract_opcode_from_array};
use super::memory::{INITIAL_MEMORY_POINTER};

/// Cartridge max size
const CARTRIDGE_MAX_SIZE: usize = 4096 - 512;

/// CHIP-8 cartridge type
pub struct Cartridge(Vec<C8Byte>);

impl Cartridge {

    /// Load cartridge from path
    /// 
    /// # Arguments
    /// 
    /// * `path` - Path to file
    /// 
    pub fn load_from_games_directory(path: &str) -> Cartridge {
        let games_dir = Cartridge::get_games_directory();
        let mut file = File::open(games_dir.join(path))
                            .expect(&format!("File `{}` not found", path));

        let mut contents = Vec::with_capacity(CARTRIDGE_MAX_SIZE);
        file.read_to_end(&mut contents)
            .expect("Error while reading file contents");

        Cartridge(contents)
    }

    /// Get games directory
    pub fn get_games_directory() -> PathBuf {
        let cargo_path = match env::var("CARGO_MANIFEST_DIR") {
            Ok(path) => path,
            Err(_) => ".".to_string()
        };

        Path::new(&cargo_path).join("games")
    }

    /// Get internal data
    pub fn get_data(&self) -> &[C8Byte] {
        &self.0
    }

    /// Disassemble cartridge
    /// 
    /// Returns a tuple (assembly, verbose)
    /// 
    pub fn disassemble(&self) -> (Vec<C8Addr>, Vec<String>, Vec<String>) {
        let mut code_output = Vec::with_capacity(CARTRIDGE_MAX_SIZE / 2);
        let mut assembly_output = Vec::with_capacity(CARTRIDGE_MAX_SIZE / 2);
        let mut verbose_output = Vec::with_capacity(CARTRIDGE_MAX_SIZE / 2);
        let mut ptr = 0;

        while ptr < self.0.len() {
            let opcode_value = extract_opcode_from_array(&self.0, ptr);
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