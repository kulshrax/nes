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

use std::{fmt::Debug, num::Wrapping};

use bitflags::bitflags;

use crate::mem::{Address, Memory};

#[derive(Default)]
pub struct Registers {
    // Accumulator.
    a: u8,

    // Index registers.
    x: u8,
    y: u8,

    // Stack pointer.
    s: u8,

    // Program counter.
    pc: Address,

    // Status register.
    p: Flags,
}

impl Registers {
    pub fn new() -> Self {
        Default::default()
    }
}

bitflags! {
    /// Values corresponding to the bit flags stored
    /// in the status (P) register.
    struct Flags: u8 {
        /// Indicates that the last operation resulted in an overflow
        /// from bit 8 or an underflow from bit 0.
        const CARRY = 1;

        /// Indicates that the result of the last operation was zero.
        const ZERO = 1 << 1;

        /// When set, the CPU will not respond to hardware interrupts.
        /// This flag is set by the "set interrupt disable" (SEI)
        /// instruction and remains in effect until cleared by the
        /// "clear interrupt disable" (CLI) instruction.
        const INTERRUPT_DISABLE = 1 << 2;

        /// Specifies that the CPU should use binary coded decimal
        /// for arithmetic operations. This mode is disabled in the
        /// Ricoh 2A03 CPU used by the NES, but this flag is included
        /// here for completeness.
        const DECIMAL = 1 << 3;

        /// Indicates that a BRK instruction has been executed
        /// and an interrupt has been generated to process it.
        const BREAK = 1 << 4;

        /// This bit is officially unused in the MOS 6502.
        const UNUSED = 1 << 5;

        /// Indicates that the result of an arithmetic operation on
        /// signed values would result in an invalid twos-complement
        /// result (such as the addition of two positive values resulting
        /// in a negative value due to an overflow into the sign bit).
        const OVERFLOW = 1 << 6;

        /// Indicates that the result of the last operation was negative.
        /// (Specifically, that the sign bit (i.e., bit 7) was set to 1.)
        const NEGATIVE = 1 << 7;
    }
}

impl Default for Flags {
    fn default() -> Self {
        Flags::INTERRUPT_DISABLE | Flags::UNUSED
    }
}

trait AddressingMode: Debug {
    /// Load a value from the location specified by this addressing mode.
    /// This may be loaded from a location in memory, from a register,
    /// from an immediate value, or from a combination of these (in the
    /// case of indexed and indirect addressing modes).
    fn load(&self, _memory: &Memory, _registers: &Registers) -> u8 {
        panic!("Loads not supported for address mode: {:?}", &self);
    }

    /// Store a value to the location specified by this addressing mode.
    /// This may be loaded from a location in memory, from a register,
    /// from an immediate value, or from a combination of these (in the
    /// case of indexed and indirect addressing modes).
    fn store(&self, _memory: &mut Memory, _registers: &mut Registers, _value: u8) {
        panic!("Stores not supported for address mode: {:?}", &self);
    }
}

/// Implicit addressing denotes instructions which do not require a source
/// or destination to be specified because this information is implied by
/// the function of the instruction itself. (e.g., "clear carry flag (CLC)")
/// As such, this mode implements neither load nor store.
#[derive(Copy, Clone, Debug)]
struct Implicit;
impl AddressingMode for Implicit {}

/// Accumulator addresssing means that the instruction should load
/// or store a value directly to/from the A register.
#[derive(Copy, Clone, Debug)]
struct Accumulator;
impl AddressingMode for Accumulator {
    fn load(&self, _memory: &Memory, registers: &Registers) -> u8 {
        registers.a
    }

    fn store(&self, _memory: &mut Memory, registers: &mut Registers, value: u8) {
        registers.a = value;
    }
}

/// Immediate addressing denotes that an immediate value was given
/// as part of the instruction. As such, a load should just directly
/// use the immediate value. Stores do not make sense in this mode.
#[derive(Copy, Clone, Debug)]
struct Immediate(u8);
impl AddressingMode for Immediate {
    fn load(&self, _memory: &Memory, _registers: &Registers) -> u8 {
        self.0
    }
}

/// Zero page addressing uses an 8-bit address to refer to a location in the
/// first 256 bytes of memory, which is referred to as the "zero page" since
/// the most significant byte of addresses on this page is always zero.
///
/// Since the first byte of the address is known to be zero, it can be
/// omitted from the argument, making the instruction shorter and faster
/// to execute (since fewer memory fetches are required during execution).
#[derive(Copy, Clone, Debug)]
struct ZeroPage(u8);
impl AddressingMode for ZeroPage {
    fn load(&self, memory: &Memory, _registers: &Registers) -> u8 {
        memory.load(self.0 as Address)
    }

    fn store(&self, memory: &mut Memory, _registers: &mut Registers, value: u8) {
        memory.store(self.0 as Address, value);
    }
}

/// X-indexed zero page addressing takes an 8-bit immediate value,
/// adds it to the current value of the X register (wrapping around if
/// the sum exceeds 0xFF), and interprets the result as an 8-bit zero
/// page address, which is then used to load/store the given value.
#[derive(Copy, Clone, Debug)]
struct ZeroPageX(u8);
impl AddressingMode for ZeroPageX {
    fn load(&self, memory: &Memory, registers: &Registers) -> u8 {
        let addr = Wrapping(registers.x) + Wrapping(self.0);
        memory.load(addr.0 as Address)
    }

    fn store(&self, memory: &mut Memory, registers: &mut Registers, value: u8) {
        let addr = Wrapping(registers.x) + Wrapping(self.0);
        memory.store(addr.0 as Address, value)
    }
}

/// Y-indexed zero page addressing takes an 8-bit immediate value,
/// adds it to the current value of the Y register (wrapping around if
/// the sum exceeds 0xFF), and interprets the result as an 8-bit zero
/// page address, which is then used to load/store the given value.
#[derive(Copy, Clone, Debug)]
struct ZeroPageY(u8);
impl AddressingMode for ZeroPageY {
    fn load(&self, _memory: &Memory, _registers: &Registers) -> u8 {
        unimplemented!();
    }

    fn store(&self, _memory: &mut Memory, _registers: &mut Registers, _value: u8) {
        unimplemented!();
    }
}

/// Absolute addressing means that the instruction 's operand consists
/// of the exact 16-bit address of the target value.
#[derive(Copy, Clone, Debug)]
struct Absolute(Address);
impl AddressingMode for Absolute {
    fn load(&self, memory: &Memory, _registers: &Registers) -> u8 {
        memory.load(self.0)
    }

    fn store(&self, memory: &mut Memory, _registers: &mut Registers, value: u8) {
        memory.store(self.0, value);
    }
}

#[derive(Copy, Clone, Debug)]
struct AbsoluteX(Address);
impl AddressingMode for AbsoluteX {
    fn load(&self, _memory: &Memory, _registers: &Registers) -> u8 {
        unimplemented!();
    }

    fn store(&self, _memory: &mut Memory, _registers: &mut Registers, _value: u8) {
        unimplemented!();
    }
}

#[derive(Copy, Clone, Debug)]
struct AbsoluteY(Address);
impl AddressingMode for AbsoluteY {
    fn load(&self, _memory: &Memory, _registers: &Registers) -> u8 {
        unimplemented!();
    }

    fn store(&self, _memory: &mut Memory, _registers: &mut Registers, _value: u8) {
        unimplemented!();
    }
}

#[derive(Copy, Clone, Debug)]
struct Indirect(Address);
impl AddressingMode for Indirect {
    fn load(&self, _memory: &Memory, _registers: &Registers) -> u8 {
        unimplemented!();
    }

    fn store(&self, _memory: &mut Memory, _registers: &mut Registers, _value: u8) {
        unimplemented!();
    }
}

#[derive(Copy, Clone, Debug)]
struct IndexedIndirect();
impl AddressingMode for IndexedIndirect {
    fn load(&self, _memory: &Memory, _registers: &Registers) -> u8 {
        unimplemented!();
    }

    fn store(&self, _memory: &mut Memory, _registers: &mut Registers, _value: u8) {
        unimplemented!();
    }
}

#[derive(Copy, Clone, Debug)]
struct IndirectIndexed();
impl AddressingMode for IndirectIndexed {
    fn load(&self, _memory: &Memory, _registers: &Registers) -> u8 {
        unimplemented!();
    }

    fn store(&self, _memory: &mut Memory, _registers: &mut Registers, _value: u8) {
        unimplemented!();
    }
}

pub enum Instruction {
    Ldx,
}

impl Instruction {
    /// Fetch and decode an instruction from memory at the address
    /// of the given program counter. Instructions are generally 1 to
    /// 3 bytes long: a 1-byte opcode followed by a one or two byte
    /// argument. This method will increment the program counter by
    /// the appropriate amount after decoding the instruction.
    fn fetch(memory: &Memory, pc: &mut Address) -> Self {
        let opcode = memory.load(*pc);
        match opcode {
            illegal => panic!("Illegal opcode: {:#X}", illegal),
        }
    }
}

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
            _ => self.registers.pc += 1,
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
