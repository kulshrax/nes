use crate::mem::{Address, Bus};

use super::registers::Registers;

pub(super) trait AddressingMode: Clone {
    /// Return the address of the target location specified by this addressing
    /// mode. This will panic for modes where this is not possible. (For
    /// example, attempting to get the address of a register or immediate value)
    fn address(&self, memory: &mut dyn Bus, registers: &Registers) -> Address;

    /// Load a value from the location specified by this addressing mode. This
    /// may be loaded from a location in memory, from a register, from an
    /// immediate value, or from a combination of these (in the case of indexed
    /// and indirect addressing modes).
    fn load(&self, memory: &mut dyn Bus, registers: &Registers) -> u8 {
        let addr = self.address(memory, registers);
        memory.load(addr)
    }

    /// Store a value to the location specified by this addressing mode. This
    /// may be loaded from a location in memory, from a register, from an
    /// immediate value, or from a combination of these (in the case of indexed
    /// and indirect addressing modes).
    fn store(&self, memory: &mut dyn Bus, registers: &mut Registers, value: u8) {
        let addr = self.address(memory, registers);
        memory.store(addr, value);
    }
}

/// Accumulator addresssing means that the instruction should load or store a
/// value directly to/from the A register.
#[derive(Copy, Clone, Debug)]
pub(super) struct Accumulator;

impl AddressingMode for Accumulator {
    fn address(&self, _memory: &mut dyn Bus, _registers: &Registers) -> Address {
        panic!("Cannot take address of accumulator");
    }

    fn load(&self, _memory: &mut dyn Bus, registers: &Registers) -> u8 {
        registers.a
    }

    fn store(&self, _memory: &mut dyn Bus, registers: &mut Registers, value: u8) {
        registers.a = value;
    }
}

/// Immediate addressing denotes that an immediate value was given as part of
/// the instruction. As such, a load should just directly use the immediate
/// value. Stores do not make sense in this mode.
#[derive(Copy, Clone, Debug)]
pub(super) struct Immediate(pub(super) u8);

impl AddressingMode for Immediate {
    fn address(&self, _memory: &mut dyn Bus, _registers: &Registers) -> Address {
        panic!("Cannot take address of immediate value");
    }

    fn load(&self, _memory: &mut dyn Bus, _registers: &Registers) -> u8 {
        self.0
    }
}

/// Zero page addressing uses an 8-bit address to refer to a location in the
/// first 256 bytes of memory, which is referred to as the "zero page" since the
/// most significant byte of addresses on this page is always zero.
///
/// Since the first byte of the address is known to be zero, it can be omitted
/// from the argument, making the instruction shorter and faster to execute
/// (since fewer memory fetches are required during execution).
#[derive(Copy, Clone, Debug)]
pub(super) struct ZeroPage(pub(super) u8);

impl AddressingMode for ZeroPage {
    fn address(&self, _memory: &mut dyn Bus, _registers: &Registers) -> Address {
        Address::from(self.0)
    }
}

/// X-indexed zero page addressing takes an 8-bit immediate value, adds it to
/// the current value of the X register (wrapping around if the sum exceeds
/// 0xFF), and interprets the result as an 8-bit zero page address, which is
/// then used to load/store the given value.
#[derive(Copy, Clone, Debug)]
pub(super) struct ZeroPageX(pub(super) u8);

impl AddressingMode for ZeroPageX {
    fn address(&self, _memory: &mut dyn Bus, registers: &Registers) -> Address {
        Address::from(registers.x.wrapping_add(self.0))
    }
}

/// Y-indexed zero page addressing takes an 8-bit immediate value, adds it to
/// the current value of the Y register (wrapping around if the sum exceeds
/// 0xFF), and interprets the result as an 8-bit zero page address, which is
/// then used to load/store the given value.
#[derive(Copy, Clone, Debug)]
pub(super) struct ZeroPageY(pub(super) u8);

impl AddressingMode for ZeroPageY {
    fn address(&self, _memory: &mut dyn Bus, registers: &Registers) -> Address {
        Address::from(registers.y.wrapping_add(self.0))
    }
}

/// Relative addressing is used to compute addresses relative to the current
/// program counter. The instruction takes an 8-bit operand which is treated as
/// a signed relative offset from the current program counter. Note that the
/// program counter is itself incremented during the execution of this
/// instruction, so the final target address will be (program counter + operand
/// + 2).
#[derive(Copy, Clone, Debug)]
pub(super) struct Relative(pub(super) i8);

impl AddressingMode for Relative {
    fn address(&self, _memory: &mut dyn Bus, registers: &Registers) -> Address {
        registers.pc + self.0
    }
}

/// Absolute addressing means that the instruction 's operand consists
/// of the exact 16-bit address of the target value.
#[derive(Copy, Clone, Debug)]
pub(super) struct Absolute(pub(super) Address);

impl AddressingMode for Absolute {
    fn address(&self, _memory: &mut dyn Bus, _registers: &Registers) -> Address {
        self.0
    }
}

/// X-indexed absolute addressing takes a 16-bit address as an operand and adds
/// the 8-bit value of the X register (which is treated as an offset) to compute
/// the target memory location.
#[derive(Copy, Clone, Debug)]
pub(super) struct AbsoluteX(pub(super) Address);

impl AddressingMode for AbsoluteX {
    fn address(&self, _memory: &mut dyn Bus, registers: &Registers) -> Address {
        self.0 + registers.x
    }
}

/// Y-indexed absolute addressing takes a 16-bit address as an operand and adds
/// the 8-bit value of the Y register (which is treated as an offset) to compute
/// the target memory location.
#[derive(Copy, Clone, Debug)]
pub(super) struct AbsoluteY(pub(super) Address);

impl AddressingMode for AbsoluteY {
    fn address(&self, _memory: &mut dyn Bus, registers: &Registers) -> Address {
        self.0 + registers.y
    }
}

/// Indirect addressing is only supported by the JMP instruction. In this
/// addressing mode, the operand is the 16-bit address of the least significant
/// byte of a little-endian 16-bit value which is then used as the target
/// location for the operation.
///
/// Note that if the specified address has a low byte of 0xFF, the second read
/// will wrap to the start of the page. (For example, if the specified address
/// is 0xABFF, the target address bytes will be read from 0xABFF and 0xAB00.)
#[derive(Copy, Clone, Debug)]
pub(super) struct Indirect(pub(super) Address);

impl AddressingMode for Indirect {
    fn address(&self, memory: &mut dyn Bus, _registers: &Registers) -> Address {
        let low = memory.load(self.0);

        // Only increment the low byte of the address, thereby wrapping the
        // read if we're at a page boundary.
        let mut addr_bytes = self.0.to_le_bytes();
        addr_bytes[0] = addr_bytes[0].wrapping_add(1);
        let high = memory.load(addr_bytes.into());

        Address::from([low, high])
    }
}

/// Indexed indirect addressing assumes that the program has a table of
/// addresses stored on the zero page. The 8-bit operand is treated as the
/// starting address of the lookup table, and the value of the
/// X register is treated as the offset of the least significant byte of the
/// target address within the table. Once found, the value in the table is
/// intepreted as a 16-bit little endian memory address which is then used as
/// the target address for the operation.
#[derive(Copy, Clone, Debug)]
pub(super) struct IndexedIndirect(pub(super) u8);

impl AddressingMode for IndexedIndirect {
    fn address(&self, memory: &mut dyn Bus, registers: &Registers) -> Address {
        let low_addr = Address::from(self.0.wrapping_add(registers.x));
        let low = memory.load(low_addr);

        let high_addr = Address::from(self.0.wrapping_add(registers.x).wrapping_add(1));
        let high = memory.load(high_addr);

        Address::from([low, high])
    }
}

/// Indirect indexed addressing takes an 8-bit operand which is treated as the
/// location of the least significant byte of a 16-bit little endian address
/// stored on the zero page. The value of the Y register is added to this
/// address to determine the target location.
#[derive(Copy, Clone, Debug)]
pub(super) struct IndirectIndexed(pub(super) u8);

impl AddressingMode for IndirectIndexed {
    fn address(&self, memory: &mut dyn Bus, registers: &Registers) -> Address {
        let low_addr = Address::from(self.0);
        let low = memory.load(low_addr);

        let high_addr = Address::from(self.0.wrapping_add(1));
        let high = memory.load(high_addr);

        let addr = Address::from([low, high]);
        addr + registers.y
    }
}
