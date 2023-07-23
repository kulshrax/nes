// #![deny(warnings)]

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::exit;

use anyhow::Result;
use clap::Parser;

mod cpu;
mod io;
mod mapper;
mod mem;
mod nes;
mod ppu;
mod rom;
mod ui;

use crate::cpu::Cpu;
use crate::mem::Address;
use crate::nes::{Nes, ShowPatternUi};
use crate::rom::Rom;
use crate::ui::Ui;

#[derive(Debug, Parser)]
#[clap(name = "nes", about = "A toy NES emulator")]
enum Command {
    Run(RunArgs),
    RunCpu(RunCpuArgs),
    RunHeadless(RunHeadlessArgs),
    ShowPattern(ShowPatternArgs),
    ShowHeader(ShowHeaderArgs),
}

#[derive(Debug, Parser)]
#[clap(about = "Run a NES ROM file")]
struct RunArgs {
    #[clap(help = "Path to ROM file")]
    rom: PathBuf,
}

#[derive(Debug, Parser)]
#[clap(about = "Run a raw MOS 6502 binary")]
struct RunCpuArgs {
    #[clap(help = "Path to binary file")]
    binary: PathBuf,
    #[clap(help = "Address at which to start execution")]
    start: Option<Address>,
    #[clap(help = "Address at which to end execution")]
    end: Option<Address>,
}

#[derive(Debug, Parser)]
#[clap(about = "Run a NES ROM file without video output")]
struct RunHeadlessArgs {
    #[clap(help = "Path to ROM file")]
    rom: PathBuf,
    #[clap(help = "Address at which to start execution")]
    start: Option<Address>,
}

#[derive(Debug, Parser)]
#[clap(about = "Display the pattern table from a ROM file")]
struct ShowPatternArgs {
    #[clap(help = "Path to ROM file")]
    rom: PathBuf,
}

#[derive(Debug, Parser)]
#[clap(about = "Display header information from a ROM file")]
struct ShowHeaderArgs {
    #[clap(help = "Path to ROM file")]
    rom: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    match Command::parse() {
        Command::Run(args) => cmd_run(args),
        Command::RunCpu(args) => cmd_run_cpu(args),
        Command::RunHeadless(args) => cmd_run_headless(args),
        Command::ShowPattern(args) => cmd_show_pattern(args),
        Command::ShowHeader(args) => cmd_show_header(args),
    }
}

fn cmd_run(args: RunArgs) -> Result<()> {
    log::info!("Loading ROM: {:?}", &args.rom);
    let rom = Rom::load(&args.rom)?;
    let nes = Nes::new(rom);
    nes.run()
}

fn cmd_run_cpu(args: RunCpuArgs) -> Result<()> {
    if !args.binary.is_file() {
        log::error!("{:?} is not a file", &args.binary);
        exit(1);
    }
    log::info!("Executing binary: {:?}", &args.binary);

    let mut binary = Vec::new();
    let mut file = File::open(&args.binary)?;
    let _ = file.read_to_end(&mut binary)?;

    let mut cpu = Cpu::new();
    cpu.run(&binary, args.start, args.end);

    Ok(())
}

fn cmd_run_headless(args: RunHeadlessArgs) -> Result<()> {
    log::info!("Loading ROM: {:?}", &args.rom);
    let rom = Rom::load(&args.rom)?;
    let mut nes = Nes::new(rom);
    nes.run_cpu(args.start);
    Ok(())
}

fn cmd_show_pattern(args: ShowPatternArgs) -> Result<()> {
    log::info!("Displaying pattern table for ROM: {:?}", &args.rom);
    let rom = Rom::load(&args.rom)?;
    let nes = Nes::new(rom);
    let ui = ShowPatternUi::new(nes);
    ui.run()
}

fn cmd_show_header(args: ShowHeaderArgs) -> Result<()> {
    if !log::log_enabled!(log::Level::Info) {
        log::error!("This command will print nothing at the current log level.");
        log::error!("Please re-run with: RUST_LOG=info");
    }

    let rom = Rom::load(&args.rom)?;
    log::info!("iNES 1.0 ROM header: {:#?}", &rom.header);
    log::info!("First 8 bytes of PRG data: {:X?}", &rom.prg[..8]);
    log::info!(
        "Last 8 bytes of PRG data: {:X?}",
        &rom.prg[rom.prg.len() - 8..]
    );
    log::info!("First 8 bytes of CHR data: {:X?}", &rom.chr[..8]);
    log::info!(
        "Last 8 bytes of CHR data: {:X?}",
        &rom.chr[rom.chr.len() - 8..]
    );
    Ok(())
}
