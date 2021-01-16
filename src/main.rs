use std::{fs::File, io::prelude::*, path::PathBuf, process::exit};

use anyhow::Result;
use env_logger;
use log;
use structopt::StructOpt;

mod cpu;
mod mem;
mod nes;
mod ppu;
mod rom;

use crate::cpu::Cpu;
use crate::mem::Address;
use crate::nes::Nes;
use crate::rom::Rom;

#[derive(Debug, StructOpt)]
#[structopt(name = "nes", about = "A toy NES emulator")]
enum Command {
    Run(RunArgs),
    RunRaw(RunRawArgs),
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Run a NES ROM file")]
struct RunArgs {
    #[structopt(parse(from_os_str), help = "Path to ROM file")]
    rom: PathBuf,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Run a raw MOS 6502 binary")]
struct RunRawArgs {
    #[structopt(parse(from_os_str), help = "Path to binary file")]
    binary: PathBuf,
    #[structopt(help = "Address at which to start execution")]
    start: Option<Address>,
}

fn main() -> Result<()> {
    env_logger::init();
    match Command::from_args() {
        Command::Run(args) => cmd_run(args),
        Command::RunRaw(args) => cmd_run_raw(args),
    }
}

fn cmd_run(args: RunArgs) -> Result<()> {
    log::info!("Loading ROM: {:?}", &args.rom);
    let rom = Rom::load(&args.rom)?;
    let nes = Nes::new(rom);
    nes.start()
}

fn cmd_run_raw(args: RunRawArgs) -> Result<()> {
    if !args.binary.is_file() {
        eprintln!("{:?} is not a file", &args.binary);
        exit(1);
    }
    log::info!("Loading raw binary: {:?}", &args.binary);

    let mut binary = Vec::new();
    let mut file = File::open(&args.binary)?;
    let _ = file.read_to_end(&mut binary)?;

    let mut cpu = Cpu::new();
    cpu.run(&binary, args.start)
}
