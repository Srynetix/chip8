//! Cartridge.

use std::env;
use std::error::Error;
use std::fmt;
use std::fs::metadata;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::core::error::CResult;
use crate::core::opcodes::{
    extract_opcode_from_array, get_opcode_enum, get_opcode_str, is_opcode_schip,
};
use crate::core::types::{C8Addr, C8Byte};

use super::memory::INITIAL_MEMORY_POINTER;

/// Cartridge max size.
pub const CARTRIDGE_MAX_SIZE: usize = 4096 - 512;
/// Empty game name.
pub const EMPTY_GAME_NAME: &str = "<EMPTY>";

/// Cartridge type.
pub struct Cartridge {
    title: String,
    path: String,
    data: Vec<C8Byte>,
}

/// Missing cartridge error.
#[derive(Debug)]
pub struct MissingCartridgeError(String);

impl Error for MissingCartridgeError {
    fn description(&self) -> &str {
        "missing cartridge"
    }
}

impl fmt::Display for MissingCartridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "game cartridge is not found: {}", self.0)
    }
}

impl Cartridge {
    /// New empty cartridge.
    ///
    /// # Returns
    ///
    /// * Cartridge instance.
    ///
    pub fn new_empty() -> Self {
        Self {
            title: String::from(EMPTY_GAME_NAME),
            path: String::from(""),
            data: vec![],
        }
    }

    /// Set data.
    ///
    /// # Arguments
    ///
    /// * `data` - Data
    ///
    pub fn set_data(&mut self, data: Vec<C8Byte>) {
        self.data = data;
    }

    /// Get game name from path.
    ///
    /// # Arguments
    ///
    /// * `path` - Game path.
    ///
    /// # Returns
    ///
    /// * Game name.
    ///
    pub fn get_game_name(path: &Path) -> String {
        match path.file_stem() {
            Some(stem) => stem.to_string_lossy().to_uppercase().replace("_", " "),
            None => String::from(EMPTY_GAME_NAME),
        }
    }

    /// Check game extension.
    ///
    /// # Arguments
    ///
    /// * `path` - Path.
    ///
    /// # Returns
    ///
    /// * `true` if game extension is correct.
    /// * `false` if game extension is incorrect.
    ///
    fn check_game_extension(path: &Path) -> bool {
        // Handle empty path.
        if path.to_string_lossy().is_empty() {
            return false;
        }

        match path.extension() {
            Some(ext) => match ext.to_string_lossy().as_ref() {
                "ch8" | "CH8" => true,
                _ => false,
            },
            None => true,
        }
    }

    /// List games from directory.
    ///
    /// # Returns
    ///
    /// * Game names.
    ///
    pub fn list_from_games_directory() -> Vec<String> {
        let mut res = vec![];
        let game_dir = Self::get_games_directory();

        for entry in walkdir::WalkDir::new(game_dir.to_str().unwrap())
            .into_iter()
            .filter_map(Result::ok)
        {
            // Remove game_dir from entry.
            let mdata = metadata(entry.path()).unwrap();
            if mdata.is_dir() {
                continue;
            }

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
    /// * `path` - Path to file.
    ///
    /// # Returns
    ///
    /// * Cartridge result.
    ///
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> CResult<Cartridge> {
        let mut file = File::open(path.as_ref())?;

        let mut contents = Vec::with_capacity(CARTRIDGE_MAX_SIZE);
        file.read_to_end(&mut contents)?;

        // Strip path.
        let game_name = Self::get_game_name(path.as_ref());
        Cartridge::load_from_string(&game_name, path.as_ref(), &contents)
    }

    /// Save cartridge to path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to file.
    ///
    pub fn save_to_path<P: AsRef<Path>>(&self, path: P) -> CResult<()> {
        let mut file = File::create(path.as_ref())?;

        file.write_all(&self.data)?;
        Ok(())
    }

    /// Load cartridge from bytes.
    ///
    /// # Arguments
    ///
    /// * `title` - Title.
    /// * `path` - Path.
    /// * `bytes` - Bytes contents.
    ///
    /// # Returns
    ///
    /// * Cartridge result.
    ///
    pub fn load_from_string<P: AsRef<Path>>(
        title: &str,
        path: P,
        bytes: &[C8Byte],
    ) -> CResult<Cartridge> {
        let title = title.to_string();
        let data = bytes.to_vec();
        let path = path.as_ref().to_str().unwrap().to_string();

        Ok(Cartridge { title, data, path })
    }

    /// Get games directory.
    ///
    /// # Arguments
    ///
    /// * Games directory.
    ///
    pub fn get_games_directory() -> PathBuf {
        let cargo_path = match env::var("CARGO_MANIFEST_DIR") {
            Ok(path) => path,
            Err(_) => ".".to_string(),
        };

        Path::new(&cargo_path).join("games")
    }

    /// Get cartridge title.
    ///
    /// # Returns
    ///
    /// * Title.
    ///
    pub fn get_title(&self) -> &str {
        &self.title
    }

    /// Get cartridge path.
    ///
    /// # Returns
    ///
    /// * Path.
    ///
    pub fn get_path(&self) -> &str {
        &self.path
    }

    /// Get internal data.
    ///
    /// # Returns
    ///
    /// * Data.
    ///
    pub fn get_data(&self) -> &[C8Byte] {
        &self.data
    }

    /// Disassemble cartridge.
    ///
    /// # Returns
    ///
    /// * Returns a tuple (code, assembly, verbose).
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
    /// * `output_file` - Output stream.
    ///
    pub fn write_disassembly_to_file(&self, output_file: &str) {
        if output_file == "-" {
            println!("disassembly:");
            self.write_disassembly_to_stream(&mut io::stdout());
        } else {
            println!("disassembly dumped to file {}", output_file);
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
    /// * `output_stream` - Output stream.
    ///
    pub fn write_disassembly_to_stream<W: Write>(&self, output_stream: &mut W) {
        let (code, assembly, verbose) = self.disassemble();
        let mut ptr_value = INITIAL_MEMORY_POINTER;

        for i in 0..assembly.len() {
            let schip_chr = if is_opcode_schip(code[i]) { "*" } else { " " };

            writeln!(
                output_stream,
                "{:04X}|{}({:04X})  {:20} ; {}",
                ptr_value, schip_chr, code[i], assembly[i], verbose[i]
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
            "0200| (00E0)  CLS                  ; clearing screen"
        );
        assert_eq!(
            disasm_lines[1],
            "0202| (6300)  LD V3, 00            ; set V3 = 00"
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
        assert!(Cartridge::check_game_extension(Path::new("TEST/TOTO")));
        assert!(!Cartridge::check_game_extension(Path::new("TOTO.c8k")));
        assert!(!Cartridge::check_game_extension(Path::new("TOTO.txt")));
        assert!(!Cartridge::check_game_extension(Path::new("TEST/TOTO.c8k")));
        assert!(!Cartridge::check_game_extension(Path::new("TEST/TOTO.bat")));
        assert!(!Cartridge::check_game_extension(Path::new("")));
    }
}
