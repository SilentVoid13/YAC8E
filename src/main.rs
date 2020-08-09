mod chip8;
mod ram;
mod cpu;
mod bus;
mod keyboard;
mod display;
mod utils;

use crate::chip8::{Chip8, Chip8Config};

use std::error::Error;

use clap::{App, Arg};
use std::path::Path;
use std::process;

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
                Arg::with_name("width")
                    .short("w")
                    .long("width")
                    .value_name("WIDTH")
                    .help("Sets the window width (default: 640)")
            )
            .arg(
                Arg::with_name("height")
                    .short("h")
                    .long("height")
                    .value_name("HEIGHT")
                    .help("Sets the window height (default: 320)")
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
        None => 640,
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
        None => 320,
    };

    let chip8_config = Chip8Config {
        rom: rom,
        debug: debug,
        window_width: width,
        window_height: height,
    };

    if let Err(e) = Chip8::run_rom(chip8_config) {
        eprintln!("[-] An error occured: {}", e);
        process::exit(1);
    }

    Ok(())
}
