//! CHIP-8 shell

use std::env;

use super::logger::init_logger;

use chip8_cpu::CPU;
use chip8_cpu::Cartridge;
use clap::{Arg, App};
use log;

const VERSION: &str = "1.0.0";

/// Start shell
pub fn start_shell() {
    let mut app = App::new("chip8")
        .version(VERSION)
        .author("Denis B. <bourge.denis@gmail.com>")
        .about("CHIP-8 emulator")
        .arg(Arg::with_name("file")
            .value_name("FILENAME")
            .required(true)
            .help("cartridge name (not the path)")
            .takes_value(true))
        .arg(Arg::with_name("disassemble")
            .long("disassemble")
            .short("d")
            .help("disassemble cartridge")
            .takes_value(true))
        .arg(Arg::with_name("breakpoint")
            .short("b")
            .long("breakpoint")
            .multiple(true)
            .number_of_values(1)
            .help("breakpoint at address")
            .takes_value(true))
        .arg(Arg::with_name("trace")
            .short("t")
            .long("trace")
            .help("trace execution")
            .takes_value(true))
        .arg(Arg::with_name("verbose")
            .long("verbose")
            .short("v")
            .help("verbose mode"));

    let matches = app.get_matches_from_safe_borrow(&mut env::args_os());
    match matches {
        Ok(result) => {
            let level = if result.is_present("verbose") {
                debug!("D> Using verbose mode.");
                log::LogLevelFilter::Debug
            } else {
                log::LogLevelFilter::Info
            };

            init_logger(level).expect(&format!("Failed to initialize logger with level: {:?}", level));

            let cartridge_filename = result.value_of("file").unwrap();
            let cartridge = Cartridge::load_from_games_directory(cartridge_filename);

            if result.is_present("disassemble") {
                let dis_file = result.value_of("disassemble").unwrap();
                cartridge.print_disassembly(dis_file);
            } else {
                let mut cpu = CPU::new();

                if result.is_present("trace") {
                    cpu.tracefile(result.value_of("trace").unwrap());
                }

                if result.is_present("breakpoint") {
                    let bp_values: Vec<&str> = result.values_of("breakpoint").unwrap().collect();
                    for v in bp_values {
                        cpu.breakpoints.register(u16::from_str_radix(&v[2..], 16).unwrap());
                    }
                }

                cpu.run(&cartridge);
            }       
        },

        Err(error) => {
            eprintln!("{}", error.to_string());
        }
    }
}
