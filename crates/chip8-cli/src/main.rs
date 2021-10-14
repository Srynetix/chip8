//! CHIP-8 shell.

use std::{path::PathBuf, process};

use argh::FromArgs;
use chip8_core::{
    core::assembler::Assembler,
    debugger::{Debugger, DebuggerContext},
    drivers::WindowInterface,
    emulator::{Emulator, EmulatorContext},
    errors::CResult,
    peripherals::{cartridge::Cartridge, memory::INITIAL_MEMORY_POINTER},
};
use chip8_drivers::MQWindowDriver;

/// CHIP-8 emulator
#[derive(FromArgs)]
pub struct Args {
    /// verbose mode
    #[argh(switch, short = 'v')]
    pub verbose: bool,

    /// subcommand
    #[argh(subcommand)]
    pub nested: SubCommands,
}

/// Subcommands
#[derive(FromArgs)]
#[argh(subcommand)]
pub enum SubCommands {
    /// Play command
    Play(PlayCommand),
    /// Debug command
    Debug(DebugCommand),
    /// Assemble command
    Assemble(AssembleCommand),
    /// Disassemble command
    Disassemble(DisassembleCommand),
    /// Version command
    Version(VersionCommand),
}

/// play cartridge
#[derive(FromArgs)]
#[argh(subcommand, name = "play")]
pub struct PlayCommand {
    /// cartridge path
    #[argh(positional)]
    pub file: PathBuf,

    /// trace output file
    #[argh(option, short = 't')]
    pub trace: Option<PathBuf>,
}

/// debug cartridge
#[derive(FromArgs)]
#[argh(subcommand, name = "debug")]
pub struct DebugCommand {
    /// cartridge path
    #[argh(positional)]
    pub file: PathBuf,

    /// add breakpoint at address
    #[argh(option, short = 'b')]
    pub breakpoint: Vec<String>,
}

/// assemble cartridge
#[derive(FromArgs)]
#[argh(subcommand, name = "assemble")]
pub struct AssembleCommand {
    /// source assembly path
    #[argh(positional)]
    pub source: PathBuf,

    /// output file
    #[argh(positional)]
    pub output: PathBuf,
}

/// disassemble cartridge
#[derive(FromArgs)]
#[argh(subcommand, name = "disassemble")]
pub struct DisassembleCommand {
    /// cartridge path
    #[argh(positional)]
    pub file: PathBuf,

    /// output file (omit argument for stdout)
    #[argh(option, short = 'o')]
    pub output: Option<PathBuf>,
}

/// show version
#[derive(FromArgs)]
#[argh(subcommand, name = "version")]
pub struct VersionCommand {}

/// Start shell.
fn main() -> CResult {
    let args: Args = argh::from_env();
    start_shell_using_args(args)
}

/// Start shell using args.
pub fn start_shell_using_args(args: Args) -> CResult {
    parse_args(args)
}

/// Parse arguments.
fn parse_args(args: Args) -> CResult {
    let s = tracing_subscriber::fmt(); //.with_env_filter(EnvFilter::from_default_env());
    s.compact().init();

    match args.nested {
        SubCommands::Version(_) => {
            let cmd_name = std::env::current_exe()
                .unwrap()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();
            println!("{} {}", cmd_name, env!("CARGO_PKG_VERSION"));
        }
        SubCommands::Assemble(cmd) => {
            let assembler =
                Assembler::from_path(&cmd.source).expect("error while reading assembly");
            let cartridge = assembler
                .assemble_cartridge()
                .expect("error while assembling cartridge");
            cartridge
                .save_to_path(&cmd.output)
                .expect("error while saving cartridge");
        }
        SubCommands::Disassemble(cmd) => {
            let cartridge_handle = Cartridge::load_from_path(&cmd.file)?;
            cartridge_handle.write_disassembly_to_file(cmd.output);
        }
        SubCommands::Play(cmd) => {
            // CLI mode.
            let cartridge_handle = Cartridge::load_from_path(&cmd.file);
            if let Err(error) = cartridge_handle {
                eprintln!("{}", error);
                process::exit(1);
            }

            // Extract cartridge.
            let cartridge = cartridge_handle.unwrap();

            let mut emulator = Emulator::new();
            let emulator_context = EmulatorContext::new();
            emulator.load_game(&cartridge);

            if let Some(trace) = cmd.trace {
                emulator.set_tracefile(&trace.to_string_lossy().to_string());
            }

            let mut driver = MQWindowDriver::new();
            if let Err(e) = driver.run_emulator(emulator, emulator_context, cartridge) {
                eprintln!("execution error: {}", e);
                process::exit(1);
            }
        }
        SubCommands::Debug(cmd) => {
            // CLI mode.
            let cartridge_handle = Cartridge::load_from_path(&cmd.file);
            if let Err(error) = cartridge_handle {
                eprintln!("{}", error);
                process::exit(1);
            }

            // Extract cartridge.
            let cartridge = cartridge_handle.unwrap();

            let mut emulator = Emulator::new();
            let emulator_context = EmulatorContext::new();
            emulator.load_game(&cartridge);

            let debugger = Debugger::new();
            let mut debugger_context = DebuggerContext::new();
            debugger_context.set_address(INITIAL_MEMORY_POINTER);

            for v in &cmd.breakpoint {
                debugger_context.register_breakpoint_str(v).unwrap();
            }

            debugger_context.register_breakpoint(INITIAL_MEMORY_POINTER);

            let mut driver = MQWindowDriver::new();
            if let Err(e) = driver.run_debugger(
                debugger,
                debugger_context,
                emulator,
                emulator_context,
                cartridge,
            ) {
                eprintln!("execution error: {}", e);
                process::exit(1);
            }
        }
    }

    Ok(())
}
