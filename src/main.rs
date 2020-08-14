mod chip8;
mod cpu;
mod ram;
mod handler;
mod screen;
mod keypad;
mod utils;

use crate::chip8::{Chip8, Chip8Config};
use crate::handler::HandlerType;

use std::error::Error;
use std::path::Path;
use std::process;

use clap::{App, Arg};

fn main() -> Result<(), Box<dyn Error>>{
    let version = "0.1";
    let author = "SilentVoid <silentvoid13@protonmail.com>";
    let about = "Just a simple CHIP-8 emulator";

    let matches =
        App::new("Yet Another CHIP-8 Emulator")
            .version(version)
            .author(author)
            .about(about)
            .arg(
                Arg::with_name("ROM_FILE")
                    .index(1)
                    .required(true)
                    .help("The ROM file to run")
            )
            .arg(
                Arg::with_name("debug")
                    .short("d")
                    .long("debug")
                    .help("Sets debugging output")
            )
            .arg(
                Arg::with_name("library")
                    .short("l")
                    .long("library")
                    .possible_value("sdl")
                    .possible_value("minifb")
                    .default_value("sdl")
                    .value_name("LIBRARY")
                    .takes_value(true)
                    .help("Sets the handling library to use (default: sdl) (minifb doesn't support sounds)")
            )
            .arg(
                Arg::with_name("hertz")
                    .short("H")
                    .long("hertz")
                    .default_value("500")
                    .value_name("HERTZ")
                    .help("Sets the Hertz value for the CPU clock cycle per second speed")
            )
            .arg(
                Arg::with_name("width")
                    .long("width")
                    .default_value("640")
                    .value_name("WIDTH")
                    .help("Sets the window width")
            )
            .arg(
                Arg::with_name("height")
                    .long("height")
                    .default_value("320")
                    .value_name("HEIGHT")
                    .help("Sets the window height")
            )
            .get_matches();

    let rom = match matches.value_of("ROM_FILE") {
        Some(f) => {
            if !Path::new(f).exists() {
                eprintln!("[-] File does not exist");
                process::exit(1);
            }
            String::from(f)
        },
        None => {
            eprintln!("[-] Argument parsing error");
            process::exit(1);
        }
    };

    let debug = match matches.occurrences_of("debug") {
        0 => false,
        _ => true,
    };

    let handler_type = match matches.value_of("library") {
        Some(d) => {
            match d {
                "minifb" => HandlerType::MINIFB,
                "sdl" => HandlerType::SDL,
                _ => {
                    eprintln!("\n[-] Invalid Display library value\n");
                    process::exit(1);
                }
            }
        },
        None => {
            eprintln!("[-] Argument parsing error");
            process::exit(1);
        }
    };

    // 500 Hz is considered a good value for CHIP-8 emulators.
    // This mean roughly that 1 clock cycle ~= 2ms
    // (This may vary depending on the instruction, i.e: drawing a sprite costs more than a simple XOR operation)
    let hertz: f64 = match matches.value_of("hertz") {
        Some(t) => {
            match t.parse().unwrap_or_else(|_| {
                eprintln!("\n[-] Invalid Hertz value\n");
                process::exit(1);
            }) {
                x if x > 0.0 && x < 100000.0 => x,
                _ => {
                    eprintln!("\n[-] Invalid Hertz value\n");
                    process::exit(1);
                }
            }
        },
        None => {
            eprintln!("[-] Argument parsing error");
            process::exit(1);
        }
    };

    let width: usize = match matches.value_of("width") {
        Some(t) => {
            match t.parse().unwrap_or_else(|_| {
                eprintln!("\n[-] Invalid width value\n");
                process::exit(1);
            }) {
                x if x > 0 && x < 10000 => x,
                _ => {
                    eprintln!("\n[-] Invalid width value\n");
                    process::exit(1);
                }
            }
        },
        None => {
            eprintln!("[-] Argument parsing error");
            process::exit(1);
        }
    };

    let height: usize = match matches.value_of("height") {
        Some(t) => {
            match t.parse().unwrap_or_else(|_| {
                eprintln!("\n[-] Invalid height value\n");
                process::exit(1);
            }) {
                x if x > 0 && x < 10000 => x,
                _ => {
                    eprintln!("\n[-] Invalid height value\n");
                    process::exit(1);
                }
            }
        },
        None => {
            eprintln!("[-] Argument parsing error");
            process::exit(1);
        }
    };

    let chip8_config = Chip8Config {
        rom: rom,
        debug: debug,
        handler_type: handler_type,
        hertz: hertz,
        window_width: width,
        window_height: height,
    };

    println!("chip8_config: {:#?}", chip8_config);

    if let Err(e) = Chip8::run_rom(chip8_config) {
        eprintln!("[-] An error occured: {}", e);
        process::exit(1);
    }

    Ok(())
}
