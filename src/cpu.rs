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
//! Many thanks to Andrew Jacobs, whose introductory guide to the MOS
//! 6502 (http://www.obelisk.me.uk/6502/) was an invaluable resource
//! for this implementation.

use crate::mem::{Address, Memory};

use addressing::{Absolute, AddressingMode, Relative};
use instruction::Instruction;
use registers::{Flags, Registers};

mod addressing;
mod instruction;
mod registers;

/// The 6502 has a 256-byte stack address space that is fixed
/// at memory page 1 (addresses 0x100 to 0x1FF). The stack
/// starts at 0x1FF and grows downward as values are pushed.
/// The next free location on the stack is pointed at by the
/// S register, which contains the low byte of the next
/// available stack address. There is no overflow checking
/// for the call stack.
const STACK_START: u16 = 0x0100;

/// When the CPU receives an interrupt, it loads an address
/// from a fixed memory location (known as an interrupt vector)
/// and sets the program counter to that address. This allows the
/// program to specify interrupt handlers by writing the address
/// of the start of the handler code to the appropriate interrupt
/// vector location. There are several interrupt vectors, each
/// corresponding to a different kind of interrupt. All of them
/// stored in the highest bytes of the 16-bit address space.
const NMI_VECTOR: [u16; 2] = [0xFFFA, 0xFFFB];
const RESET_VECTOR: [u16; 2] = [0xFFFC, 0xFFFD];
const IRQ_VECTOR: [u16; 2] = [0xFFFE, 0xFFFF];

/// Emulated MOS 6502 CPU.
pub struct Cpu {
    registers: Registers,
    irq_pending: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            irq_pending: false,
        }
    }

    /// Manually set the address stored in the CPU's initialization
    /// vector. The CPU will jump to this address on startup or
    /// reset to begin execution.
    pub fn set_init(&mut self, memory: &mut Memory, addr: Address) {
        let [low, high] = <[u8; 2]>::from(addr);
        memory.store(Address::from(RESET_VECTOR[0]), low);
        memory.store(Address::from(RESET_VECTOR[1]), high);
    }

    pub fn registers(&self) -> &Registers {
        &self.registers
    }

    /// Fetch and execute a single instruction. Returns the
    /// post-operation value of the program counter.
    pub fn step(&mut self, memory: &mut Memory) -> Address {
        // If there is a pending interrupt and interrupts
        // are not disabled, service it immediately.
        if self.irq_pending && !self.registers.p.contains(Flags::INTERRUPT_DISABLE) {
            log::trace!("Handling pending IRQ");
            self.irq_pending = false;
            self.irq(memory);
        }

        let op = Instruction::fetch(memory, &mut self.registers.pc);
        self.exec(memory, op);
        log::trace!("Registers: {}", &self.registers);
        self.registers.pc
    }

    /// Reset the CPU by disabling interrupts and jumping to the
    /// location specified by the initialization vector.
    pub fn reset(&mut self, memory: &Memory) {
        self.registers.p.insert(Flags::INTERRUPT_DISABLE);
        let low = memory.load(Address::from(RESET_VECTOR[0]));
        let high = memory.load(Address::from(RESET_VECTOR[1]));
        self.registers.pc = Address::from([low, high]);
    }

    /// Interrupt request.
    pub fn irq(&mut self, memory: &mut Memory) {
        log::trace!("Received IRQ");
        if self.registers.p.contains(Flags::INTERRUPT_DISABLE) {
            log::trace!("Interrupts are disabled; IRQ will be handled when they are enabled");
            self.irq_pending = true;
        } else {
            log::trace!("Handling IRQ");
            self.interrupt(memory, &IRQ_VECTOR, false);
        }
    }

    /// Non-maskable interrupt.
    pub fn nmi(&mut self, memory: &mut Memory) {
        self.interrupt(memory, &NMI_VECTOR, false);
    }

    /// Execute the given instruction.
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
            Brk => self.brk(memory),
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
            Jsr(am) => self.jsr(am, memory),
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

    /// Cause the CPU to stop the normal execution flow and begin
    /// executing an interrupt handler. The interrupt handler that
    /// is executed is determined from by the address stored at the
    /// location specified by the given interrupt vector. The brk
    /// parameter allows specifying whether this was a software or
    /// hardware interrupt.
    fn interrupt(&mut self, memory: &mut Memory, vector: &[u16; 2], brk: bool) {
        // Push program counter to stack.
        let [low, high] = <[u8; 2]>::from(self.registers.pc + 1u8);
        self.push_stack(memory, high);
        self.push_stack(memory, low);

        // Push flags to stack. Set the BRK flag in the pushed byte
        // appropriately. In particular, if this interrupt was
        // triggered by a BRK instruction, then the BRK bit should
        // be 1 in the pushed value; if the interrupt was triggered
        // by a hardware interrupt, it should be set to 0.
        let mut flags = self.registers.p;
        flags.set(Flags::BREAK, brk);
        self.push_stack(memory, flags.bits());

        // Disable interrupts so that the interrupt handler
        // is not itself interrupted.
        self.registers.p.insert(Flags::INTERRUPT_DISABLE);

        // Load the interrupt handler address from a fixed
        // location in memory, then jump to that address.
        let low = memory.load(Address::from(vector[0]));
        let high = memory.load(Address::from(vector[1]));
        self.registers.pc = Address::from([low, high]);
    }

    /// Get the current address of the next available memory
    /// location on the call stack.
    fn stack(&self) -> Address {
        Address::from(STACK_START) + self.registers.s
    }

    /// Push a value onto the call stack. Note that if the
    /// stack pointer overflows, this will wrap around and
    /// overwrite data at the start of the stack.
    fn push_stack(&mut self, memory: &mut Memory, value: u8) {
        memory.store(self.stack(), value);
        self.registers.s = self.registers.s.wrapping_sub(1);
    }

    /// Pull ("pop" in more modern terms) a value from
    /// the call stack. If the stack pointer underflows,
    /// it will wrap around to the top of memory page 1,
    /// potentially reading garbage.
    fn pull_stack(&mut self, memory: &mut Memory) -> u8 {
        self.registers.s = self.registers.s.wrapping_add(1);
        memory.load(self.stack())
    }

    /// Check if the given value is zero or negative
    /// and set the appropriate flags in the status
    /// register. Note that since the value is unsigned,
    /// the negative check is just checking if the sign
    /// bit is set if the value were interpreted as a
    /// two's complement signed integer.
    fn check_zero_or_negative(&mut self, value: u8) {
        self.registers.p.set(Flags::ZERO, value == 0);
        self.registers.p.set(Flags::NEGATIVE, value > 127);
    }
}

/// Methods corresponding to operations in the MOS 6502 instruction set.
///
/// See http://obelisk.me.uk/6502/reference.html for details about
/// each instruction.
impl Cpu {
    /// Add with carry.
    fn adc(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let value = am.load(memory, &self.registers);
        let carry_in = self.registers.p.contains(Flags::CARRY);

        // Cast the values to u16's before adding so we can
        // more easily check for overflow.
        let res = self.registers.a as u16 + value as u16 + carry_in as u16;
        let carry_out = res > 255;
        let res = res as u8;

        let overflow = twos_complement_overflow(self.registers.a, value, res);
        self.registers.p.set(Flags::OVERFLOW, overflow);

        self.registers.a = res;
        self.registers.p.set(Flags::CARRY, carry_out);
        self.check_zero_or_negative(res);
    }

    /// Logical AND.
    fn and(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let value = am.load(memory, &self.registers);
        self.registers.a &= value;
        self.check_zero_or_negative(self.registers.a);
    }

    /// Arithmetic left shift.
    fn asl(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let value = am.load(memory, &self.registers);
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
        let value = am.load(memory, &self.registers);
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
    fn brk(&mut self, memory: &mut Memory) {
        self.interrupt(memory, &IRQ_VECTOR, true);
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
    fn cmp(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let value = am.load(memory, &self.registers);
        log::info!("A: {}, M: {}", self.registers.a, value);
        let (res, overflowed) = self.registers.a.overflowing_sub(value);
        self.registers.p.set(Flags::CARRY, !overflowed);
        self.check_zero_or_negative(res);
    }

    /// Compare X register.
    fn cpx(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let value = am.load(memory, &self.registers);
        let (res, overflowed) = self.registers.x.overflowing_sub(value);
        self.registers.p.set(Flags::CARRY, !overflowed);
        self.check_zero_or_negative(res);
    }

    /// Compare Y register.
    fn cpy(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let value = am.load(memory, &self.registers);
        let (res, overflowed) = self.registers.y.overflowing_sub(value);
        self.registers.p.set(Flags::CARRY, !overflowed);
        self.check_zero_or_negative(res);
    }

    /// Decrement memory.
    fn dec(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let mut value = am.load(memory, &self.registers);
        value = value.wrapping_sub(1);
        am.store(memory, &mut self.registers, value);
        self.check_zero_or_negative(value);
    }

    /// Decrement X register.
    fn dex(&mut self) {
        self.registers.x = self.registers.x.wrapping_sub(1);
        self.check_zero_or_negative(self.registers.x);
    }

    /// Decrement Y register.
    fn dey(&mut self) {
        self.registers.y = self.registers.y.wrapping_sub(1);
        self.check_zero_or_negative(self.registers.y);
    }

    /// Exclusive OR.
    fn eor(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let value = am.load(memory, &self.registers);
        self.registers.a ^= value;
        self.check_zero_or_negative(self.registers.a);
    }

    /// Incrememnt memory.
    fn inc(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let mut value = am.load(memory, &self.registers);
        value = value.wrapping_add(1);
        am.store(memory, &mut self.registers, value);
        self.check_zero_or_negative(value);
    }

    /// Increment X register.
    fn inx(&mut self) {
        self.registers.x = self.registers.x.wrapping_add(1);
        self.check_zero_or_negative(self.registers.x);
    }

    /// Increment Y register.
    fn iny(&mut self) {
        self.registers.y = self.registers.y.wrapping_add(1);
        self.check_zero_or_negative(self.registers.y);
    }

    /// Jump.
    fn jmp(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        self.registers.pc = am.address(memory, &mut self.registers);
    }

    /// Jump to subroutine.
    fn jsr(&mut self, am: Absolute, memory: &mut Memory) {
        let ret = self.registers.pc - 1u8;
        let [low, high] = <[u8; 2]>::from(ret);
        self.push_stack(memory, high);
        self.push_stack(memory, low);
        self.registers.pc = am.address(memory, &mut self.registers);
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
        let value = am.load(memory, &self.registers);
        let res = value >> 1;
        am.store(memory, &mut self.registers, res);

        self.registers.p.set(Flags::CARRY, value & 1 > 0);
        self.check_zero_or_negative(res);
    }

    /// No operation.
    fn nop(&mut self) {}

    /// Logical inclusive OR.
    fn ora(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let value = am.load(memory, &self.registers);
        self.registers.a |= value;
        self.check_zero_or_negative(self.registers.a);
    }

    /// Push accumulator.
    fn pha(&mut self, memory: &mut Memory) {
        self.push_stack(memory, self.registers.a);
    }

    /// Push processor status.
    fn php(&mut self, memory: &mut Memory) {
        let flags = self.registers.p | Flags::ALWAYS_ON;
        self.push_stack(memory, flags.bits());
    }

    /// Pull accumulator.
    fn pla(&mut self, memory: &mut Memory) {
        self.registers.a = self.pull_stack(memory);
        self.check_zero_or_negative(self.registers.a);
    }

    /// Pull processor status.
    fn plp(&mut self, memory: &mut Memory) {
        let bits = self.pull_stack(memory);
        self.registers.p = Flags::from_bits_truncate(bits) | Flags::ALWAYS_ON;
    }

    /// Rotate left.
    fn rol(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let mut value = am.load(memory, &self.registers);

        // Current value of the carry flag, which will be
        // rotated into bit 0.
        let old_carry = self.registers.p.contains(Flags::CARRY) as u8;

        // Bit 7, which is about to be rotated out into the carry flag.
        let new_carry = value & (1 << 7) > 0;

        value = (value << 1) | old_carry;
        am.store(memory, &mut self.registers, value);

        self.registers.p.set(Flags::CARRY, new_carry);
        self.check_zero_or_negative(value);
    }

    /// Rotate right.
    fn ror(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let mut value = am.load(memory, &self.registers);

        // Current value of the carry flag, which will be
        // rotated into bit 7.
        let old_carry = self.registers.p.contains(Flags::CARRY) as u8;

        // Bit 0, which is about to be rotated out into the carry flag.
        let new_carry = value & 1 > 0;

        value = (value >> 1) | (old_carry << 7);
        am.store(memory, &mut self.registers, value);

        self.registers.p.set(Flags::CARRY, new_carry);
        self.check_zero_or_negative(value);
    }

    /// Return from interrupt.
    fn rti(&mut self, memory: &mut Memory) {
        let bits = self.pull_stack(memory);
        self.registers.p = Flags::from_bits_truncate(bits) | Flags::ALWAYS_ON;

        let low = self.pull_stack(memory);
        let high = self.pull_stack(memory);
        self.registers.pc = Address::from([low, high]);
    }

    /// Return from subroutine.
    fn rts(&mut self, memory: &mut Memory) {
        let low = self.pull_stack(memory);
        let high = self.pull_stack(memory);
        self.registers.pc = Address::from([low, high]) + 1u8;
    }

    /// Subtract with carry.
    fn sbc(&mut self, am: impl AddressingMode, memory: &mut Memory) {
        let value = am.load(memory, &self.registers);
        let carry_in = !self.registers.p.contains(Flags::CARRY);

        // Cast the values to i16's before substracing so we can
        // more easily check for underflow.
        let res = self.registers.a as i16 - value as i16 - carry_in as i16;
        let carry_out = res >= 0;
        let res = res as u8;

        let overflow = twos_complement_overflow(self.registers.a, !value, res);
        self.registers.p.set(Flags::OVERFLOW, overflow);

        self.registers.a = res;
        self.registers.p.set(Flags::CARRY, carry_out);
        self.check_zero_or_negative(res);
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
    }

    /// Transfer Y to accumulator.
    fn tya(&mut self) {
        let y = self.registers.y;
        self.registers.a = y;
        self.check_zero_or_negative(y);
    }
}

/// Check for two's complement overflow during addition or subtraction
/// by checking whether the sign bit of each operand matches that of the
/// result. See the folowing webpage for a more detailed explanation:
/// http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
#[inline]
fn twos_complement_overflow(n: u8, m: u8, res: u8) -> bool {
    (n ^ res) & (m ^ res) & (1 << 7) > 0
}
