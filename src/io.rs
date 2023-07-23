use std::fmt;

use crate::mem::Address;

#[derive(Debug)]
pub enum IoRegister {
    Sq1Vol,
    Sq1Sweep,
    Sq1Lo,
    Sq1Hi,
    Sq2Vol,
    Sq2Sweep,
    Sq2Lo,
    Sq2Hi,
    TriLinear,
    TriLo,
    TriHi,
    NoiseVol,
    NoiseLo,
    NoiseHi,
    DmcFreq,
    DmcRaw,
    DmcStart,
    DmcLen,
    OamDma,
    SndChn,
    Joy1,
    Joy2,
}

impl fmt::Display for IoRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use IoRegister::*;
        match self {
            Sq1Vol => write!(f, "SQ1_VOL"),
            Sq1Sweep => write!(f, "SQ1_SWEEP"),
            Sq1Lo => write!(f, "SQ1_LO"),
            Sq1Hi => write!(f, "SQ1_HI"),
            Sq2Vol => write!(f, "SQ1_VOL"),
            Sq2Sweep => write!(f, "SQ2_SWEEP"),
            Sq2Lo => write!(f, "SQ2_LO"),
            Sq2Hi => write!(f, "SQ2_HI"),
            TriLinear => write!(f, "TRI_LINEAR"),
            TriLo => write!(f, "TRI_LO"),
            TriHi => write!(f, "TRI_HI"),
            NoiseVol => write!(f, "NOISE_VOL"),
            NoiseLo => write!(f, "NOISE_LO"),
            NoiseHi => write!(f, "NOISE_HI"),
            DmcFreq => write!(f, "DMC_FREQ"),
            DmcRaw => write!(f, "DMC_RAW"),
            DmcStart => write!(f, "DMC_START"),
            DmcLen => write!(f, "DMC_LEN"),
            OamDma => write!(f, "OAMDMA"),
            SndChn => write!(f, "SND_CHN"),
            Joy1 => write!(f, "JOY1"),
            Joy2 => write!(f, "JOY2"),
        }
    }
}

impl From<Address> for IoRegister {
    fn from(addr: Address) -> Self {
        use IoRegister::*;
        match addr.as_usize() {
            0x4000 => Sq1Vol,
            0x4001 => Sq1Sweep,
            0x4002 => Sq1Lo,
            0x4003 => Sq1Hi,
            0x4004 => Sq2Vol,
            0x4005 => Sq2Sweep,
            0x4006 => Sq2Lo,
            0x4007 => Sq2Hi,
            0x4008 => TriLinear,
            0x400A => TriLo,
            0x400B => TriHi,
            0x400C => NoiseVol,
            0x400E => NoiseLo,
            0x400F => NoiseHi,
            0x4010 => DmcFreq,
            0x4011 => DmcRaw,
            0x4012 => DmcStart,
            0x4013 => DmcLen,
            0x4014 => OamDma,
            0x4015 => SndChn,
            0x4016 => Joy1,
            0x4017 => Joy2,
            _ => panic!("Invalid IO register address: {}", &addr),
        }
    }
}
