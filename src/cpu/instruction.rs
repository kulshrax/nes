use crate::mem::{Address, Memory};

use super::addressing::*;

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
    AslAZ(AbsoluteX),

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
    DecZPX(ZeroPageX),
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
    IncZP(ZeroPageX),
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
    /// of the given program counter. Instructions are generally 1 to
    /// 3 bytes long: a 1-byte opcode followed by a one or two byte
    /// argument. This method will increment the program counter by
    /// the appropriate amount after decoding the instruction.
    pub(super) fn fetch(memory: &Memory, pc: &mut Address) -> Self {
        let opcode = memory.load(*pc);
        match opcode {
            illegal => panic!("Illegal opcode: {:#X}", illegal),
        }
    }
}
