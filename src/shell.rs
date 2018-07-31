//! CHIP-8 shell

use std::env;
use std::process;

use super::cartridge::Cartridge;
use super::emulator::Emulator;
use super::logger::init_logger;

use clap::{App, Arg};
use log;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Start shell
pub fn start_shell() {
    let mut app = App::new("chip8")
        .version(VERSION)
        .author("Denis B. <bourge.denis@gmail.com>")
        .about("CHIP-8 emulator")
        .arg(
            Arg::with_name("file")
                .value_name("FILENAME")
                .required(true)
                .help("cartridge name (not the path)")
                .takes_value(true),
        ).arg(
            Arg::with_name("disassemble")
                .long("disassemble")
                .short("d")
                .help("disassemble cartridge to file (use '-' to trace in console)")
                .takes_value(true),
        ).arg(
            Arg::with_name("breakpoint")
                .short("b")
                .long("breakpoint")
                .multiple(true)
                .number_of_values(1)
                .help("add breakpoint at address")
                .takes_value(true),
        ).arg(
            Arg::with_name("break-at-start")
                .short("s")
                .long("break-at-start")
                .help("add breakpoint at start"),
        ).arg(
            Arg::with_name("trace")
                .short("t")
                .long("trace")
                .help("trace execution to file")
                .takes_value(true),
        ).arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .help("verbose mode"),
        );

    let matches = app.get_matches_from_safe_borrow(&mut env::args_os());
    match matches {
        Ok(result) => {
            let level = if result.is_present("verbose") {
                debug!("D> Using verbose mode.");
                log::LogLevelFilter::Debug
            } else {
                log::LogLevelFilter::Info
            };

            init_logger(level)
                .unwrap_or_else(|_| panic!("Failed to initialize logger with level: {:?}", level));

            let cartridge_filename = result.value_of("file").unwrap();
            let cartridge_handle = Cartridge::load_from_games_directory(cartridge_filename);

            if let Err(error) = cartridge_handle {
                println!("{}", error);
                process::exit(1);
            }

            // Extract cartridge
            let cartridge = cartridge_handle.unwrap();

            if result.is_present("disassemble") {
                let dis_file = result.value_of("disassemble").unwrap();
                cartridge.print_disassembly(dis_file);
            } else {
                let mut emulator = Emulator::new();

                if result.is_present("trace") {
                    emulator.set_tracefile(result.value_of("trace").unwrap());
                }

                if result.is_present("breakpoint") {
                    let bp_values: Vec<&str> = result.values_of("breakpoint").unwrap().collect();
                    for v in bp_values {
                        emulator.register_breakpoint(v);
                    }
                }

                if result.is_present("break-at-start") {
                    emulator.register_breakpoint("0200");
                }

                emulator.run(&cartridge);
            }
        }

        Err(error) => {
            eprintln!("{}", error.to_string());
        }
    }
}
