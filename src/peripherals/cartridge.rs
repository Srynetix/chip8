//! CHIP-8 cartridge

use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use walkdir;

use crate::core::error::CResult;
use crate::core::opcodes::{extract_opcode_from_array, get_opcode_enum, get_opcode_str};
use crate::core::types::{C8Addr, C8Byte};

use super::memory::INITIAL_MEMORY_POINTER;

/// Cartridge max size
const CARTRIDGE_MAX_SIZE: usize = 4096 - 512;
const EMPTY_GAME_NAME: &str = "<EMPTY>";

/// Available extensions
///
/// - No extension ("")
/// - CH8 extension (.ch8/.CH8)
///
const AVAILABLE_EXTENSIONS: [&str; 5] = ["", "ch8", "CH8", "c8k", "C8K"];

/// CHIP-8 cartridge type
pub struct Cartridge {
    title: String,
    path: String,
    data: Vec<C8Byte>,
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Game cartridge is not found: {}", self.0)
    }
}

impl Cartridge {
    /// New empty cartridge
    pub fn new_empty() -> Self {
        Self {
            title: String::from(EMPTY_GAME_NAME),
            path: String::from(""),
            data: vec![],
        }
    }

    /// Get game path.
    ///
    /// Automatically add extension if not in name.
    /// Supported extensions are: "", "ch8", "CH8"
    ///
    /// # Arguments
    ///
    /// * `name` - Game name
    ///
    fn get_game_path(name: &str) -> Result<String, Box<dyn Error>> {
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

    /// Get game name from path
    pub fn get_game_name(path: &Path) -> String {
        match path.file_stem() {
            Some(stem) => stem.to_string_lossy().to_uppercase().replace("_", " "),
            None => String::from(EMPTY_GAME_NAME),
        }
    }

    /// Check game extension
    fn check_game_extension(path: &Path) -> bool {
        // Handle empty path
        if path.to_string_lossy().is_empty() {
            return false;
        }

        match path.extension() {
            Some(ext) => match ext.to_string_lossy().as_ref() {
                "ch8" | "CH8" | "c8k" | "C8K" => true,
                _ => false,
            },
            None => true,
        }
    }

    /// List games from directory.
    pub fn list_from_games_directory() -> Vec<String> {
        let mut res = vec![];
        let game_dir = Self::get_games_directory();

        for entry in walkdir::WalkDir::new(game_dir.to_str().unwrap())
            .into_iter()
            .filter_map(|e| e.ok())
        {
            // Remove game_dir from entry
            let fname = entry.path().strip_prefix(&game_dir).unwrap();
            if Self::check_game_extension(&fname) {
                res.push(fname.to_string_lossy().into_owned());
            }
        }

        res
    }

    /// Load cartridge from path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to file
    ///
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> CResult<Cartridge> {
        let mut file = File::open(path.as_ref())?;

        let mut contents = Vec::with_capacity(CARTRIDGE_MAX_SIZE);
        file.read_to_end(&mut contents)?;

        // Strip path
        let game_name = Self::get_game_name(path.as_ref());
        Cartridge::load_from_string(&game_name, path.as_ref(), &contents)
    }

    /// Load cartridge from games directory.
    ///
    /// # Arguments
    ///
    /// * `name` - Game name
    ///
    pub fn load_from_games_directory(name: &str) -> CResult<Cartridge> {
        let game_path = Cartridge::get_game_path(name)?;
        let mut file = File::open(&game_path)?;

        let mut contents = Vec::with_capacity(CARTRIDGE_MAX_SIZE);
        file.read_to_end(&mut contents)?;

        // Strip path
        let game_name = Self::get_game_name(Path::new(&game_path));
        Cartridge::load_from_string(&game_name, &game_path, &contents)
    }

    /// Load cartridge from bytes.
    ///
    /// # Arguments
    ///
    /// * `bytes` - Bytes contents
    ///
    pub fn load_from_string<P: AsRef<Path>>(
        title: &str,
        path: P,
        bytes: &[C8Byte],
    ) -> Result<Cartridge, Box<dyn Error>> {
        let title = title.to_string();
        let data = bytes.to_vec();
        let path = path.as_ref().to_str().unwrap().to_string();

        Ok(Cartridge { title, data, path })
    }

    /// Get games directory.
    fn get_games_directory() -> PathBuf {
        let cargo_path = match env::var("CARGO_MANIFEST_DIR") {
            Ok(path) => path,
            Err(_) => ".".to_string(),
        };

        Path::new(&cargo_path).join("games")
    }

    /// Get cartridge title.
    pub fn get_title(&self) -> &str {
        &self.title
    }

    /// Get cartridge path.
    pub fn get_path(&self) -> &str {
        &self.path
    }

    /// Get internal data.
    pub fn get_data(&self) -> &[C8Byte] {
        &self.data
    }

    /// Disassemble cartridge.
    ///
    /// Returns a tuple (code, assembly, verbose).
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

    /// Write disassembly to file.
    ///
    /// If file is '-', print to console.
    ///
    /// # Arguments
    ///
    /// * `output_file` - Output stream
    ///
    pub fn write_disassembly_to_file(&self, output_file: &str) {
        if output_file == "-" {
            println!("> Disassembly:");
            self.write_disassembly_to_stream(&mut io::stdout());
        } else {
            println!("> Disassembly dumped to file {}.", output_file);
            let mut file_handle = OpenOptions::new()
                .create(true)
                .write(true)
                .open(output_file)
                .unwrap();

            self.write_disassembly_to_stream(&mut file_handle);
        }
    }

    /// Write disassembly to stream.
    ///
    /// # Arguments
    ///
    /// * `output_stream` - Output stream
    ///
    pub fn write_disassembly_to_stream<W: Write>(&self, output_stream: &mut W) {
        let (code, assembly, verbose) = self.disassemble();
        let mut ptr_value = INITIAL_MEMORY_POINTER;

        for i in 0..assembly.len() {
            writeln!(
                output_stream,
                "{:04X}| ({:04X})  {:20} ; {}",
                ptr_value, code[i], assembly[i], verbose[i]
            )
            .unwrap();
            ptr_value += 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_from_string() {
        let example: &[C8Byte] = b"\x00\xE0\x63\x00";
        let cartridge = Cartridge::load_from_string("Test", "", example);
        assert!(cartridge.is_ok());

        let cartridge = cartridge.unwrap();
        let mut disasm_raw = Vec::new();
        cartridge.write_disassembly_to_stream(&mut disasm_raw);
        let disasm_str = ::std::str::from_utf8(&disasm_raw).unwrap();
        let disasm_lines: Vec<_> = disasm_str.split("\n").collect();

        assert_eq!(
            disasm_lines[0],
            "0200| (00E0)  CLS                  ; Clearing screen"
        );
        assert_eq!(
            disasm_lines[1],
            "0202| (6300)  LD V3, 00            ; Set V3 = 00"
        );
    }

    #[test]
    fn test_game_list() {
        let game_list = Cartridge::list_from_games_directory();
        assert!(game_list.len() > 0);
    }

    #[test]
    fn test_game_name() {
        assert_eq!(
            Cartridge::get_game_name(Path::new("TOTO.ch8")),
            String::from("TOTO")
        );
        assert_eq!(
            Cartridge::get_game_name(Path::new("TEST/TOTO.ch8")),
            String::from("TOTO")
        );
        assert_eq!(
            Cartridge::get_game_name(Path::new("TEST/TOTO_TUTU.c8k")),
            String::from("TOTO TUTU")
        );
        assert_eq!(
            Cartridge::get_game_name(Path::new("SUPERCHIP/TOTO")),
            String::from("TOTO")
        );
        assert_eq!(
            Cartridge::get_game_name(Path::new("")),
            String::from(EMPTY_GAME_NAME)
        );
    }

    #[test]
    fn test_game_extension() {
        assert!(Cartridge::check_game_extension(Path::new("TOTO.ch8")));
        assert!(Cartridge::check_game_extension(Path::new("TOTO.c8k")));
        assert!(Cartridge::check_game_extension(Path::new("TEST/TOTO.c8k")));
        assert!(Cartridge::check_game_extension(Path::new("TEST/TOTO")));
        assert!(!Cartridge::check_game_extension(Path::new("TOTO.txt")));
        assert!(!Cartridge::check_game_extension(Path::new("TEST/TOTO.bat")));
        assert!(!Cartridge::check_game_extension(Path::new("")));
    }
}
