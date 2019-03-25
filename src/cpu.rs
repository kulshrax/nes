use bitflags::bitflags;

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
    pc: u16,

    // Status register.
    p: Flags,
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
        const INTERRUPT_DISABLE = 1 << 2;
        const DECIMAL = 1 << 3;
        const BREAK = 1 << 4;
        const UNUSED = 1 << 5;
        const OVERFLOW = 1 << 6;
        const NEGATIVE = 1 << 7; 
    }
}

impl Default for Flags {
    fn default() -> Self {
        Flags::INTERRUPT_DISABLE | Flags::UNUSED
    }
}


pub enum Instruction {

}

impl Instruction {
    fn decode(bytes: &[u8]) -> Self {
        unimplemented!()
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

    pub fn step(&mut self) {

    }
}

/// Methods corresponding to operations in the MOS 6502 instruction set.
/// 
/// See http://obelisk.me.uk/6502/reference.html for details about
/// each instruction.
impl Cpu {
    /// Add with carry.
    fn adc(&mut self) {
 
    }

    /// Logical AND.
    fn and(&mut self) {

    }

    /// Arithmetic left shift.
    fn asl(&mut self) {

    }

    /// Branch if carry clear.
    fn bcc(&mut self) {

    }

    /// Branch if carry set.
    fn bcs(&mut self) {

    }

    /// Branch if equal.
    fn beq(&mut self) {

    }

    /// Bit test.
    fn bit(&mut self) {

    }

    /// Branch if minus.
    fn bmi(&mut self) {

    }

    /// Branch if not equal.
    fn bne(&mut self) {
 
    }

    /// Branch if positive.
    fn bpl(&mut self) {

    }

    /// Force interrupt.
    fn brk(&mut self) {

    }

    /// Branch if overflow clear.
    fn bvc(&mut self) {

    }

    /// Branch if overflow set.
    fn bvs(&mut self) {

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
    fn cmp(&mut self) {

    }

    /// Compare X register.
    fn cpx(&mut self) {

    }

    /// Compare Y register.
    fn cpy(&mut self) {

    }

    /// Decrement memory.
    fn dec(&mut self) {

    }

    /// Decrement X register.
    fn dex(&mut self) {

    }

    /// Decrement Y register.
    fn dey(&mut self) {

    }

    /// Exclusive OR.
    fn eor(&mut self) {

    }

    /// Incrememnt memory.
    fn inc(&mut self) {

    }

    /// Increment X register.
    fn inx(&mut self) {

    }

    /// Increment Y register.
    fn iny(&mut self) {

    }

    /// Jump.
    fn jmp(&mut self) {
        
    }

    /// Jump to subroutine.
    fn jsr(&mut self) {

    }

    /// Load accumulator.
    fn lda(&mut self) {

    }

    /// Load X register.
    fn ldx(&mut self) {

    }

    /// Load Y register.
    fn ldy(&mut self) {
        
    }

    /// Logical shift right.
    fn lsr(&mut self) {

    }

    /// No operation.
    fn nop(&mut self) {}

    /// Logical inclusive OR.
    fn ora(&mut self) {

    }

    /// Push accumulator.
    fn pha(&mut self) {

    }

    /// Push processor status.
    fn php(&mut self) {

    }

    /// Pull accumulator.
    fn pla(&mut self) {

    }

    /// Pull processor status.
    fn plp(&mut self) {

    }

    /// Rotate left.
    fn rol(&mut self) {

    }

    /// Rotate right.
    fn ror(&mut self) {

    }

    /// Return from interrupt.
    fn rti(&mut self) {

    }

    /// Return from subroutine.
    fn rts(&mut self) {

    }

    /// Subtract with carry.
    fn sbc(&mut self) {

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
    fn sta(&mut self) {
    
    }

    /// Store X register.
    fn stx(&mut self) {

    }

    /// Store Y register.
    fn sty(&mut self) {

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