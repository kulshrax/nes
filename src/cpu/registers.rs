use std::fmt;

use bitflags::bitflags;

use crate::mem::Address;

#[derive(Default)]
pub struct Registers {
    // Accumulator.
    pub(super) a: u8,

    // Index registers.
    pub(super) x: u8,
    pub(super) y: u8,

    // Stack pointer.
    pub(super) s: u8,

    // Program counter.
    pub(super) pc: Address,

    // Status register.
    pub(super) p: Flags,
}

impl Registers {
    pub(super) fn new() -> Self {
        Self {
            s: 0xfd,
            ..Default::default()
        }
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "A: {:#x}, X: {:#x}, Y: {:#x}, S: {:#x}, PC: {}, P: {}",
            self.a, self.x, self.y, self.s, self.pc, self.p
        )
    }
}

bitflags! {
    /// Values corresponding to the bit flags stored
    /// in the status (P) register.
    pub(super) struct Flags: u8 {
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
        /// Ricoh 2A03 CPU used by the NES, so the flag does not
        /// change the behavior of arithmetic operations. However,
        /// the NES still supports getting and setting the flag.
        const DECIMAL = 1 << 3;

        /// Indicates that a BRK instruction has been executed
        /// and an interrupt has been generated to process it.
        /// This bit has no significance in the P register; it is
        /// only set to a meaningful value when P register is
        /// pushed to the stack prior to executing an interrupt
        /// handler. The handler may then read the flags from the
        /// stack to determine if this bit was set (which indicates
        /// the interrupt was triggered by software (via the BRK
        /// instruction) rather than by hardware).
        const BREAK = 1 << 4;

        /// This bit is not used for anything but is expected to
        /// always be set.
        const UNUSED = 1 << 5;

        /// Indicates that the result of an arithmetic operation on
        /// signed values would result in an invalid twos-complement
        /// result (such as the addition of two positive values resulting
        /// in a negative value due to an overflow into the sign bit).
        const OVERFLOW = 1 << 6;

        /// Indicates that the result of the last operation was negative.
        /// (Specifically, that the sign bit (i.e., bit 7) was set to 1.)
        const NEGATIVE = 1 << 7;

        /// Bits 4 and 5 are both unused in the P register; however,
        /// for correctness, these bits must always be present, since
        /// the original CPU considers them to be always switched on.
        const ALWAYS_ON = Self::BREAK.bits | Self::UNUSED.bits;
    }
}

impl Default for Flags {
    fn default() -> Self {
        Flags::ALWAYS_ON
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = if self.contains(Self::CARRY) { "C" } else { "-" };
        let z = if self.contains(Self::ZERO) { "Z" } else { "-" };
        let i = if self.contains(Self::INTERRUPT_DISABLE) {
            "I"
        } else {
            "-"
        };
        let d = if self.contains(Self::DECIMAL) {
            "D"
        } else {
            "-"
        };
        let b = if self.contains(Self::BREAK) { "B" } else { "-" };
        let u = if self.contains(Self::UNUSED) {
            "U"
        } else {
            "-"
        };
        let v = if self.contains(Self::OVERFLOW) {
            "V"
        } else {
            "-"
        };
        let n = if self.contains(Self::NEGATIVE) {
            "N"
        } else {
            "-"
        };
        write!(f, "[{}{}{}{}{}{}{}{}]", n, v, u, b, d, i, z, c)
    }
}
