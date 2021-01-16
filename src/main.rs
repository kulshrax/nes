use std::{
    fs::File,
    io::prelude::*,
    path::PathBuf,
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
    /// Raw 6502 binary to load.
    #[structopt(parse(from_os_str))]
    binary: PathBuf,
    /// Address from which to start execution.
    start: Option<Address>,

}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::from_args();

    if !args.binary.is_file() {
        eprintln!("{:?} is not a file", &args.binary);
        exit(1);
    }

    log::info!("Loading file: {:?}", &args.binary);
    run_raw_binary(&args)
}

fn run_raw_binary(args: &Args) -> Result<()> {
    let mut binary = Vec::new();
    let mut file = File::open(&args.binary)?;
    let _ = file.read_to_end(&mut binary)?;

    let mut cpu = Cpu::new();
    cpu.run(&binary, args.start)
}
