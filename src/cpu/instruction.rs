use crate::mem::{Address, Bus};

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

    // DEC - Decrement Memory.
    DecZ(ZeroPage),
    DecZX(ZeroPageX),
    DecA(Absolute),
    DecAX(AbsoluteX),

    // DEX - Decrement X Register.
    Dex,

    // DEY - Decrement Y Register.
    Dey,

    // EOR - Exclusive OR (XOR).
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

    // =========================================================================
    // UNDOCUMENTED INSTRUCTIONS
    // =========================================================================
    //
    // All instructions after this point are undocumented. They correspond to
    // illegal opcodes, but nonetheless behave in predicable ways. Although few
    // games actually use these instrictions, many emulator test suites (such as
    // nestest) do check that these opcodes work correctly.
    //
    // All undocumented instructions are prefixed with 'U', which conventiently
    // causes them to be lexographically sorted after the official instructions.
    // =========================================================================

    // DCP - Decrement memory.
    UDcpZ(ZeroPage),
    UDcpZX(ZeroPageX),
    UDcpA(Absolute),
    UDcpAX(AbsoluteX),
    UDcpAY(AbsoluteY),
    UDcpIX(IndexedIndirect),
    UDcpIY(IndirectIndexed),

    // ISB - Increment memory and subtract value from accumulator.
    UIsbZ(ZeroPage),
    UIsbZX(ZeroPageX),
    UIsbA(Absolute),
    UIsbAX(AbsoluteX),
    UIsbAY(AbsoluteY),
    UIsbIX(IndexedIndirect),
    UIsbIY(IndirectIndexed),

    // LAX - Load accumulator and X register.
    ULaxZ(ZeroPage),
    ULaxZY(ZeroPageY),
    ULaxA(Absolute),
    ULaxAY(AbsoluteY),
    ULaxIX(IndexedIndirect),
    ULaxIY(IndirectIndexed),

    // NOP - Illegal NOP.
    UNop,

    // NOP - NOP that loads (and discards) a value.
    UNopI(Immediate),
    UNopZ(ZeroPage),
    UNopZX(ZeroPageX),
    UNopA(Absolute),
    UNopAX(AbsoluteX),

    // RLA - Rotate left, then AND the value with the accumulator.
    URlaZ(ZeroPage),
    URlaZX(ZeroPageX),
    URlaA(Absolute),
    URlaAX(AbsoluteX),
    URlaAY(AbsoluteY),
    URlaIX(IndexedIndirect),
    URlaIY(IndirectIndexed),

    // RRA - Rotate right, then add the value to the accumulator.
    URraZ(ZeroPage),
    URraZX(ZeroPageX),
    URraA(Absolute),
    URraAX(AbsoluteX),
    URraAY(AbsoluteY),
    URraIX(IndexedIndirect),
    URraIY(IndirectIndexed),

    // SAX - AND X register with accumulator, then write result to memory.
    USaxZ(ZeroPage),
    USaxZY(ZeroPageY),
    USaxA(Absolute),
    USaxIX(IndexedIndirect),

    // SBC - Subtract with carry. (Same as legal SBC instruction.)
    USbcI(Immediate),

    // SLO - Left shift, then OR the value with the accumulator.
    USloZ(ZeroPage),
    USloZX(ZeroPageX),
    USloA(Absolute),
    USloAX(AbsoluteX),
    USloAY(AbsoluteY),
    USloIX(IndexedIndirect),
    USloIY(IndirectIndexed),

    // SRE - Right shift, then XOR (EOR) the value with the accumulator.
    USreZ(ZeroPage),
    USreZX(ZeroPageX),
    USreA(Absolute),
    USreAX(AbsoluteX),
    USreAY(AbsoluteY),
    USreIX(IndexedIndirect),
    USreIY(IndirectIndexed),

    // STP - Causes the CPU to unrecoverably lock up, requiring a reset.
    UStp,
}

impl Instruction {
    /// Fetch and decode an instruction from memory at the address of the given
    /// program counter. Instructions are 1 to 3 bytes long: a 1-byte opcode
    /// optionally followed by a one or two byte argument. This method will
    /// increment the program counter by the appropriate amount after decoding
    /// the instruction. The opcode will be returned alongside the decoded
    /// instruction.
    pub(super) fn fetch(memory: &mut dyn Bus, pc: &mut Address) -> (Self, u8) {
        use Instruction::*;

        let start_pc = *pc;
        let opcode = memory.load(start_pc);
        *pc += 1u8;

        let instruction = match opcode {
            0x00 => Brk,
            0x01 => OraIX(IndexedIndirect(read_byte(memory, pc))),
            0x02 => UStp,
            0x03 => USloIX(IndexedIndirect(read_byte(memory, pc))),
            0x04 => UNopZ(ZeroPage(read_byte(memory, pc))),
            0x05 => OraZ(ZeroPage(read_byte(memory, pc))),
            0x06 => AslZ(ZeroPage(read_byte(memory, pc))),
            0x07 => USloZ(ZeroPage(read_byte(memory, pc))),
            0x08 => Php,
            0x09 => OraI(Immediate(read_byte(memory, pc))),
            0x0A => AslAcc(Accumulator),
            0x0B => panic!("Illegal opcode 0x0B (AAC) at {}", start_pc),
            0x0C => UNopA(Absolute(read_addr(memory, pc))),
            0x0D => OraA(Absolute(read_addr(memory, pc))),
            0x0E => AslA(Absolute(read_addr(memory, pc))),
            0x0F => USloA(Absolute(read_addr(memory, pc))),
            0x10 => Bpl(Relative(read_byte(memory, pc) as i8)),
            0x11 => OraIY(IndirectIndexed(read_byte(memory, pc))),
            0x12 => UStp,
            0x13 => USloIY(IndirectIndexed(read_byte(memory, pc))),
            0x14 => UNopZX(ZeroPageX(read_byte(memory, pc))),
            0x15 => OraZX(ZeroPageX(read_byte(memory, pc))),
            0x16 => AslZX(ZeroPageX(read_byte(memory, pc))),
            0x17 => USloZX(ZeroPageX(read_byte(memory, pc))),
            0x18 => Clc,
            0x19 => OraAY(AbsoluteY(read_addr(memory, pc))),
            0x1A => UNop,
            0x1B => USloAY(AbsoluteY(read_addr(memory, pc))),
            0x1C => UNopAX(AbsoluteX(read_addr(memory, pc))),
            0x1D => OraAX(AbsoluteX(read_addr(memory, pc))),
            0x1E => AslAX(AbsoluteX(read_addr(memory, pc))),
            0x1F => USloAX(AbsoluteX(read_addr(memory, pc))),
            0x20 => Jsr(Absolute(read_addr(memory, pc))),
            0x21 => AndIX(IndexedIndirect(read_byte(memory, pc))),
            0x22 => UStp,
            0x23 => URlaIX(IndexedIndirect(read_byte(memory, pc))),
            0x24 => BitZ(ZeroPage(read_byte(memory, pc))),
            0x25 => AndZ(ZeroPage(read_byte(memory, pc))),
            0x26 => RolZ(ZeroPage(read_byte(memory, pc))),
            0x27 => URlaZ(ZeroPage(read_byte(memory, pc))),
            0x28 => Plp,
            0x29 => AndI(Immediate(read_byte(memory, pc))),
            0x2A => RolAcc(Accumulator),
            0x2B => panic!("Illegal opcode 0x2B (AAC) at {}", start_pc),
            0x2C => BitA(Absolute(read_addr(memory, pc))),
            0x2D => AndA(Absolute(read_addr(memory, pc))),
            0x2E => RolA(Absolute(read_addr(memory, pc))),
            0x2F => URlaA(Absolute(read_addr(memory, pc))),
            0x30 => Bmi(Relative(read_byte(memory, pc) as i8)),
            0x31 => AndIY(IndirectIndexed(read_byte(memory, pc))),
            0x32 => UStp,
            0x33 => URlaIY(IndirectIndexed(read_byte(memory, pc))),
            0x34 => UNopZX(ZeroPageX(read_byte(memory, pc))),
            0x35 => AndZX(ZeroPageX(read_byte(memory, pc))),
            0x36 => RolZX(ZeroPageX(read_byte(memory, pc))),
            0x37 => URlaZX(ZeroPageX(read_byte(memory, pc))),
            0x38 => Sec,
            0x39 => AndAY(AbsoluteY(read_addr(memory, pc))),
            0x3A => UNop,
            0x3B => URlaAY(AbsoluteY(read_addr(memory, pc))),
            0x3C => UNopAX(AbsoluteX(read_addr(memory, pc))),
            0x3D => AndAX(AbsoluteX(read_addr(memory, pc))),
            0x3E => RolAX(AbsoluteX(read_addr(memory, pc))),
            0x3F => URlaAX(AbsoluteX(read_addr(memory, pc))),
            0x40 => Rti,
            0x41 => EorIX(IndexedIndirect(read_byte(memory, pc))),
            0x42 => UStp,
            0x43 => USreIX(IndexedIndirect(read_byte(memory, pc))),
            0x44 => UNopZ(ZeroPage(read_byte(memory, pc))),
            0x45 => EorZ(ZeroPage(read_byte(memory, pc))),
            0x46 => LsrZ(ZeroPage(read_byte(memory, pc))),
            0x47 => USreZ(ZeroPage(read_byte(memory, pc))),
            0x48 => Pha,
            0x49 => EorI(Immediate(read_byte(memory, pc))),
            0x4A => LsrAcc(Accumulator),
            0x4B => panic!("Illegal opcode 0x4B (ASR) at {}", start_pc),
            0x4C => JmpA(Absolute(read_addr(memory, pc))),
            0x4D => EorA(Absolute(read_addr(memory, pc))),
            0x4E => LsrA(Absolute(read_addr(memory, pc))),
            0x4F => USreA(Absolute(read_addr(memory, pc))),
            0x50 => Bvc(Relative(read_byte(memory, pc) as i8)),
            0x51 => EorIY(IndirectIndexed(read_byte(memory, pc))),
            0x52 => UStp,
            0x53 => USreIY(IndirectIndexed(read_byte(memory, pc))),
            0x54 => UNopZX(ZeroPageX(read_byte(memory, pc))),
            0x55 => EorZX(ZeroPageX(read_byte(memory, pc))),
            0x56 => LsrZX(ZeroPageX(read_byte(memory, pc))),
            0x57 => USreZX(ZeroPageX(read_byte(memory, pc))),
            0x58 => Cli,
            0x59 => EorAY(AbsoluteY(read_addr(memory, pc))),
            0x5A => UNop,
            0x5B => USreAY(AbsoluteY(read_addr(memory, pc))),
            0x5C => UNopAX(AbsoluteX(read_addr(memory, pc))),
            0x5D => EorAX(AbsoluteX(read_addr(memory, pc))),
            0x5E => LsrAX(AbsoluteX(read_addr(memory, pc))),
            0x5F => USreAX(AbsoluteX(read_addr(memory, pc))),
            0x60 => Rts,
            0x61 => AdcIX(IndexedIndirect(read_byte(memory, pc))),
            0x62 => UStp,
            0x63 => URraIX(IndexedIndirect(read_byte(memory, pc))),
            0x64 => UNopZ(ZeroPage(read_byte(memory, pc))),
            0x65 => AdcZ(ZeroPage(read_byte(memory, pc))),
            0x66 => RorZ(ZeroPage(read_byte(memory, pc))),
            0x67 => URraZ(ZeroPage(read_byte(memory, pc))),
            0x68 => Pla,
            0x69 => AdcI(Immediate(read_byte(memory, pc))),
            0x6A => RorAcc(Accumulator),
            0x6B => panic!("Illegal opcode 0x6B (ARR) at {}", start_pc),
            0x6C => JmpI(Indirect(read_addr(memory, pc))),
            0x6D => AdcA(Absolute(read_addr(memory, pc))),
            0x6E => RorA(Absolute(read_addr(memory, pc))),
            0x6F => URraA(Absolute(read_addr(memory, pc))),
            0x70 => Bvs(Relative(read_byte(memory, pc) as i8)),
            0x71 => AdcIY(IndirectIndexed(read_byte(memory, pc))),
            0x72 => UStp,
            0x73 => URraIY(IndirectIndexed(read_byte(memory, pc))),
            0x74 => UNopZX(ZeroPageX(read_byte(memory, pc))),
            0x75 => AdcZX(ZeroPageX(read_byte(memory, pc))),
            0x76 => RorZX(ZeroPageX(read_byte(memory, pc))),
            0x77 => URraZX(ZeroPageX(read_byte(memory, pc))),
            0x78 => Sei,
            0x79 => AdcAY(AbsoluteY(read_addr(memory, pc))),
            0x7A => UNop,
            0x7B => URraAY(AbsoluteY(read_addr(memory, pc))),
            0x7C => UNopAX(AbsoluteX(read_addr(memory, pc))),
            0x7D => AdcAX(AbsoluteX(read_addr(memory, pc))),
            0x7E => RorAX(AbsoluteX(read_addr(memory, pc))),
            0x7F => URraAX(AbsoluteX(read_addr(memory, pc))),
            0x80 => UNopI(Immediate(read_byte(memory, pc))),
            0x81 => StaIX(IndexedIndirect(read_byte(memory, pc))),
            0x82 => UNopI(Immediate(read_byte(memory, pc))),
            0x83 => USaxIX(IndexedIndirect(read_byte(memory, pc))),
            0x84 => StyZ(ZeroPage(read_byte(memory, pc))),
            0x85 => StaZ(ZeroPage(read_byte(memory, pc))),
            0x86 => StxZ(ZeroPage(read_byte(memory, pc))),
            0x87 => USaxZ(ZeroPage(read_byte(memory, pc))),
            0x88 => Dey,
            0x89 => UNopI(Immediate(read_byte(memory, pc))),
            0x8A => Txa,
            0x8B => panic!("Illegal opcode 0x8B (XAA) at {}", start_pc),
            0x8C => StyA(Absolute(read_addr(memory, pc))),
            0x8D => StaA(Absolute(read_addr(memory, pc))),
            0x8E => StxA(Absolute(read_addr(memory, pc))),
            0x8F => USaxA(Absolute(read_addr(memory, pc))),
            0x90 => Bcc(Relative(read_byte(memory, pc) as i8)),
            0x91 => StaIY(IndirectIndexed(read_byte(memory, pc))),
            0x92 => UStp,
            0x93 => panic!("Illegal opcode 0x93 (AXA) at {}", start_pc),
            0x94 => StyZX(ZeroPageX(read_byte(memory, pc))),
            0x95 => StaZX(ZeroPageX(read_byte(memory, pc))),
            0x96 => StxZY(ZeroPageY(read_byte(memory, pc))),
            0x97 => USaxZY(ZeroPageY(read_byte(memory, pc))),
            0x98 => Tya,
            0x99 => StaAY(AbsoluteY(read_addr(memory, pc))),
            0x9A => Txs,
            0x9B => panic!("Illegal opcode 0x9B (XAS) at {}", start_pc),
            0x9C => panic!("Illegal opcode 0x9C (SYA) at {}", start_pc),
            0x9D => StaAX(AbsoluteX(read_addr(memory, pc))),
            0x9E => panic!("Illegal opcode 0x9E (SXA) at {}", start_pc),
            0x9F => panic!("Illegal opcode 0x9F (AXA) at {}", start_pc),
            0xA0 => LdyI(Immediate(read_byte(memory, pc))),
            0xA1 => LdaIX(IndexedIndirect(read_byte(memory, pc))),
            0xA2 => LdxI(Immediate(read_byte(memory, pc))),
            0xA3 => ULaxIX(IndexedIndirect(read_byte(memory, pc))),
            0xA4 => LdyZ(ZeroPage(read_byte(memory, pc))),
            0xA5 => LdaZ(ZeroPage(read_byte(memory, pc))),
            0xA6 => LdxZ(ZeroPage(read_byte(memory, pc))),
            0xA7 => ULaxZ(ZeroPage(read_byte(memory, pc))),
            0xA8 => Tay,
            0xA9 => LdaI(Immediate(read_byte(memory, pc))),
            0xAA => Tax,
            0xAB => panic!("Illegal opcode 0xAB (ATX) at {}", start_pc),
            0xAC => LdyA(Absolute(read_addr(memory, pc))),
            0xAD => LdaA(Absolute(read_addr(memory, pc))),
            0xAE => LdxA(Absolute(read_addr(memory, pc))),
            0xAF => ULaxA(Absolute(read_addr(memory, pc))),
            0xB0 => Bcs(Relative(read_byte(memory, pc) as i8)),
            0xB1 => LdaIY(IndirectIndexed(read_byte(memory, pc))),
            0xB2 => UStp,
            0xB3 => ULaxIY(IndirectIndexed(read_byte(memory, pc))),
            0xB4 => LdyZX(ZeroPageX(read_byte(memory, pc))),
            0xB5 => LdaZX(ZeroPageX(read_byte(memory, pc))),
            0xB6 => LdxZY(ZeroPageY(read_byte(memory, pc))),
            0xB7 => ULaxZY(ZeroPageY(read_byte(memory, pc))),
            0xB8 => Clv,
            0xB9 => LdaAY(AbsoluteY(read_addr(memory, pc))),
            0xBA => Tsx,
            0xBB => panic!("Illegal opcode 0xBB (LAR) at {}", start_pc),
            0xBC => LdyAX(AbsoluteX(read_addr(memory, pc))),
            0xBD => LdaAX(AbsoluteX(read_addr(memory, pc))),
            0xBE => LdxAY(AbsoluteY(read_addr(memory, pc))),
            0xBF => ULaxAY(AbsoluteY(read_addr(memory, pc))),
            0xC0 => CpyI(Immediate(read_byte(memory, pc))),
            0xC1 => CmpIX(IndexedIndirect(read_byte(memory, pc))),
            0xC2 => UNopI(Immediate(read_byte(memory, pc))),
            0xC3 => UDcpIX(IndexedIndirect(read_byte(memory, pc))),
            0xC4 => CpyZ(ZeroPage(read_byte(memory, pc))),
            0xC5 => CmpZ(ZeroPage(read_byte(memory, pc))),
            0xC6 => DecZ(ZeroPage(read_byte(memory, pc))),
            0xC7 => UDcpZ(ZeroPage(read_byte(memory, pc))),
            0xC8 => Iny,
            0xC9 => CmpI(Immediate(read_byte(memory, pc))),
            0xCA => Dex,
            0xCB => panic!("Illegal opcode 0xCB (AXS) at {}", start_pc),
            0xCC => CpyA(Absolute(read_addr(memory, pc))),
            0xCD => CmpA(Absolute(read_addr(memory, pc))),
            0xCE => DecA(Absolute(read_addr(memory, pc))),
            0xCF => UDcpA(Absolute(read_addr(memory, pc))),
            0xD0 => Bne(Relative(read_byte(memory, pc) as i8)),
            0xD1 => CmpIY(IndirectIndexed(read_byte(memory, pc))),
            0xD2 => UStp,
            0xD3 => UDcpIY(IndirectIndexed(read_byte(memory, pc))),
            0xD4 => UNopZX(ZeroPageX(read_byte(memory, pc))),
            0xD5 => CmpZX(ZeroPageX(read_byte(memory, pc))),
            0xD6 => DecZX(ZeroPageX(read_byte(memory, pc))),
            0xD7 => UDcpZX(ZeroPageX(read_byte(memory, pc))),
            0xD8 => Cld,
            0xD9 => CmpAY(AbsoluteY(read_addr(memory, pc))),
            0xDA => UNop,
            0xDB => UDcpAY(AbsoluteY(read_addr(memory, pc))),
            0xDC => UNopAX(AbsoluteX(read_addr(memory, pc))),
            0xDD => CmpAX(AbsoluteX(read_addr(memory, pc))),
            0xDE => DecAX(AbsoluteX(read_addr(memory, pc))),
            0xDF => UDcpAX(AbsoluteX(read_addr(memory, pc))),
            0xE0 => CpxI(Immediate(read_byte(memory, pc))),
            0xE1 => SbcIX(IndexedIndirect(read_byte(memory, pc))),
            0xE2 => UNopI(Immediate(read_byte(memory, pc))),
            0xE3 => UIsbIX(IndexedIndirect(read_byte(memory, pc))),
            0xE4 => CpxZ(ZeroPage(read_byte(memory, pc))),
            0xE5 => SbcZ(ZeroPage(read_byte(memory, pc))),
            0xE6 => IncZ(ZeroPage(read_byte(memory, pc))),
            0xE7 => UIsbZ(ZeroPage(read_byte(memory, pc))),
            0xE8 => Inx,
            0xE9 => SbcI(Immediate(read_byte(memory, pc))),
            0xEA => Nop,
            0xEB => USbcI(Immediate(read_byte(memory, pc))),
            0xEC => CpxA(Absolute(read_addr(memory, pc))),
            0xED => SbcA(Absolute(read_addr(memory, pc))),
            0xEE => IncA(Absolute(read_addr(memory, pc))),
            0xEF => UIsbA(Absolute(read_addr(memory, pc))),
            0xF0 => Beq(Relative(read_byte(memory, pc) as i8)),
            0xF1 => SbcIY(IndirectIndexed(read_byte(memory, pc))),
            0xF2 => UStp,
            0xF3 => UIsbIY(IndirectIndexed(read_byte(memory, pc))),
            0xF4 => UNopZX(ZeroPageX(read_byte(memory, pc))),
            0xF5 => SbcZX(ZeroPageX(read_byte(memory, pc))),
            0xF6 => IncZX(ZeroPageX(read_byte(memory, pc))),
            0xF7 => UIsbZX(ZeroPageX(read_byte(memory, pc))),
            0xF8 => Sed,
            0xF9 => SbcAY(AbsoluteY(read_addr(memory, pc))),
            0xFA => UNop,
            0xFB => UIsbAY(AbsoluteY(read_addr(memory, pc))),
            0xFC => UNopAX(AbsoluteX(read_addr(memory, pc))),
            0xFD => SbcAX(AbsoluteX(read_addr(memory, pc))),
            0xFE => IncAX(AbsoluteX(read_addr(memory, pc))),
            0xFF => UIsbAX(AbsoluteX(read_addr(memory, pc))),
        };

        (instruction, opcode)
    }
}

/// Read a 16-bit little endian address from memory at the location of the
/// current program counter, incrementing the program counter by two.
fn read_addr(memory: &mut dyn Bus, pc: &mut Address) -> Address {
    let lsb = memory.load(*pc);
    let msb = memory.load(*pc + 1u8);
    *pc += 2u8;
    Address::from([lsb, msb])
}

/// Read a byte from memory at the location of the current program coutner,
/// incrementing the program counter by one.
fn read_byte(memory: &mut dyn Bus, pc: &mut Address) -> u8 {
    let byte = memory.load(*pc);
    *pc += 1u8;
    byte
}
