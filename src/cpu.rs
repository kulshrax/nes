use bitflags::bitflags;

pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
        }
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


bitflags! {
    struct Flags: u8 {
        const CARRY = 1;
        const ZERO = 1 << 1;
        const IRQ = 1 << 2;
        const DECIMAL = 1 << 3;
        const BREAK = 1 << 4;
        const UNUSED = 1 << 5;
        const OVERFLOW = 1 << 6;
        const NEGATIVE = 1 << 7; 
    }
}

impl Default for Flags {
    fn default() -> Self {
        Flags::IRQ | Flags::UNUSED
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