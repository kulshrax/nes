#![allow(dead_code)]

use std::{path::PathBuf, process::exit};

use env_logger;
use log;
use structopt::StructOpt;

mod cpu;
mod mem;
mod nes;
mod rom;

use crate::mem::Address;
use crate::nes::Nes;
use crate::rom::Rom;

#[derive(Debug, StructOpt)]
#[structopt(name = "nes", about = "A toy NES emulator")]
struct Args {
    /// NES ROM file to load
    #[structopt(parse(from_os_str))]
    rom: PathBuf,
}

fn main() {
    env_logger::init();
    let args = Args::from_args();

    if !args.rom.is_file() {
        eprintln!("{:?} is not a file", &args.rom);
        exit(1);
    }

    log::info!("Loading ROM: {:?}", &args.rom);
    let rom = Rom::load(&args.rom).expect("Failed to load ROM");

    let mut nes = Nes::new();
    nes.run(&rom, Address::from(0x400u16));
}
