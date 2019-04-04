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

use addressing::*;
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

    fn exec(&mut self, _memory: &mut Memory, op: Instruction) {
        match op {
            _ => self.registers.pc += 1u8,
        }
    }
}

/// Methods corresponding to operations in the MOS 6502 instruction set.
///
/// See http://obelisk.me.uk/6502/reference.html for details about
/// each instruction.
impl Cpu {
    /// Add with carry.
    fn adc(&mut self, _memory: &mut Memory) {}

    /// Logical AND.
    fn and(&mut self, _memory: &mut Memory) {}

    /// Arithmetic left shift.
    fn asl(&mut self, _memory: &mut Memory) {}

    /// Branch if carry clear.
    fn bcc(&mut self, _memory: &mut Memory) {}

    /// Branch if carry set.
    fn bcs(&mut self, _memory: &mut Memory) {}

    /// Branch if equal.
    fn beq(&mut self, _memory: &mut Memory) {}

    /// Bit test.
    fn bit(&mut self, _memory: &mut Memory) {}

    /// Branch if minus.
    fn bmi(&mut self, _memory: &mut Memory) {}

    /// Branch if not equal.
    fn bne(&mut self, _memory: &mut Memory) {}

    /// Branch if positive.
    fn bpl(&mut self, _memory: &mut Memory) {}

    /// Force interrupt.
    fn brk(&mut self, _memory: &mut Memory) {}

    /// Branch if overflow clear.
    fn bvc(&mut self, _memory: &mut Memory) {}

    /// Branch if overflow set.
    fn bvs(&mut self, _memory: &mut Memory) {}

    /// Clear carry flag.
    fn clc(&mut self, _memory: &mut Memory) {
        self.registers.p.remove(Flags::CARRY);
    }

    /// Clear decimal mode.
    fn cld(&mut self, _memory: &mut Memory) {
        self.registers.p.remove(Flags::DECIMAL);
    }

    /// Clear interrupt disable.
    fn cli(&mut self, _memory: &mut Memory) {
        self.registers.p.remove(Flags::INTERRUPT_DISABLE);
    }

    /// Clear overflow flag.
    fn clv(&mut self, _memory: &mut Memory) {
        self.registers.p.remove(Flags::OVERFLOW);
    }

    /// Compare.
    fn cmp(&mut self, _memory: &mut Memory) {}

    /// Compare X register.
    fn cpx(&mut self, _memory: &mut Memory) {}

    /// Compare Y register.
    fn cpy(&mut self, _memory: &mut Memory) {}

    /// Decrement memory.
    fn dec(&mut self, _memory: &mut Memory) {}

    /// Decrement X register.
    fn dex(&mut self, _memory: &mut Memory) {}

    /// Decrement Y register.
    fn dey(&mut self, _memory: &mut Memory) {}

    /// Exclusive OR.
    fn eor(&mut self, _memory: &mut Memory) {}

    /// Incrememnt memory.
    fn inc(&mut self, _memory: &mut Memory) {}

    /// Increment X register.
    fn inx(&mut self, _memory: &mut Memory) {}

    /// Increment Y register.
    fn iny(&mut self, _memory: &mut Memory) {}

    /// Jump.
    fn jmp(&mut self, _memory: &mut Memory) {}

    /// Jump to subroutine.
    fn jsr(&mut self, _memory: &mut Memory) {}

    /// Load accumulator.
    fn lda(&mut self, _memory: &mut Memory) {}

    /// Load X register.
    fn ldx(&mut self, _memory: &mut Memory) {}

    /// Load Y register.
    fn ldy(&mut self, _memory: &mut Memory) {}

    /// Logical shift right.
    fn lsr(&mut self, _memory: &mut Memory) {}

    /// No operation.
    fn nop(&mut self, _memory: &mut Memory) {}

    /// Logical inclusive OR.
    fn ora(&mut self, _memory: &mut Memory) {}

    /// Push accumulator.
    fn pha(&mut self, _memory: &mut Memory) {}

    /// Push processor status.
    fn php(&mut self, _memory: &mut Memory) {}

    /// Pull accumulator.
    fn pla(&mut self, _memory: &mut Memory) {}

    /// Pull processor status.
    fn plp(&mut self, _memory: &mut Memory) {}

    /// Rotate left.
    fn rol(&mut self, _memory: &mut Memory) {}

    /// Rotate right.
    fn ror(&mut self, _memory: &mut Memory) {}

    /// Return from interrupt.
    fn rti(&mut self, _memory: &mut Memory) {}

    /// Return from subroutine.
    fn rts(&mut self, _memory: &mut Memory) {}

    /// Subtract with carry.
    fn sbc(&mut self, _memory: &mut Memory) {}

    /// Set carry flag.
    fn sec(&mut self, _memory: &mut Memory) {
        self.registers.p.insert(Flags::CARRY);
    }

    /// Set decimal flag.
    fn sed(&mut self, _memory: &mut Memory) {
        self.registers.p.insert(Flags::DECIMAL);
    }

    /// Set interrupt disable.
    fn sei(&mut self, _memory: &mut Memory) {
        self.registers.p.insert(Flags::INTERRUPT_DISABLE);
    }

    /// Store accumulator.
    fn sta(&mut self, _memory: &mut Memory) {}

    /// Store X register.
    fn stx(&mut self, _memory: &mut Memory) {}

    /// Store Y register.
    fn sty(&mut self, _memory: &mut Memory) {}

    /// Transfer accumulator to X.
    fn tax(&mut self, _memory: &mut Memory) {
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
    fn tay(&mut self, _memory: &mut Memory) {
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
    fn tsx(&mut self, _memory: &mut Memory) {
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
    fn txa(&mut self, _memory: &mut Memory) {
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
    fn txs(&mut self, _memory: &mut Memory) {
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
    fn tya(&mut self, _memory: &mut Memory) {
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
