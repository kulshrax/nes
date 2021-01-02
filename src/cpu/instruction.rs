use anyhow::{bail, Result};
use log;

use crate::mem::{Address, Memory};

use super::addressing::*;

#[derive(Debug, Copy, Clone)]
pub(super) enum Instruction {
    // ADC - Add with carry.
    AdcI(Immediate),
    AdcZ(ZeroPage),
    AdcZX(ZeroPageX),
    AdcA(Absolute),
    AdcAX(AbsoluteX),
    AdcAY(AbsoluteY),
    AdcIX(IndexedIndirect),
    AdcIY(IndirectIndexed),

    // AND - Logical AND.
    AndI(Immediate),
    AndZ(ZeroPage),
    AndZX(ZeroPageX),
    AndA(Absolute),
    AndAX(AbsoluteX),
    AndAY(AbsoluteY),
    AndIX(IndexedIndirect),
    AndIY(IndirectIndexed),

    // ASL - Arithmetic Shift Left.
    AslAcc(Accumulator),
    AslZ(ZeroPage),
    AslZX(ZeroPageX),
    AslA(Absolute),
    AslAX(AbsoluteX),

    // BCC - Branch if Carry Clear.
    Bcc(Relative),

    // BCS - Branch if Carry Set.
    Bcs(Relative),

    // BEQ - Branch if Equal.
    Beq(Relative),

    // BIT - Bit Test.
    BitZ(ZeroPage),
    BitA(Absolute),

    // BMI - Branch if Minus.
    Bmi(Relative),

    // BNE - Branch if Not Equal.
    Bne(Relative),

    // BPL - Branch if Positive.
    Bpl(Relative),

    // BRK - Force Interrupt.
    Brk,

    // BVC - Branch if Overflow Clear.
    Bvc(Relative),

    // BVS - Branch if Overflow Set.
    Bvs(Relative),

    // CLC - Clear Carry Flag.
    Clc,

    // CLD - Clear Decimal Mode.
    Cld,

    // CLI - Clear Interrupt Disable.
    Cli,

    // CLV - Clear Overflow Flag.
    Clv,

    // CMP - Compare.
    CmpI(Immediate),
    CmpZ(ZeroPage),
    CmpZX(ZeroPageX),
    CmpA(Absolute),
    CmpAX(AbsoluteX),
    CmpAY(AbsoluteY),
    CmpIX(IndexedIndirect),
    CmpIY(IndirectIndexed),

    // CPX - Compare X Register.
    CpxI(Immediate),
    CpxZ(ZeroPage),
    CpxA(Absolute),

    // CPY - Compare Y Register.
    CpyI(Immediate),
    CpyZ(ZeroPage),
    CpyA(Absolute),

    // DEC - Decrement Memory
    DecZ(ZeroPage),
    DecZX(ZeroPageX),
    DecA(Absolute),
    DecAX(AbsoluteX),

    // DEX - Decrement X Register.
    Dex,

    // DEY - Decrement Y Register.
    Dey,

    // EOR - Exclusive OR.
    EorI(Immediate),
    EorZ(ZeroPage),
    EorZX(ZeroPageX),
    EorA(Absolute),
    EorAX(AbsoluteX),
    EorAY(AbsoluteY),
    EorIX(IndexedIndirect),
    EorIY(IndirectIndexed),

    // INC - Increment Memory,
    IncZ(ZeroPage),
    IncZX(ZeroPageX),
    IncA(Absolute),
    IncAX(AbsoluteX),

    // INX - Increment X Register.
    Inx,

    // INY - Increment Y Register.
    Iny,

    // JMP - Jump.
    JmpA(Absolute),
    JmpI(Indirect),

    // JSR - Jump to Subroutine.
    Jsr(Absolute),

    // LDA - Load Accumulator.
    LdaI(Immediate),
    LdaZ(ZeroPage),
    LdaZX(ZeroPageX),
    LdaA(Absolute),
    LdaAX(AbsoluteX),
    LdaAY(AbsoluteY),
    LdaIX(IndexedIndirect),
    LdaIY(IndirectIndexed),

    // LDX - Load X Register.
    LdxI(Immediate),
    LdxZ(ZeroPage),
    LdxZY(ZeroPageY),
    LdxA(Absolute),
    LdxAY(AbsoluteY),

    // LDY - Load Y Register.
    LdyI(Immediate),
    LdyZ(ZeroPage),
    LdyZX(ZeroPageX),
    LdyA(Absolute),
    LdyAX(AbsoluteX),

    // LSR - Logical Right Shift.
    LsrAcc(Accumulator),
    LsrZ(ZeroPage),
    LsrZX(ZeroPageX),
    LsrA(Absolute),
    LsrAX(AbsoluteX),

    // NOP - No Operation.
    Nop,

    // ORA - Logical Inclusive OR.
    OraI(Immediate),
    OraZ(ZeroPage),
    OraZX(ZeroPageX),
    OraA(Absolute),
    OraAX(AbsoluteX),
    OraAY(AbsoluteY),
    OraIX(IndexedIndirect),
    OraIY(IndirectIndexed),

    // PHA - Push Accumulator.
    Pha,

    // PHP - Push Processor Status.
    Php,

    // PLA - Pull Accumulator.
    Pla,

    // PLP - Pull Processor Status.
    Plp,

    // ROL - Rotate Left.
    RolAcc(Accumulator),
    RolZ(ZeroPage),
    RolZX(ZeroPageX),
    RolA(Absolute),
    RolAX(AbsoluteX),

    // ROR - Rotate Right.
    RorAcc(Accumulator),
    RorZ(ZeroPage),
    RorZX(ZeroPageX),
    RorA(Absolute),
    RorAX(AbsoluteX),

    // RTI - Return from Interrupt.
    Rti,

    // RTS - Return from Subroutine.
    Rts,

    // SBC - Subtract with Carry.
    SbcI(Immediate),
    SbcZ(ZeroPage),
    SbcZX(ZeroPageX),
    SbcA(Absolute),
    SbcAX(AbsoluteX),
    SbcAY(AbsoluteY),
    SbcIX(IndexedIndirect),
    SbcIY(IndirectIndexed),

    // SEC - Set Carry Flag,
    Sec,

    // SED - Set Decimal Flag,
    Sed,

    // SEI - Set Interrupt Disable.
    Sei,

    // STA - Store Accumulator.
    StaZ(ZeroPage),
    StaZX(ZeroPageX),
    StaA(Absolute),
    StaAX(AbsoluteX),
    StaAY(AbsoluteY),
    StaIX(IndexedIndirect),
    StaIY(IndirectIndexed),

    // STX - Store X Register.
    StxZ(ZeroPage),
    StxZY(ZeroPageY),
    StxA(Absolute),

    // STY - Store X Register.
    StyZ(ZeroPage),
    StyZX(ZeroPageX),
    StyA(Absolute),

    // TAX - Transfer Accumulator to X.
    Tax,

    // TAY - Transfer Accumulator to Y.
    Tay,

    // TSX - Transfer Stack Pointer to X.
    Tsx,

    // TXA - Transfer X to Accumulator.
    Txa,

    // TXS - Transfer X to Stack Pointer.
    Txs,

    // TYA - Transfer Y to Accumulator.
    Tya,
}

impl Instruction {
    /// Fetch and decode an instruction from memory at the address
    /// of the given program counter. Instructions are 1 to 3 bytes
    /// long: a 1-byte opcode optionally followed by a one or two byte
    /// argument. This method will increment the program counter by
    /// the appropriate amount after decoding the instruction.
    pub(super) fn fetch(memory: &Memory, pc: &mut Address) -> Result<Self> {
        use Instruction::*;

        let start_pc = *pc;
        let opcode = memory.load(start_pc);
        *pc += 1u8;

        let instruction = match opcode {
            0x69 => AdcI(Immediate(read_byte(memory, pc))),
            0x65 => AdcZ(ZeroPage(read_byte(memory, pc))),
            0x75 => AdcZX(ZeroPageX(read_byte(memory, pc))),
            0x6D => AdcA(Absolute(read_addr(memory, pc))),
            0x7D => AdcAX(AbsoluteX(read_addr(memory, pc))),
            0x79 => AdcAY(AbsoluteY(read_addr(memory, pc))),
            0x61 => AdcIX(IndexedIndirect(read_byte(memory, pc))),
            0x71 => AdcIY(IndirectIndexed(read_byte(memory, pc))),
            0x29 => AndI(Immediate(read_byte(memory, pc))),
            0x25 => AndZ(ZeroPage(read_byte(memory, pc))),
            0x35 => AndZX(ZeroPageX(read_byte(memory, pc))),
            0x2D => AndA(Absolute(read_addr(memory, pc))),
            0x3D => AndAX(AbsoluteX(read_addr(memory, pc))),
            0x39 => AndAY(AbsoluteY(read_addr(memory, pc))),
            0x21 => AndIX(IndexedIndirect(read_byte(memory, pc))),
            0x31 => AndIY(IndirectIndexed(read_byte(memory, pc))),
            0x0A => AslAcc(Accumulator),
            0x06 => AslZ(ZeroPage(read_byte(memory, pc))),
            0x16 => AslZX(ZeroPageX(read_byte(memory, pc))),
            0x0E => AslA(Absolute(read_addr(memory, pc))),
            0x1E => AslAX(AbsoluteX(read_addr(memory, pc))),
            0x90 => Bcc(Relative(read_byte(memory, pc) as i8)),
            0xB0 => Bcs(Relative(read_byte(memory, pc) as i8)),
            0xF0 => Beq(Relative(read_byte(memory, pc) as i8)),
            0x24 => BitZ(ZeroPage(read_byte(memory, pc))),
            0x2C => BitA(Absolute(read_addr(memory, pc))),
            0x30 => Bmi(Relative(read_byte(memory, pc) as i8)),
            0xD0 => Bne(Relative(read_byte(memory, pc) as i8)),
            0x10 => Bpl(Relative(read_byte(memory, pc) as i8)),
            0x00 => Brk,
            0x50 => Bvc(Relative(read_byte(memory, pc) as i8)),
            0x70 => Bvs(Relative(read_byte(memory, pc) as i8)),
            0x18 => Clc,
            0xD8 => Cld,
            0x58 => Cli,
            0xB8 => Clv,
            0xC9 => CmpI(Immediate(read_byte(memory, pc))),
            0xC5 => CmpZ(ZeroPage(read_byte(memory, pc))),
            0xD5 => CmpZX(ZeroPageX(read_byte(memory, pc))),
            0xCD => CmpA(Absolute(read_addr(memory, pc))),
            0xDD => CmpAX(AbsoluteX(read_addr(memory, pc))),
            0xD9 => CmpAY(AbsoluteY(read_addr(memory, pc))),
            0xC1 => CmpIX(IndexedIndirect(read_byte(memory, pc))),
            0xD1 => CmpIY(IndirectIndexed(read_byte(memory, pc))),
            0xE0 => CpxI(Immediate(read_byte(memory, pc))),
            0xE4 => CpxZ(ZeroPage(read_byte(memory, pc))),
            0xEC => CpxA(Absolute(read_addr(memory, pc))),
            0xC0 => CpyI(Immediate(read_byte(memory, pc))),
            0xC4 => CpyZ(ZeroPage(read_byte(memory, pc))),
            0xCC => CpyA(Absolute(read_addr(memory, pc))),
            0xC6 => DecZ(ZeroPage(read_byte(memory, pc))),
            0xD6 => DecZX(ZeroPageX(read_byte(memory, pc))),
            0xCE => DecA(Absolute(read_addr(memory, pc))),
            0xDE => DecAX(AbsoluteX(read_addr(memory, pc))),
            0xCA => Dex,
            0x88 => Dey,
            0x49 => EorI(Immediate(read_byte(memory, pc))),
            0x45 => EorZ(ZeroPage(read_byte(memory, pc))),
            0x55 => EorZX(ZeroPageX(read_byte(memory, pc))),
            0x4D => EorA(Absolute(read_addr(memory, pc))),
            0x5D => EorAX(AbsoluteX(read_addr(memory, pc))),
            0x59 => EorAY(AbsoluteY(read_addr(memory, pc))),
            0x41 => EorIX(IndexedIndirect(read_byte(memory, pc))),
            0x51 => EorIY(IndirectIndexed(read_byte(memory, pc))),
            0xE6 => IncZ(ZeroPage(read_byte(memory, pc))),
            0xF6 => IncZX(ZeroPageX(read_byte(memory, pc))),
            0xEE => IncA(Absolute(read_addr(memory, pc))),
            0xFE => IncAX(AbsoluteX(read_addr(memory, pc))),
            0xE8 => Inx,
            0xC8 => Iny,
            0x4C => JmpA(Absolute(read_addr(memory, pc))),
            0x6C => JmpI(Indirect(read_addr(memory, pc))),
            0x20 => Jsr(Absolute(read_addr(memory, pc))),
            0xA9 => LdaI(Immediate(read_byte(memory, pc))),
            0xA5 => LdaZ(ZeroPage(read_byte(memory, pc))),
            0xB5 => LdaZX(ZeroPageX(read_byte(memory, pc))),
            0xAD => LdaA(Absolute(read_addr(memory, pc))),
            0xBD => LdaAX(AbsoluteX(read_addr(memory, pc))),
            0xB9 => LdaAY(AbsoluteY(read_addr(memory, pc))),
            0xA1 => LdaIX(IndexedIndirect(read_byte(memory, pc))),
            0xB1 => LdaIY(IndirectIndexed(read_byte(memory, pc))),
            0xA2 => LdxI(Immediate(read_byte(memory, pc))),
            0xA6 => LdxZ(ZeroPage(read_byte(memory, pc))),
            0xB6 => LdxZY(ZeroPageY(read_byte(memory, pc))),
            0xAE => LdxA(Absolute(read_addr(memory, pc))),
            0xBE => LdxAY(AbsoluteY(read_addr(memory, pc))),
            0xA0 => LdyI(Immediate(read_byte(memory, pc))),
            0xA4 => LdyZ(ZeroPage(read_byte(memory, pc))),
            0xB4 => LdyZX(ZeroPageX(read_byte(memory, pc))),
            0xAC => LdyA(Absolute(read_addr(memory, pc))),
            0xBC => LdyAX(AbsoluteX(read_addr(memory, pc))),
            0x4A => LsrAcc(Accumulator),
            0x46 => LsrZ(ZeroPage(read_byte(memory, pc))),
            0x56 => LsrZX(ZeroPageX(read_byte(memory, pc))),
            0x4E => LsrA(Absolute(read_addr(memory, pc))),
            0x5E => LsrAX(AbsoluteX(read_addr(memory, pc))),
            0xEA => Nop,
            0x09 => OraI(Immediate(read_byte(memory, pc))),
            0x05 => OraZ(ZeroPage(read_byte(memory, pc))),
            0x15 => OraZX(ZeroPageX(read_byte(memory, pc))),
            0x0D => OraA(Absolute(read_addr(memory, pc))),
            0x1D => OraAX(AbsoluteX(read_addr(memory, pc))),
            0x19 => OraAY(AbsoluteY(read_addr(memory, pc))),
            0x01 => OraIX(IndexedIndirect(read_byte(memory, pc))),
            0x11 => OraIY(IndirectIndexed(read_byte(memory, pc))),
            0x48 => Pha,
            0x08 => Php,
            0x68 => Pla,
            0x28 => Plp,
            0x2A => RolAcc(Accumulator),
            0x26 => RolZ(ZeroPage(read_byte(memory, pc))),
            0x36 => RolZX(ZeroPageX(read_byte(memory, pc))),
            0x2E => RolA(Absolute(read_addr(memory, pc))),
            0x3E => RolAX(AbsoluteX(read_addr(memory, pc))),
            0x6A => RorAcc(Accumulator),
            0x66 => RorZ(ZeroPage(read_byte(memory, pc))),
            0x76 => RorZX(ZeroPageX(read_byte(memory, pc))),
            0x6E => RorA(Absolute(read_addr(memory, pc))),
            0x7E => RorAX(AbsoluteX(read_addr(memory, pc))),
            0x40 => Rti,
            0x60 => Rts,
            0xE9 => SbcI(Immediate(read_byte(memory, pc))),
            0xE5 => SbcZ(ZeroPage(read_byte(memory, pc))),
            0xF5 => SbcZX(ZeroPageX(read_byte(memory, pc))),
            0xED => SbcA(Absolute(read_addr(memory, pc))),
            0xFD => SbcAX(AbsoluteX(read_addr(memory, pc))),
            0xF9 => SbcAY(AbsoluteY(read_addr(memory, pc))),
            0xE1 => SbcIX(IndexedIndirect(read_byte(memory, pc))),
            0xF1 => SbcIY(IndirectIndexed(read_byte(memory, pc))),
            0x38 => Sec,
            0xF8 => Sed,
            0x78 => Sei,
            0x85 => StaZ(ZeroPage(read_byte(memory, pc))),
            0x95 => StaZX(ZeroPageX(read_byte(memory, pc))),
            0x8D => StaA(Absolute(read_addr(memory, pc))),
            0x9D => StaAX(AbsoluteX(read_addr(memory, pc))),
            0x99 => StaAY(AbsoluteY(read_addr(memory, pc))),
            0x81 => StaIX(IndexedIndirect(read_byte(memory, pc))),
            0x91 => StaIY(IndirectIndexed(read_byte(memory, pc))),
            0x86 => StxZ(ZeroPage(read_byte(memory, pc))),
            0x96 => StxZY(ZeroPageY(read_byte(memory, pc))),
            0x8E => StxA(Absolute(read_addr(memory, pc))),
            0x84 => StyZ(ZeroPage(read_byte(memory, pc))),
            0x94 => StyZX(ZeroPageX(read_byte(memory, pc))),
            0x8C => StyA(Absolute(read_addr(memory, pc))),
            0xAA => Tax,
            0xA8 => Tay,
            0xBA => Tsx,
            0x8A => Txa,
            0x9A => Txs,
            0x98 => Tya,
            illegal => bail!("Illegal opcode at address {}: {:#X}", start_pc, illegal),
        };

        log::trace!(
            "PC: {}; OP: {:#X}; Instruction: {:?}",
            start_pc,
            opcode,
            instruction
        );

        Ok(instruction)
    }
}

/// Read a 16-bit little endian address from memory at the
/// location of the current program counter, incrementing
/// the program counter by two.
fn read_addr(memory: &Memory, pc: &mut Address) -> Address {
    let lsb = memory.load(*pc);
    let msb = memory.load(*pc + 1u8);
    *pc += 2u8;
    Address::from([lsb, msb])
}

/// Read a byte from memory at the location of the current
/// program coutner, incrementing the program counter by one.
fn read_byte(memory: &Memory, pc: &mut Address) -> u8 {
    let byte = memory.load(*pc);
    *pc += 1u8;
    byte
}
