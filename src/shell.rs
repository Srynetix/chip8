//! CHIP-8 shell.

use std::env;
use std::process;

use super::core::assembler::Assembler;
use super::core::logger::init_logger;
use super::debugger::{Debugger, DebuggerContext};
use super::emulator::{Emulator, EmulatorContext};
use super::peripherals::cartridge::Cartridge;
use super::peripherals::memory::INITIAL_MEMORY_POINTER;
use super::window::{start_window_cli, start_window_gui};

use clap::{App, Arg, ArgMatches};
use log;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Start shell.
pub fn start_shell() {
    let args: Vec<String> = env::args().skip(1).collect();
    start_shell_using_args(&args);
}

/// Start shell using args.
///
/// # Arguments
///
/// * `args` - Arguments.
///
pub fn start_shell_using_args(args: &[String]) {
    let mut app = App::new("chip8")
        .version(VERSION)
        .author("Denis B. <bourge.denis@gmail.com>")
        .about("CHIP-8 emulator")
        .arg(
            Arg::with_name("file")
                .value_name("FILENAME")
                .help("cartridge path")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("disassemble")
                .long("disassemble")
                .short("d")
                .help("disassemble cartridge to file (use '-' to trace in console)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("breakpoint")
                .short("b")
                .long("breakpoint")
                .multiple(true)
                .number_of_values(1)
                .help("add breakpoint at address")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("break-at-start")
                .short("s")
                .long("break-at-start")
                .help("add breakpoint at start"),
        )
        .arg(
            Arg::with_name("trace")
                .short("t")
                .long("trace")
                .help("trace execution to file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .help("verbose mode"),
        )
        .arg(
            Arg::with_name("assemble")
                .long("assemble")
                .short("a")
                .help("assemble code")
                .value_name("OUTPUT")
                .takes_value(true),
        )
        .arg(Arg::with_name("gui").long("gui").help("GUI mode"));

    let cmd_name = std::env::current_exe().expect("current executable name missing");
    let mut full_args: Vec<String> = vec![];
    full_args.push(String::from(cmd_name.to_str().unwrap()));
    for arg in args.iter() {
        full_args.push(arg.clone());
    }

    match app.get_matches_from_safe_borrow(&full_args) {
        Ok(result) => parse_args(&result),
        Err(error) => {
            eprintln!("{}", error.to_string());
        }
    }
}

/// Parse arguments.
///
/// # Arguments
///
/// * `matches` - Matches.
///
pub fn parse_args(matches: &ArgMatches<'_>) {
    let level = if matches.is_present("verbose") {
        debug!("using verbose mode");
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    init_logger(level)
        .unwrap_or_else(|_| panic!("failed to initialize logger with level: {:?}", level));

    if matches.is_present("gui") {
        // GUI mode.
        if let Err(e) = start_window_gui() {
            eprintln!("execution error: {}", e);
            process::exit(1);
        }
    } else if matches.is_present("assemble") {
        // Assembly mode.
        let cartridge_path = match matches.value_of("file") {
            Some(f) => f,
            None => {
                eprintln!("error: missing file argument. show help with --help");
                process::exit(1);
            }
        };

        let output_path = match matches.value_of("assemble") {
            Some(f) => f,
            None => {
                eprintln!("error: missing output argument. show help with --help");
                process::exit(1);
            }
        };

        let assembler = Assembler::from_path(cartridge_path).expect("error while reading assembly");
        let cartridge = assembler
            .assemble_cartridge()
            .expect("error while assembling cartridge");
        cartridge
            .save_to_path(output_path)
            .expect("error while saving cartridge");
    } else {
        // CLI mode.
        let cartridge_path = match matches.value_of("file") {
            Some(f) => f,
            None => {
                eprintln!("error: missing file argument. show help with --help");
                process::exit(1);
            }
        };

        let cartridge_handle = Cartridge::load_from_path(cartridge_path);
        if let Err(error) = cartridge_handle {
            eprintln!("{}", error);
            process::exit(1);
        }

        // Extract cartridge.
        let cartridge = cartridge_handle.unwrap();

        if matches.is_present("disassemble") {
            let dis_file = matches.value_of("disassemble").unwrap();
            cartridge.write_disassembly_to_file(dis_file);
        } else {
            let mut emulator = Emulator::new();
            let mut emulator_context = EmulatorContext::new();
            emulator.load_game(&cartridge);

            let mut debugger = Debugger::new();
            let mut debugger_context = DebuggerContext::new();
            debugger_context.set_address(INITIAL_MEMORY_POINTER);

            if matches.is_present("trace") {
                emulator.set_tracefile(matches.value_of("trace").unwrap());
            }

            if matches.is_present("breakpoint") {
                let bp_values: Vec<&str> = matches.values_of("breakpoint").unwrap().collect();
                for v in bp_values {
                    debugger_context.register_breakpoint_str(v).unwrap();
                }
            }

            if matches.is_present("break-at-start") {
                debugger_context.register_breakpoint(INITIAL_MEMORY_POINTER);
            }

            if let Err(e) = start_window_cli(
                &mut debugger,
                &mut debugger_context,
                &mut emulator,
                &mut emulator_context,
                &cartridge,
            ) {
                eprintln!("execution error: {}", e);
                process::exit(1);
            }
        }
    }
}
