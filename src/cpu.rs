//! An emulated MOS 6502 CPU.
//!
//! The NES uses an 8-bit Ricoh 2A03 CPU running at 1.79 MHz (for the NTSC
//! version of the console). The chip includes a CPU core based on the MOS
//! 6502 CPU (modified to disable decimal mode) along with an audio
//! processing unit (APU) for audio generation.
//!
//! This module implements an emulator for the MOS 6502, supporting all
//! of the official opcodes in the CPU's instruction set. (Some NES games
//! rely on unofficial opcodes outside of the documented instruction set;
//! these games will not work with this emulator, as this implementation
//! treats unofficial opcodes as illegal instructions.)
//!
//! Many thanks to Andrew Jacobs, whose introductory guide on the MOS
//! 6502 (http://www.obelisk.me.uk/6502/) was an invaluable resource
//! for this implementation.

use crate::mem::{Address, Memory};

mod addressing;
mod instruction;
mod registers;

use addressing::{AddressingMode, Relative};
use instruction::Instruction;
use registers::{Flags, Registers};

/// Emulated MOS 6502 CPU.
pub struct Cpu {
    registers: Registers,
}

/// Public API.
impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
        }
    }

    // Manually set the CPU's program counter.
    pub fn set_pc(&mut self, pc: Address) {
        self.registers.pc = pc;
    }

    pub fn step(&mut self, memory: &mut Memory) {
        let op = Instruction::fetch(memory, &mut self.registers.pc);
        self.exec(memory, op);
    }

    fn exec(&mut self, memory: &mut Memory, op: Instruction) {
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
            Brk => self.brk(),
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
            Jsr(am) => self.lsr(am, memory),
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
            Nop => self.nop(),
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
        }
    }
}

/// Methods corresponding to operations in the MOS 6502 instruction set.
///
/// See http://obelisk.me.uk/6502/reference.html for details about
/// each instruction.
impl Cpu {
    /// Add with carry.
    fn adc(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Logical AND.
    fn and(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Arithmetic left shift.
    fn asl(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Branch if carry clear.
    fn bcc(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Branch if carry set.
    fn bcs(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Branch if equal.
    fn beq(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Bit test.
    fn bit(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Branch if minus.
    fn bmi(&mut self, _am: Relative, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Branch if not equal.
    fn bne(&mut self, _am: Relative, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Branch if positive.
    fn bpl(&mut self, _am: Relative, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Force interrupt.
    fn brk(&mut self) {
        unimplemented!()
    }

    /// Branch if overflow clear.
    fn bvc(&mut self, _am: Relative, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Branch if overflow set.
    fn bvs(&mut self, _am: Relative, _memory: &mut Memory) {
        unimplemented!()
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
    fn cmp(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Compare X register.
    fn cpx(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Compare Y register.
    fn cpy(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Decrement memory.
    fn dec(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Decrement X register.
    fn dex(&mut self) {
        unimplemented!()
    }

    /// Decrement Y register.
    fn dey(&mut self) {
        unimplemented!()
    }

    /// Exclusive OR.
    fn eor(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Incrememnt memory.
    fn inc(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Increment X register.
    fn inx(&mut self) {
        unimplemented!()
    }

    /// Increment Y register.
    fn iny(&mut self) {
        unimplemented!()
    }

    /// Jump.
    fn jmp(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Jump to subroutine.
    fn jsr(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Load accumulator.
    fn lda(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Load X register.
    fn ldx(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Load Y register.
    fn ldy(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Logical shift right.
    fn lsr(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// No operation.
    fn nop(&mut self) {}

    /// Logical inclusive OR.
    fn ora(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Push accumulator.
    fn pha(&mut self, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Push processor status.
    fn php(&mut self, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Pull accumulator.
    fn pla(&mut self, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Pull processor status.
    fn plp(&mut self, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Rotate left.
    fn rol(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Rotate right.
    fn ror(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Return from interrupt.
    fn rti(&mut self, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Return from subroutine.
    fn rts(&mut self, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Subtract with carry.
    fn sbc(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
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
    fn sta(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Store X register.
    fn stx(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Store Y register.
    fn sty(&mut self, _am: impl AddressingMode, _memory: &mut Memory) {
        unimplemented!()
    }

    /// Transfer accumulator to X.
    fn tax(&mut self) {
        let a = self.registers.a;
        self.registers.x = a;

        if a == 0 {
            self.registers.p.insert(Flags::ZERO);
        }

        if a > 128 {
            self.registers.p.insert(Flags::NEGATIVE);
        }
    }

    /// Transfer accumulator to Y.
    fn tay(&mut self) {
        let a = self.registers.a;
        self.registers.y = a;

        if a == 0 {
            self.registers.p.insert(Flags::ZERO);
        }

        if a > 128 {
            self.registers.p.insert(Flags::NEGATIVE);
        }
    }

    /// Transfer stack pointer to X.
    fn tsx(&mut self) {
        let s = self.registers.s;
        self.registers.x = s;

        if s == 0 {
            self.registers.p.insert(Flags::ZERO);
        }

        if s > 128 {
            self.registers.p.insert(Flags::NEGATIVE);
        }
    }

    /// Transfer X to accumulator.
    fn txa(&mut self) {
        let x = self.registers.x;
        self.registers.a = x;

        if x == 0 {
            self.registers.p.insert(Flags::ZERO);
        }

        if x > 128 {
            self.registers.p.insert(Flags::NEGATIVE);
        }
    }

    /// Transfer X to stack pointer.
    fn txs(&mut self) {
        let x = self.registers.x;
        self.registers.s = x;

        if x == 0 {
            self.registers.p.insert(Flags::ZERO);
        }

        if x > 128 {
            self.registers.p.insert(Flags::NEGATIVE);
        }
    }

    /// Transfer Y to accumulator.
    fn tya(&mut self) {
        let y = self.registers.y;
        self.registers.a = y;

        if y == 0 {
            self.registers.p.insert(Flags::ZERO);
        }

        if y > 128 {
            self.registers.p.insert(Flags::NEGATIVE);
        }
    }
}
