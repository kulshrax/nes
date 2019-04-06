#![allow(dead_code)]

use std::path::PathBuf;

use env_logger;
use log;
use structopt::StructOpt;

mod cpu;
mod mem;
mod nes;
mod rom;

use crate::nes::Nes;
use crate::rom::Rom;

#[derive(Debug, StructOpt)]
#[structopt(name = "nes", about = "A toy NES emulator")]
struct Args {
    /// NES ROM file to load
    #[structopt(parse(from_os_str))]
    rom_file: PathBuf,
}

fn main() {
    env_logger::init();
    let args = Args::from_args();

    log::info!("Loading ROM: {:?}", &args.rom_file);
    let rom = Rom::load(&args.rom_file).expect("Failed to load ROM");

    let mut nes = Nes::new();
    nes.run(&rom);
}
