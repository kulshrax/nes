// #![deny(warnings)]

use std::{fs::File, io::prelude::*, path::PathBuf, process::exit};

use anyhow::Result;
use env_logger;
use log;
use structopt::StructOpt;

mod cpu;
mod mapper;
mod mem;
mod nes;
mod ppu;
mod rom;
mod ui;

use crate::cpu::Cpu;
use crate::mem::Address;
use crate::nes::{DebugPatternUi, Nes};
use crate::rom::Rom;
use crate::ui::Run;

#[derive(Debug, StructOpt)]
#[structopt(name = "nes", about = "A toy NES emulator")]
enum Command {
    Run(RunArgs),
    DebugCpu(DebugCpuArgs),
    DebugPattern(DebugPatternArgs),
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Run a NES ROM file")]
struct RunArgs {
    #[structopt(parse(from_os_str), help = "Path to ROM file")]
    rom: PathBuf,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Run a raw MOS 6502 binary")]
struct DebugCpuArgs {
    #[structopt(parse(from_os_str), help = "Path to binary file")]
    binary: PathBuf,
    #[structopt(help = "Address at which to start execution")]
    start: Option<Address>,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Display the pattern table from a ROM file")]
struct DebugPatternArgs {
    #[structopt(parse(from_os_str), help = "Path to ROM file")]
    rom: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    match Command::from_args() {
        Command::Run(args) => cmd_run(args),
        Command::DebugCpu(args) => cmd_debug_cpu(args),
        Command::DebugPattern(args) => cmd_debug_pattern(args),
    }
}

fn cmd_run(args: RunArgs) -> Result<()> {
    log::info!("Loading ROM: {:?}", &args.rom);
    let rom = Rom::load(&args.rom)?;
    let mut nes = Nes::new(rom);
    nes.run()
}

fn cmd_debug_cpu(args: DebugCpuArgs) -> Result<()> {
    if !args.binary.is_file() {
        eprintln!("{:?} is not a file", &args.binary);
        exit(1);
    }
    log::info!("Executing binary: {:?}", &args.binary);

    let mut binary = Vec::new();
    let mut file = File::open(&args.binary)?;
    let _ = file.read_to_end(&mut binary)?;

    let mut cpu = Cpu::new();
    cpu.run(&binary, args.start)
}

fn cmd_debug_pattern(args: DebugPatternArgs) -> Result<()> {
    log::info!("Displaying pattern table for ROM: {:?}", &args.rom);
    let rom = Rom::load(&args.rom)?;
    let nes = Nes::new(rom);
    let ui = DebugPatternUi::new(nes);
    ui.run()
}
