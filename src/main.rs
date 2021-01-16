use std::{
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
    process::exit,
};

use anyhow::Result;
use env_logger;
use log;
use structopt::StructOpt;

mod cpu;
mod mem;
// mod nes;
// mod ppu;
// mod rom;

use crate::cpu::Cpu;
use crate::mem::Address;

#[derive(Debug, StructOpt)]
#[structopt(name = "nes", about = "A toy NES emulator")]
struct Args {
    /// NES ROM file to load
    #[structopt(parse(from_os_str))]
    rom: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::from_args();

    if !args.rom.is_file() {
        eprintln!("{:?} is not a file", &args.rom);
        exit(1);
    }

    log::info!("Loading ROM: {:?}", &args.rom);

    run_raw_binary(&args.rom, Address::from(0x400u16))
}

fn run_raw_binary(path: impl AsRef<Path>, start: Address) -> Result<()> {
    let mut binary = Vec::new();
    let mut file = File::open(path)?;
    let _ = file.read_to_end(&mut binary)?;

    let mut cpu = Cpu::new();
    cpu.run(&binary, start)
}
