//! An emulated MOS 6502 CPU.
//!
//! The NES uses an 8-bit Ricoh 2A03 CPU running at 1.79 MHz (for the NTSC
//! version of the console). The chip includes a CPU core based on the MOS 6502
//! CPU (modified to disable decimal mode) along with an audio processing unit
//! (APU) for audio generation.
//!
//! This module implements an emulator for the MOS 6502, supporting all of the
//! official opcodes in the CPU's instruction set, plus some (though not all)
//! undocumented instructions as well.
//!
//! Many thanks to Andrew Jacobs, whose introductory guide to the MOS 6502
//! (http://www.obelisk.me.uk/6502/) was an invaluable resource for this
//! implementation.

use std::cmp;

use crate::mem::{Address, Bus};

use addressing::{Absolute, AddressingMode, Relative};
use instruction::Instruction;
use registers::{Flags, Registers};

mod addressing;
mod instruction;
mod registers;

/// The 6502 has a 256-byte stack address space that is fixed at memory page 1
/// (addresses 0x100 to 0x1FF). The stack starts at 0x1FF and grows downward as
/// values are pushed. The next free location on the stack is pointed at by the
/// S register, which contains the low byte of the next available stack address.
/// There is no overflow checking for the call stack.
const STACK_START: u16 = 0x0100;

/// When the CPU receives an interrupt, it loads an address from a fixed memory
/// location (known as an interrupt vector) and sets the program counter to that
/// address. This allows the program to specify interrupt handlers by writing
/// the address of the start of the handler code to the appropriate interrupt
/// vector location. There are several interrupt vectors, each corresponding to
/// a different kind of interrupt. All of them stored in the highest bytes of
/// the 16-bit address space.
const NMI_VECTOR: [u16; 2] = [0xFFFA, 0xFFFB];
const RESET_VECTOR: [u16; 2] = [0xFFFC, 0xFFFD];
const IRQ_VECTOR: [u16; 2] = [0xFFFE, 0xFFFF];

/// The number of cycles that each machine operation takes, indexed by opcode.
///
/// This was copied from the source code of [sprocketnes], which in turn
/// copied it from FCEU.
///
/// [sprocketnes]: https://github.com/pcwalton/sprocketnes
#[rustfmt::skip]
static CYCLE_TABLE: [u8; 256] = [
    /*0x00*/ 7, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 4, 4, 6, 6,
    /*0x10*/ 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
    /*0x20*/ 6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 4, 4, 6, 6,
    /*0x30*/ 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
    /*0x40*/ 6, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 3, 4, 6, 6,
    /*0x50*/ 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
    /*0x60*/ 6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 5, 4, 6, 6,
    /*0x70*/ 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
    /*0x80*/ 2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4,
    /*0x90*/ 2, 6, 2, 6, 4, 4, 4, 4, 2, 5, 2, 5, 5, 5, 5, 5,
    /*0xA0*/ 2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4,
    /*0xB0*/ 2, 5, 2, 5, 4, 4, 4, 4, 2, 4, 2, 4, 4, 4, 4, 4,
    /*0xC0*/ 2, 6, 2, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6,
    /*0xD0*/ 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
    /*0xE0*/ 2, 6, 3, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6,
    /*0xF0*/ 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
];

/// Emulated MOS 6502 CPU.
pub struct Cpu {
    registers: Registers,
    irq_pending: bool,
    cycles_remaining: u8,
    cycle: u64,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            irq_pending: false,
            cycles_remaining: 0,
            cycle: 0,
        }
    }

    /// Execute a raw 6502 binary, starting from the specified address.
    ///
    /// This is primarily here to allow testing the  CPU independently from the
    /// rest of the emulator, and is not used for running actual NES ROMs.
    ///
    /// If no start address is specified, execution will begin from the address
    /// specified in the reset vector contained in the binary itself.
    ///
    /// If an end address is specified, this function will return if and when
    /// the program counter reaches that address. If not specified, the function
    /// will not return.
    pub fn run(&mut self, binary: &[u8], start: Option<Address>, end: Option<Address>) {
        // Copy the binary into a 16-bit address space.
        let mut memory = [0u8; 0x10000];
        let n = cmp::min(binary.len(), 0x10000); // Truncate binary if too big.
        memory[..n].copy_from_slice(&binary[..n]);

        // Overwrite reset vector with desired start address if specified.
        if let Some(start) = start {
            self.set_reset_vector(&mut memory, start);
        }

        // Loop until we hit the end address (or forever if not specified).
        self.reset(&mut memory);
        while end.map_or(true, |end| self.registers.pc != end) {
            // Note that we don't keep track of cycle timing here since the
            // CPU is running in isolation.
            let _ = self.step(&mut memory);
        }
    }

    /// Manually set the address stored in the CPU's reset vector. Program
    /// execution will begin from this address on CPU startup or reset.
    pub fn set_reset_vector(&mut self, memory: &mut dyn Bus, addr: Address) {
        let [low, high] = <[u8; 2]>::from(addr);
        memory.store(Address::from(RESET_VECTOR[0]), low);
        memory.store(Address::from(RESET_VECTOR[1]), high);
    }

    /// Examine the current state of the CPU's registers.
    #[cfg(test)]
    pub fn registers(&self) -> &Registers {
        &self.registers
    }

    /// Manually set the CPU's program counter. Useful for testing.
    pub fn set_pc(&mut self, addr: Address) {
        log::trace!("Manually setting program counter: {}", addr);
        self.registers.pc = addr;
    }

    /// Fetch and execute a single instruction. Returns the the number of clock
    /// cycles taken to execute the instruction. Does not update the CPU's
    /// cycle counter; cycle tracking is handled by `Cpu::tick`.
    pub fn step(&mut self, memory: &mut dyn Bus) -> u8 {
        // Save starting program counter.
        let pc = self.registers.pc;

        // If there is a pending interrupt and interrupts are not disabled,
        // service it immediately.
        if self.irq_pending && !self.registers.p.contains(Flags::INTERRUPT_DISABLE) {
            log::trace!("Handling pending IRQ");
            self.irq_pending = false;
            self.irq(memory);
        }

        let (instruction, opcode) = Instruction::fetch(memory, &mut self.registers.pc);
        self.exec(memory, instruction);

        log::trace!(
            "PC: {}; OP: {:#X}; Instruction: {:X?}; Cycle: {}",
            pc,
            opcode,
            instruction,
            self.cycle
        );
        log::trace!("Registers: {}", &self.registers);

        // Crash if we detect an infinite loop. This is useful for test ROMs
        // that intentionally enter an infinite loop to signal a test failure.
        if pc == self.registers.pc {
            panic!(
                "Detected infinite loop at {}; Registers: {}",
                pc, self.registers
            );
        }

        CYCLE_TABLE[opcode as usize]
    }

    /// Drive the CPU with an external clock signal.
    ///
    /// CPU instructions generally take several clock cycles to execute. Calling
    /// this method is equivalent to a single cycle of an external clock. When
    /// called, the CPU will either execute the next instruction, or wait until
    /// the "currently executing" instruction has finished. Note that although
    /// the CPU will "block" for the correct number of clock cycles, the actual
    /// effect of the instruction happens entirely on the first clock cycle.
    pub fn tick(&mut self, memory: &mut dyn Bus) {
        if self.cycles_remaining == 0 {
            self.cycles_remaining = self.step(memory) - 1;
        } else {
            self.cycles_remaining -= 1;
        }
        self.cycle += 1;
    }

    /// Reset the CPU by disabling interrupts and jumping to the location
    /// specified by the initialization vector.
    pub fn reset(&mut self, memory: &mut dyn Bus) {
        self.registers.p.insert(Flags::INTERRUPT_DISABLE);
        let low = memory.load(Address::from(RESET_VECTOR[0]));
        let high = memory.load(Address::from(RESET_VECTOR[1]));
        self.registers.pc = Address::from([low, high]);

        // The reset sequence takes 7 cycles before fetching the instruction
        // at the location specified by the reset vector.
        self.cycle = 7;
        self.cycles_remaining = 0;
    }

    /// Interrupt request.
    pub fn irq(&mut self, memory: &mut dyn Bus) {
        log::trace!("Received IRQ");
        if self.registers.p.contains(Flags::INTERRUPT_DISABLE) {
            log::trace!("Interrupts are disabled; IRQ will be handled when they are enabled");
            self.irq_pending = true;
        } else {
            log::trace!("Handling IRQ");
            self.interrupt(memory, &IRQ_VECTOR, false);
        }
    }

    /// Non-maskable interrupt.
    #[allow(dead_code)]
    pub fn nmi(&mut self, memory: &mut dyn Bus) {
        self.interrupt(memory, &NMI_VECTOR, false);
    }

    /// Execute the given instruction.
    fn exec(&mut self, memory: &mut dyn Bus, op: Instruction) {
        use Instruction::*;
        match op {
            AdcI(am) => self.adc(am, memory),
            AdcZ(am) => self.adc(am, memory),
            AdcZX(am) => self.adc(am, memory),
            AdcA(am) => self.adc(am, memory),
            AdcAX(am) => self.adc(am, memory),
            AdcAY(am) => self.adc(am, memory),
            AdcIX(am) => self.adc(am, memory),
            AdcIY(am) => self.adc(am, memory),
            AndI(am) => self.and(am, memory),
            AndZ(am) => self.and(am, memory),
            AndZX(am) => self.and(am, memory),
            AndA(am) => self.and(am, memory),
            AndAX(am) => self.and(am, memory),
            AndAY(am) => self.and(am, memory),
            AndIX(am) => self.and(am, memory),
            AndIY(am) => self.and(am, memory),
            AslAcc(am) => self.asl(am, memory),
            AslZ(am) => self.asl(am, memory),
            AslZX(am) => self.asl(am, memory),
            AslA(am) => self.asl(am, memory),
            AslAX(am) => self.asl(am, memory),
            Bcc(am) => self.bcc(am, memory),
            Bcs(am) => self.bcs(am, memory),
            Beq(am) => self.beq(am, memory),
            BitZ(am) => self.bit(am, memory),
            BitA(am) => self.bit(am, memory),
            Bmi(am) => self.bmi(am, memory),
            Bne(am) => self.bne(am, memory),
            Bpl(am) => self.bpl(am, memory),
            Brk => self.brk(memory),
            Bvc(am) => self.bvc(am, memory),
            Bvs(am) => self.bvs(am, memory),
            Clc => self.clc(),
            Cld => self.cld(),
            Cli => self.cli(),
            Clv => self.clv(),
            CmpI(am) => self.cmp(am, memory),
            CmpZ(am) => self.cmp(am, memory),
            CmpZX(am) => self.cmp(am, memory),
            CmpA(am) => self.cmp(am, memory),
            CmpAX(am) => self.cmp(am, memory),
            CmpAY(am) => self.cmp(am, memory),
            CmpIX(am) => self.cmp(am, memory),
            CmpIY(am) => self.cmp(am, memory),
            CpxI(am) => self.cpx(am, memory),
            CpxZ(am) => self.cpx(am, memory),
            CpxA(am) => self.cpx(am, memory),
            CpyI(am) => self.cpy(am, memory),
            CpyZ(am) => self.cpy(am, memory),
            CpyA(am) => self.cpy(am, memory),
            DecZ(am) => self.dec(am, memory),
            DecZX(am) => self.dec(am, memory),
            DecA(am) => self.dec(am, memory),
            DecAX(am) => self.dec(am, memory),
            Dex => self.dex(),
            Dey => self.dey(),
            EorI(am) => self.eor(am, memory),
            EorZ(am) => self.eor(am, memory),
            EorZX(am) => self.eor(am, memory),
            EorA(am) => self.eor(am, memory),
            EorAX(am) => self.eor(am, memory),
            EorAY(am) => self.eor(am, memory),
            EorIX(am) => self.eor(am, memory),
            EorIY(am) => self.eor(am, memory),
            IncZ(am) => self.inc(am, memory),
            IncZX(am) => self.inc(am, memory),
            IncA(am) => self.inc(am, memory),
            IncAX(am) => self.inc(am, memory),
            Inx => self.inx(),
            Iny => self.iny(),
            JmpA(am) => self.jmp(am, memory),
            JmpI(am) => self.jmp(am, memory),
            Jsr(am) => self.jsr(am, memory),
            LdaI(am) => self.lda(am, memory),
            LdaZ(am) => self.lda(am, memory),
            LdaZX(am) => self.lda(am, memory),
            LdaA(am) => self.lda(am, memory),
            LdaAX(am) => self.lda(am, memory),
            LdaAY(am) => self.lda(am, memory),
            LdaIX(am) => self.lda(am, memory),
            LdaIY(am) => self.lda(am, memory),
            LdxI(am) => self.ldx(am, memory),
            LdxZ(am) => self.ldx(am, memory),
            LdxZY(am) => self.ldx(am, memory),
            LdxA(am) => self.ldx(am, memory),
            LdxAY(am) => self.ldx(am, memory),
            LdyI(am) => self.ldy(am, memory),
            LdyZ(am) => self.ldy(am, memory),
            LdyZX(am) => self.ldy(am, memory),
            LdyA(am) => self.ldy(am, memory),
            LdyAX(am) => self.ldy(am, memory),
            LsrAcc(am) => self.lsr(am, memory),
            LsrZ(am) => self.lsr(am, memory),
            LsrZX(am) => self.lsr(am, memory),
            LsrA(am) => self.lsr(am, memory),
            LsrAX(am) => self.lsr(am, memory),
            Nop => {}
            OraI(am) => self.ora(am, memory),
            OraZ(am) => self.ora(am, memory),
            OraZX(am) => self.ora(am, memory),
            OraA(am) => self.ora(am, memory),
            OraAX(am) => self.ora(am, memory),
            OraAY(am) => self.ora(am, memory),
            OraIX(am) => self.ora(am, memory),
            OraIY(am) => self.ora(am, memory),
            Pha => self.pha(memory),
            Php => self.php(memory),
            Pla => self.pla(memory),
            Plp => self.plp(memory),
            RolAcc(am) => self.rol(am, memory),
            RolZ(am) => self.rol(am, memory),
            RolZX(am) => self.rol(am, memory),
            RolA(am) => self.rol(am, memory),
            RolAX(am) => self.rol(am, memory),
            RorAcc(am) => self.ror(am, memory),
            RorZ(am) => self.ror(am, memory),
            RorZX(am) => self.ror(am, memory),
            RorA(am) => self.ror(am, memory),
            RorAX(am) => self.ror(am, memory),
            Rti => self.rti(memory),
            Rts => self.rts(memory),
            SbcI(am) => self.sbc(am, memory),
            SbcZ(am) => self.sbc(am, memory),
            SbcZX(am) => self.sbc(am, memory),
            SbcA(am) => self.sbc(am, memory),
            SbcAX(am) => self.sbc(am, memory),
            SbcAY(am) => self.sbc(am, memory),
            SbcIX(am) => self.sbc(am, memory),
            SbcIY(am) => self.sbc(am, memory),
            Sec => self.sec(),
            Sed => self.sed(),
            Sei => self.sei(),
            StaZ(am) => self.sta(am, memory),
            StaZX(am) => self.sta(am, memory),
            StaA(am) => self.sta(am, memory),
            StaAX(am) => self.sta(am, memory),
            StaAY(am) => self.sta(am, memory),
            StaIX(am) => self.sta(am, memory),
            StaIY(am) => self.sta(am, memory),
            StxZ(am) => self.stx(am, memory),
            StxZY(am) => self.stx(am, memory),
            StxA(am) => self.stx(am, memory),
            StyZ(am) => self.sty(am, memory),
            StyZX(am) => self.sty(am, memory),
            StyA(am) => self.sty(am, memory),
            Tax => self.tax(),
            Tay => self.tay(),
            Tsx => self.tsx(),
            Txa => self.txa(),
            Txs => self.txs(),
            Tya => self.tya(),
            UDcpZ(am) => self.undoc_dcp(am, memory),
            UDcpZX(am) => self.undoc_dcp(am, memory),
            UDcpA(am) => self.undoc_dcp(am, memory),
            UDcpAX(am) => self.undoc_dcp(am, memory),
            UDcpAY(am) => self.undoc_dcp(am, memory),
            UDcpIX(am) => self.undoc_dcp(am, memory),
            UDcpIY(am) => self.undoc_dcp(am, memory),
            UIsbZ(am) => self.undoc_isb(am, memory),
            UIsbZX(am) => self.undoc_isb(am, memory),
            UIsbA(am) => self.undoc_isb(am, memory),
            UIsbAX(am) => self.undoc_isb(am, memory),
            UIsbAY(am) => self.undoc_isb(am, memory),
            UIsbIX(am) => self.undoc_isb(am, memory),
            UIsbIY(am) => self.undoc_isb(am, memory),
            ULaxZ(am) => self.undoc_lax(am, memory),
            ULaxZY(am) => self.undoc_lax(am, memory),
            ULaxA(am) => self.undoc_lax(am, memory),
            ULaxAY(am) => self.undoc_lax(am, memory),
            ULaxIX(am) => self.undoc_lax(am, memory),
            ULaxIY(am) => self.undoc_lax(am, memory),
            UNop | UNopI(_) | UNopZ(_) | UNopZX(_) | UNopA(_) | UNopAX(_) => {}
            URlaZ(am) => self.undoc_rla(am, memory),
            URlaZX(am) => self.undoc_rla(am, memory),
            URlaA(am) => self.undoc_rla(am, memory),
            URlaAX(am) => self.undoc_rla(am, memory),
            URlaAY(am) => self.undoc_rla(am, memory),
            URlaIX(am) => self.undoc_rla(am, memory),
            URlaIY(am) => self.undoc_rla(am, memory),
            URraZ(am) => self.undoc_rra(am, memory),
            URraZX(am) => self.undoc_rra(am, memory),
            URraA(am) => self.undoc_rra(am, memory),
            URraAX(am) => self.undoc_rra(am, memory),
            URraAY(am) => self.undoc_rra(am, memory),
            URraIX(am) => self.undoc_rra(am, memory),
            URraIY(am) => self.undoc_rra(am, memory),
            USaxZ(am) => self.undoc_sax(am, memory),
            USaxZY(am) => self.undoc_sax(am, memory),
            USaxA(am) => self.undoc_sax(am, memory),
            USaxIX(am) => self.undoc_sax(am, memory),
            USbcI(am) => self.sbc(am, memory), // Identical to legal SBC.
            USloZ(am) => self.undoc_slo(am, memory),
            USloZX(am) => self.undoc_slo(am, memory),
            USloA(am) => self.undoc_slo(am, memory),
            USloAX(am) => self.undoc_slo(am, memory),
            USloAY(am) => self.undoc_slo(am, memory),
            USloIX(am) => self.undoc_slo(am, memory),
            USloIY(am) => self.undoc_slo(am, memory),
            USreZ(am) => self.undoc_sre(am, memory),
            USreZX(am) => self.undoc_sre(am, memory),
            USreA(am) => self.undoc_sre(am, memory),
            USreAX(am) => self.undoc_sre(am, memory),
            USreAY(am) => self.undoc_sre(am, memory),
            USreIX(am) => self.undoc_sre(am, memory),
            USreIY(am) => self.undoc_sre(am, memory),
            UStp => panic!("CPU halted due to (illegal) STP instruction"),
        }
    }

    /// Cause the CPU to stop the normal execution flow and begin executing an
    /// interrupt handler. The interrupt handler that is executed is determined
    /// from by the address stored at the location specified by the given
    /// interrupt vector. The brk parameter allows specifying whether this was a
    /// software or hardware interrupt.
    fn interrupt(&mut self, memory: &mut dyn Bus, vector: &[u16; 2], brk: bool) {
        // Push program counter to stack.
        let [low, high] = <[u8; 2]>::from(self.registers.pc + 1u8);
        self.push_stack(memory, high);
        self.push_stack(memory, low);

        // Push flags to stack and set the BRK flag in the pushed byte. In
        // particular, if this interrupt was triggered by a BRK instruction,
        // then the BRK bit should be 1 in the pushed value; if the interrupt
        // was triggered by a hardware interrupt, it should be set to 0.
        let mut flags = self.registers.p;
        flags.set(Flags::BREAK, brk);
        self.push_stack(memory, flags.bits());

        // Disable interrupts so that the interrupt handler is not itself
        // interrupted.
        self.registers.p.insert(Flags::INTERRUPT_DISABLE);

        // Load the interrupt handler address from a fixed location in memory,
        // then jump to that address.
        let low = memory.load(Address::from(vector[0]));
        let high = memory.load(Address::from(vector[1]));
        self.registers.pc = Address::from([low, high]);

        // Interrupts take 7 cycles before beginning execution of the interrupt
        // handler code.
        self.cycle += 7;
    }

    /// Get the current address of the next available memory location on the
    /// call stack.
    fn stack(&self) -> Address {
        Address::from(STACK_START) + self.registers.s
    }

    /// Push a value onto the call stack. Note that if the stack pointer
    /// overflows, this will wrap around and overwrite data at the start of the
    /// stack.
    fn push_stack(&mut self, memory: &mut dyn Bus, value: u8) {
        memory.store(self.stack(), value);
        self.registers.s = self.registers.s.wrapping_sub(1);
    }

    /// Pull ("pop" in more modern terms) a value from the call stack. If the
    /// stack pointer underflows, it will wrap around to the top of memory page
    /// 1, potentially reading garbage.
    fn pull_stack(&mut self, memory: &mut dyn Bus) -> u8 {
        self.registers.s = self.registers.s.wrapping_add(1);
        memory.load(self.stack())
    }

    /// Pull a value from the stack and use it to set the status register.
    /// Notably, bits 4 and 5 are ignored in the pulled value; bit 4 is always
    /// set to 0, and bit 5 is always set to 1 in the status register.
    fn pull_flags(&mut self, memory: &mut dyn Bus) {
        let bits = self.pull_stack(memory);
        self.registers.p = (Flags::from_bits_truncate(bits) | Flags::UNUSED) & !Flags::BREAK;
    }

    /// Check if the given value is zero or negative and set the appropriate
    /// flags in the status register. Note that since the value is unsigned, the
    /// negative check is just checking if the sign bit is set if the value were
    /// interpreted as a two's complement signed integer.
    fn check_zero_or_negative(&mut self, value: u8) {
        self.registers.p.set(Flags::ZERO, value == 0);
        self.registers.p.set(Flags::NEGATIVE, value > 127);
    }
}

/// Methods corresponding to operations in the MOS 6502 instruction set.
///
/// See http://obelisk.me.uk/6502/reference.html for details about each
/// instruction.
impl Cpu {
    /// Add with carry.
    fn adc(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let value = am.load(memory, &self.registers);
        let carry_in = self.registers.p.contains(Flags::CARRY);

        // Cast the values to u16's before adding so we can
        // more easily check for overflow.
        let res = self.registers.a as u16 + value as u16 + carry_in as u16;
        let carry_out = res > 255;
        let res = res as u8;

        let overflow = twos_complement_overflow(self.registers.a, value, res);
        self.registers.p.set(Flags::OVERFLOW, overflow);

        self.registers.a = res;
        self.registers.p.set(Flags::CARRY, carry_out);
        self.check_zero_or_negative(res);
    }

    /// Logical AND.
    fn and(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let value = am.load(memory, &self.registers);
        self.registers.a &= value;
        self.check_zero_or_negative(self.registers.a);
    }

    /// Arithmetic left shift.
    fn asl(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let value = am.load(memory, &self.registers);
        let res = value << 1;
        am.store(memory, &mut self.registers, res);

        self.registers.p.set(Flags::CARRY, value & (1 << 7) > 0);
        self.check_zero_or_negative(res);
    }

    /// Branch if carry clear.
    fn bcc(&mut self, am: Relative, memory: &mut dyn Bus) {
        if !self.registers.p.contains(Flags::CARRY) {
            let addr = am.address(memory, &mut self.registers);
            self.registers.pc = addr;
        }
    }

    /// Branch if carry set.
    fn bcs(&mut self, am: Relative, memory: &mut dyn Bus) {
        if self.registers.p.contains(Flags::CARRY) {
            let addr = am.address(memory, &mut self.registers);
            self.registers.pc = addr;
        }
    }

    /// Branch if equal.
    fn beq(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        if self.registers.p.contains(Flags::ZERO) {
            let addr = am.address(memory, &mut self.registers);
            self.registers.pc = addr;
        }
    }

    /// Bit test.
    fn bit(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let value = am.load(memory, &self.registers);
        let res = self.registers.a & value;
        self.registers.p.set(Flags::ZERO, res == 0);
        self.registers.p.set(Flags::OVERFLOW, value & (1 << 6) > 0);
        self.registers.p.set(Flags::NEGATIVE, value & (1 << 7) > 0);
    }

    /// Branch if minus.
    fn bmi(&mut self, am: Relative, memory: &mut dyn Bus) {
        if self.registers.p.contains(Flags::NEGATIVE) {
            let addr = am.address(memory, &mut self.registers);
            self.registers.pc = addr;
        }
    }

    /// Branch if not equal.
    fn bne(&mut self, am: Relative, memory: &mut dyn Bus) {
        if !self.registers.p.contains(Flags::ZERO) {
            let addr = am.address(memory, &mut self.registers);
            self.registers.pc = addr;
        }
    }

    /// Branch if positive.
    fn bpl(&mut self, am: Relative, memory: &mut dyn Bus) {
        if !self.registers.p.contains(Flags::NEGATIVE) {
            let addr = am.address(memory, &mut self.registers);
            self.registers.pc = addr;
        }
    }

    /// Force interrupt.
    fn brk(&mut self, memory: &mut dyn Bus) {
        self.interrupt(memory, &IRQ_VECTOR, true);
    }

    /// Branch if overflow clear.
    fn bvc(&mut self, am: Relative, memory: &mut dyn Bus) {
        if !self.registers.p.contains(Flags::OVERFLOW) {
            let addr = am.address(memory, &mut self.registers);
            self.registers.pc = addr;
        }
    }

    /// Branch if overflow set.
    fn bvs(&mut self, am: Relative, memory: &mut dyn Bus) {
        if self.registers.p.contains(Flags::OVERFLOW) {
            let addr = am.address(memory, &mut self.registers);
            self.registers.pc = addr;
        }
    }

    /// Clear carry flag.
    fn clc(&mut self) {
        self.registers.p.remove(Flags::CARRY);
    }

    /// Clear decimal mode.
    fn cld(&mut self) {
        self.registers.p.remove(Flags::DECIMAL);
    }

    /// Clear interrupt disable.
    fn cli(&mut self) {
        self.registers.p.remove(Flags::INTERRUPT_DISABLE);
    }

    /// Clear overflow flag.
    fn clv(&mut self) {
        self.registers.p.remove(Flags::OVERFLOW);
    }

    /// Compare.
    fn cmp(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let value = am.load(memory, &self.registers);
        let (res, overflowed) = self.registers.a.overflowing_sub(value);
        self.registers.p.set(Flags::CARRY, !overflowed);
        self.check_zero_or_negative(res);
    }

    /// Compare X register.
    fn cpx(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let value = am.load(memory, &self.registers);
        let (res, overflowed) = self.registers.x.overflowing_sub(value);
        self.registers.p.set(Flags::CARRY, !overflowed);
        self.check_zero_or_negative(res);
    }

    /// Compare Y register.
    fn cpy(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let value = am.load(memory, &self.registers);
        let (res, overflowed) = self.registers.y.overflowing_sub(value);
        self.registers.p.set(Flags::CARRY, !overflowed);
        self.check_zero_or_negative(res);
    }

    /// Decrement memory.
    fn dec(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let mut value = am.load(memory, &self.registers);
        value = value.wrapping_sub(1);
        am.store(memory, &mut self.registers, value);
        self.check_zero_or_negative(value);
    }

    /// Decrement X register.
    fn dex(&mut self) {
        self.registers.x = self.registers.x.wrapping_sub(1);
        self.check_zero_or_negative(self.registers.x);
    }

    /// Decrement Y register.
    fn dey(&mut self) {
        self.registers.y = self.registers.y.wrapping_sub(1);
        self.check_zero_or_negative(self.registers.y);
    }

    /// Exclusive OR.
    fn eor(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let value = am.load(memory, &self.registers);
        self.registers.a ^= value;
        self.check_zero_or_negative(self.registers.a);
    }

    /// Incrememnt memory.
    fn inc(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let mut value = am.load(memory, &self.registers);
        value = value.wrapping_add(1);
        am.store(memory, &mut self.registers, value);
        self.check_zero_or_negative(value);
    }

    /// Increment X register.
    fn inx(&mut self) {
        self.registers.x = self.registers.x.wrapping_add(1);
        self.check_zero_or_negative(self.registers.x);
    }

    /// Increment Y register.
    fn iny(&mut self) {
        self.registers.y = self.registers.y.wrapping_add(1);
        self.check_zero_or_negative(self.registers.y);
    }

    /// Jump.
    fn jmp(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        self.registers.pc = am.address(memory, &mut self.registers);
    }

    /// Jump to subroutine.
    fn jsr(&mut self, am: Absolute, memory: &mut dyn Bus) {
        let ret = self.registers.pc - 1u8;
        let [low, high] = <[u8; 2]>::from(ret);
        self.push_stack(memory, high);
        self.push_stack(memory, low);
        self.registers.pc = am.address(memory, &mut self.registers);
    }

    /// Load accumulator.
    fn lda(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let value = am.load(memory, &self.registers);
        self.registers.a = value;
        self.check_zero_or_negative(value);
    }

    /// Load X register.
    fn ldx(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let value = am.load(memory, &self.registers);
        self.registers.x = value;
        self.check_zero_or_negative(value);
    }

    /// Load Y register.
    fn ldy(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let value = am.load(memory, &self.registers);
        self.registers.y = value;
        self.check_zero_or_negative(value);
    }

    /// Logical shift right.
    fn lsr(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let value = am.load(memory, &self.registers);
        let res = value >> 1;
        am.store(memory, &mut self.registers, res);

        self.registers.p.set(Flags::CARRY, value & 1 > 0);
        self.check_zero_or_negative(res);
    }

    /// Logical inclusive OR.
    fn ora(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let value = am.load(memory, &self.registers);
        self.registers.a |= value;
        self.check_zero_or_negative(self.registers.a);
    }

    /// Push accumulator.
    fn pha(&mut self, memory: &mut dyn Bus) {
        self.push_stack(memory, self.registers.a);
    }

    /// Push processor status.
    fn php(&mut self, memory: &mut dyn Bus) {
        let flags = self.registers.p | Flags::UNUSED | Flags::BREAK;
        self.push_stack(memory, flags.bits());
    }

    /// Pull accumulator.
    fn pla(&mut self, memory: &mut dyn Bus) {
        self.registers.a = self.pull_stack(memory);
        self.check_zero_or_negative(self.registers.a);
    }

    /// Pull processor status.
    fn plp(&mut self, memory: &mut dyn Bus) {
        self.pull_flags(memory);
    }

    /// Rotate left.
    fn rol(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let mut value = am.load(memory, &self.registers);

        // Current value of the carry flag, which will be
        // rotated into bit 0.
        let old_carry = self.registers.p.contains(Flags::CARRY) as u8;

        // Bit 7, which is about to be rotated out into the carry flag.
        let new_carry = value & (1 << 7) > 0;

        value = (value << 1) | old_carry;
        am.store(memory, &mut self.registers, value);

        self.registers.p.set(Flags::CARRY, new_carry);
        self.check_zero_or_negative(value);
    }

    /// Rotate right.
    fn ror(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let mut value = am.load(memory, &self.registers);

        // Current value of the carry flag, which will be rotated into bit 7.
        let old_carry = self.registers.p.contains(Flags::CARRY) as u8;

        // Bit 0, which is about to be rotated out into the carry flag.
        let new_carry = value & 1 > 0;

        value = (value >> 1) | (old_carry << 7);
        am.store(memory, &mut self.registers, value);

        self.registers.p.set(Flags::CARRY, new_carry);
        self.check_zero_or_negative(value);
    }

    /// Return from interrupt.
    fn rti(&mut self, memory: &mut dyn Bus) {
        self.pull_flags(memory);
        let low = self.pull_stack(memory);
        let high = self.pull_stack(memory);
        self.registers.pc = Address::from([low, high]);
    }

    /// Return from subroutine.
    fn rts(&mut self, memory: &mut dyn Bus) {
        let low = self.pull_stack(memory);
        let high = self.pull_stack(memory);
        self.registers.pc = Address::from([low, high]) + 1u8;
    }

    /// Subtract with carry.
    fn sbc(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let value = am.load(memory, &self.registers);
        let carry_in = !self.registers.p.contains(Flags::CARRY);

        // Cast the values to i16's before substracing so we can more easily
        // check for underflow.
        let res = self.registers.a as i16 - value as i16 - carry_in as i16;
        let carry_out = res >= 0;
        let res = res as u8;

        let overflow = twos_complement_overflow(self.registers.a, !value, res);
        self.registers.p.set(Flags::OVERFLOW, overflow);

        self.registers.a = res;
        self.registers.p.set(Flags::CARRY, carry_out);
        self.check_zero_or_negative(res);
    }

    /// Set carry flag.
    fn sec(&mut self) {
        self.registers.p.insert(Flags::CARRY);
    }

    /// Set decimal flag.
    fn sed(&mut self) {
        self.registers.p.insert(Flags::DECIMAL);
    }

    /// Set interrupt disable.
    fn sei(&mut self) {
        self.registers.p.insert(Flags::INTERRUPT_DISABLE);
    }

    /// Store accumulator.
    fn sta(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let value = self.registers.a;
        am.store(memory, &mut self.registers, value);
    }

    /// Store X register.
    fn stx(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let value = self.registers.x;
        am.store(memory, &mut self.registers, value);
    }

    /// Store Y register.
    fn sty(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let value = self.registers.y;
        am.store(memory, &mut self.registers, value);
    }

    /// Transfer accumulator to X.
    fn tax(&mut self) {
        let a = self.registers.a;
        self.registers.x = a;
        self.check_zero_or_negative(a);
    }

    /// Transfer accumulator to Y.
    fn tay(&mut self) {
        let a = self.registers.a;
        self.registers.y = a;
        self.check_zero_or_negative(a);
    }

    /// Transfer stack pointer to X.
    fn tsx(&mut self) {
        let s = self.registers.s;
        self.registers.x = s;
        self.check_zero_or_negative(s);
    }

    /// Transfer X to accumulator.
    fn txa(&mut self) {
        let x = self.registers.x;
        self.registers.a = x;
        self.check_zero_or_negative(x);
    }

    /// Transfer X to stack pointer.
    fn txs(&mut self) {
        let x = self.registers.x;
        self.registers.s = x;
    }

    /// Transfer Y to accumulator.
    fn tya(&mut self) {
        let y = self.registers.y;
        self.registers.a = y;
        self.check_zero_or_negative(y);
    }

    /// [UNDOCUMENTED] Decrement memory.
    fn undoc_dcp(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        self.dec(am.clone(), memory);
        self.cmp(am, memory);
    }

    /// [UNDOCUMENTED] Increment and subtract from accumulator.
    fn undoc_isb(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        self.inc(am.clone(), memory);
        self.sbc(am, memory);
    }

    /// [UNDOCUMENTED] Load accumulator and X register.
    fn undoc_lax(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        self.lda(am.clone(), memory);
        self.ldx(am, memory);
    }

    /// [UNDOCUMENTED] Rotate left then AND with accumulator.
    fn undoc_rla(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        self.rol(am.clone(), memory);
        self.and(am, memory);
    }

    /// [UNDOCUMENTED] Rotate right then add to accumulator.
    fn undoc_rra(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        self.ror(am.clone(), memory);
        self.adc(am, memory);
    }

    /// [UNDOCUMENTED] AND X register with accumulator and store result.
    fn undoc_sax(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        let value = self.registers.a & self.registers.x;
        am.store(memory, &mut self.registers, value);
    }

    /// [UNDOCUMENTED] Shift left then OR with accumulator.
    fn undoc_slo(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        self.asl(am.clone(), memory);
        self.ora(am, memory);
    }

    /// [UNDOCUMENTED] Shift right then XOR with accumulator.
    fn undoc_sre(&mut self, am: impl AddressingMode, memory: &mut dyn Bus) {
        self.lsr(am.clone(), memory);
        self.eor(am, memory);
    }
}

/// Check for two's complement overflow during addition or subtraction by
/// checking whether the sign bit of each operand matches that of the result.
/// See the folowing webpage for a more detailed explanation:
/// http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
#[inline]
fn twos_complement_overflow(n: u8, m: u8, res: u8) -> bool {
    (n ^ res) & (m ^ res) & (1 << 7) > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This test runs Klaus Dormann's 6502 test suite.
    ///
    /// Notably, the binary used here has been assebled with the decimal mode
    /// test disabled (since the NES's CPU does not support decimal mode).
    ///
    /// If a test fails, the program will enter an infinite loop (the address of
    /// which will specify which test failed when compared to the listing file).
    /// The emulated CPU will panic upon detecting an infinite loop and report
    /// the program counter value to assist with debugging.
    ///
    /// Upon successfull completion, the program would normally enter an
    /// infinite loop at address 0x3699. This test checks for this and returns
    ///
    /// https://github.com/Klaus2m5/6502_65C02_functional_tests
    #[test]
    fn cpu_functional_test() {
        let binary = include_bytes!("../../data/6502/6502_functional_test_padded.bin");
        let mut cpu = Cpu::new();
        cpu.run(&binary[..], Some(Address(0x400)), Some(Address(0x3699)));
    }
}
