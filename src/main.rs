#![allow(dead_code)]

use clap::{App, Arg};
use env_logger;
use log;

mod cpu;
mod mem;
mod nes;
mod rom;

use crate::nes::Nes;
use crate::rom::Rom;

fn main() {
    env_logger::init();

    let matches = App::new("nes")
        .version("0.1")
        .author("Arun Kulshreshtha <kulshrax@gmail.com>")
        .about("A toy NES emulator")
        .arg(
            Arg::with_name("ROM_FILE")
                .help("NES ROM file to load")
                .required(true)
                .index(1),
        )
        .get_matches();

    let rom_file = matches.value_of("ROM_FILE").unwrap();
    log::info!("Loading ROM: {}", &rom_file);
    let rom = Rom::load(&rom_file).expect("Failed to load ROM");

    let mut nes = Nes::new();
    nes.run(&rom);
}
