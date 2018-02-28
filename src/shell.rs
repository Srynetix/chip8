//! CHIP-8 shell

use std::env;

use super::device::Device;
use super::logger::init_logger;

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
            .help("disassemble cartridge"))
        .arg(Arg::with_name("show-cpu")
            .long("show-cpu")
            .help("show CPU dump"))
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
                cartridge.print_disassembly();
            } else {
                let mut device = Device::new();
                device.read_cartridge(&cartridge);

                if result.is_present("show-cpu") {
                    device.debug_cpu()
                } else {
                    device.run();
                }
            }       
        },

        Err(error) => {
            eprintln!("{}", error.to_string());
        }
    }
}
