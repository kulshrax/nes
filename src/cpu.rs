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

/// The 6502 has a 256-byte stack address space that is fixed
/// at memory page 1 (addresses 0x100 to 0x1FF). The stack
/// starts at 0x1FF and grows downward as values are pushed.
/// The next free location on the stack is pointed at by the
/// S register, which contains the low byte of the next
/// available stack address. There is no overflow checking
/// for the call stack.
const STACK_START: u16 = 0x0100;

/// Emulated MOS 6502 CPU.
pub struct Cpu {
    registers: Registers,
}

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

    fn stack(&self) -> Address {
        Address::from(STACK_START) + self.registers.s
    }

    fn check_zero_or_negative(&mut self, value: u8) {
        if value == 0 {
            self.registers.p.insert(Flags::ZERO);
        } else if value > 127 {
            self.registers.p.insert(Flags::NEGATIVE);
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
    fn and(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let value = am.load(memory, &mut self.registers);
        self.registers.a &= value;
        self.check_zero_or_negative(self.registers.a);
    }

    /// Arithmetic left shift.
    fn asl(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let value = am.load(memory, &mut self.registers);
        let res = value << 1;
        am.store(memory, &mut self.registers, res);

        self.registers.p.set(Flags::CARRY, value & (1 << 7) > 0);
        self.check_zero_or_negative(res);
    }

    /// Branch if carry clear.
    fn bcc(&mut self, am: Relative, memory: &mut Memory) {
        if !self.registers.p.contains(Flags::CARRY) {
            let addr = am.address(memory, &mut self.registers);
            self.registers.pc = addr;
        }
    }

    /// Branch if carry set.
    fn bcs(&mut self, am: Relative, memory: &mut Memory) {
        if self.registers.p.contains(Flags::CARRY) {
            let addr = am.address(memory, &mut self.registers);
            self.registers.pc = addr;
        }
    }

    /// Branch if equal.
    fn beq(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        if self.registers.p.contains(Flags::ZERO) {
            let addr = am.address(memory, &mut self.registers);
            self.registers.pc = addr;
        }
    }

    /// Bit test.
    fn bit(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let value = am.load(memory, &mut self.registers);
        let res = self.registers.a & value;
        self.registers.p.set(Flags::ZERO, res == 0);
        self.registers.p.set(Flags::OVERFLOW, value & (1 << 6) > 0);
        self.registers.p.set(Flags::NEGATIVE, value & (1 << 7) > 0);
    }

    /// Branch if minus.
    fn bmi(&mut self, am: Relative, memory: &mut Memory) {
        if self.registers.p.contains(Flags::NEGATIVE) {
            let addr = am.address(memory, &mut self.registers);
            self.registers.pc = addr;
        }
    }

    /// Branch if not equal.
    fn bne(&mut self, am: Relative, memory: &mut Memory) {
        if !self.registers.p.contains(Flags::ZERO) {
            let addr = am.address(memory, &mut self.registers);
            self.registers.pc = addr;
        }
    }

    /// Branch if positive.
    fn bpl(&mut self, am: Relative, memory: &mut Memory) {
        if !self.registers.p.contains(Flags::NEGATIVE) {
            let addr = am.address(memory, &mut self.registers);
            self.registers.pc = addr;
        }
    }

    /// Force interrupt.
    fn brk(&mut self) {
        unimplemented!()
    }

    /// Branch if overflow clear.
    fn bvc(&mut self, am: Relative, memory: &mut Memory) {
        if !self.registers.p.contains(Flags::OVERFLOW) {
            let addr = am.address(memory, &mut self.registers);
            self.registers.pc = addr;
        }
    }

    /// Branch if overflow set.
    fn bvs(&mut self, am: Relative, memory: &mut Memory) {
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
    fn eor(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let value = am.load(memory, &mut self.registers);
        self.registers.a ^= value;
        self.check_zero_or_negative(self.registers.a);
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
    fn lda(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let value = am.load(memory, &self.registers);
        self.registers.a = value;
        self.check_zero_or_negative(value);
    }

    /// Load X register.
    fn ldx(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let value = am.load(memory, &self.registers);
        self.registers.x = value;
        self.check_zero_or_negative(value);
    }

    /// Load Y register.
    fn ldy(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let value = am.load(memory, &self.registers);
        self.registers.y = value;
        self.check_zero_or_negative(value);
    }

    /// Logical shift right.
    fn lsr(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let value = am.load(memory, &mut self.registers);
        let res = value >> 1;
        am.store(memory, &mut self.registers, res);

        self.registers.p.set(Flags::CARRY, value & 1 > 0);
        self.check_zero_or_negative(res);
    }

    /// No operation.
    fn nop(&mut self) {}

    /// Logical inclusive OR.
    fn ora(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let value = am.load(memory, &mut self.registers);
        self.registers.a |= value;
        self.check_zero_or_negative(self.registers.a);
    }

    /// Push accumulator.
    fn pha(&mut self, memory: &mut Memory) {
        memory.store(self.stack(), self.registers.a);
        self.registers.s -= 1;
    }

    /// Push processor status.
    fn php(&mut self, memory: &mut Memory) {
        memory.store(self.stack(), self.registers.p.bits());
        self.registers.s -= 1;
    }

    /// Pull accumulator.
    fn pla(&mut self, memory: &mut Memory) {
        self.registers.s += 1;
        self.registers.a = memory.load(self.stack());
    }

    /// Pull processor status.
    fn plp(&mut self, memory: &mut Memory) {
        self.registers.s += 1;
        let bits = memory.load(self.stack());
        self.registers.p = Flags::from_bits_truncate(bits);
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
    fn sta(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let value = self.registers.a;
        am.store(memory, &mut self.registers, value);
    }

    /// Store X register.
    fn stx(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let value = self.registers.x;
        am.store(memory, &mut self.registers, value);
    }

    /// Store Y register.
    fn sty(&mut self, am: impl AddressingMode, memory: &mut Memory) {
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
        self.check_zero_or_negative(x);
    }

    /// Transfer Y to accumulator.
    fn tya(&mut self) {
        let y = self.registers.y;
        self.registers.a = y;
        self.check_zero_or_negative(y);
    }
}
