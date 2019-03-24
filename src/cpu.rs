#[derive(Default)]
pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default)]
pub struct Registers {
    a: u8,
    x: u8,
    y: u8,
    s: u8,
    p: Flags,
    pc: u16,
}

impl Registers {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default)]
pub struct Flags(u8);

impl Flags {
    const CARRY: u8 = 1;
    const ZERO: u8 = 1 << 1;
    const IRQ: u8 = 1 << 2;
    const DECIMAL: u8 = 1 << 3;
    const BREAK: u8 = 1 << 4;
    const OVERFLOW: u8 = 1 << 6;
    const NEGATIVE: u8 = 1 << 7;

    pub fn new() -> Self {
        Default::default()
    }
}

enum Instruction {
    Adc,
    And,
    Asl,
    Bcc,
    Bcs,
    Beq,
    Bit,
    Bmi,
    Bne,
    Bpl,
    Brk,
    Bvc,
    Bvs,
    Clc,
    Cld,
    Cli,
    Clv,
    Cmp,
    Cpx,
    Cpy,
    Dec,
    Dex,
    Dey,
    Eor,
    Inc,
    Inx,
    Iny,
    Jmp,
    Jsr,
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Nop,
    Ora,
    Pha,
    Php,
    Pla,
    Plp,
    Rol,
    Ror,
    Rti,
    Rts,
    Sbc,
    Sec,
    Sed,
    Sei,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,
}