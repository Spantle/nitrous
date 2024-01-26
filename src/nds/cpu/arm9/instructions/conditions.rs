use crate::nds::{cpu::arm9::Arm9, logger};

#[inline(always)]
pub fn calculate_cond<const INST_SET: u16>(arm9: &mut Arm9) -> bool {
    let cond = (INST_SET >> 8 & 0b1111) as u8;
    let s = &arm9.cpsr;
    match cond {
        0b0000 => s.get_zero(),
        0b0001 => !s.get_zero(),
        0b0010 => s.get_carry(),
        0b0011 => !s.get_carry(),
        0b0100 => s.get_negative(),
        0b0101 => !s.get_negative(),
        0b0110 => s.get_overflow(),
        0b0111 => !s.get_overflow(),
        0b1000 => s.get_carry() && !s.get_zero(),
        0b1001 => !s.get_carry() || s.get_zero(),
        0b1010 => s.get_negative() == s.get_overflow(),
        0b1011 => s.get_negative() != s.get_overflow(),
        0b1100 => !s.get_zero() && s.get_negative() == s.get_overflow(),
        0b1101 => s.get_zero() || s.get_negative() != s.get_overflow(),
        0b1110 => true,
        0b1111 => {
            // TODO: UNPREDICTABLE?
            logger::warn("UNPREDICTABLE: condition 0b1111");
            true
        }
        _ => unreachable!(),
    }
}
