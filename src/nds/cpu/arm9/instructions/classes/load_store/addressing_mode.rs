use crate::nds::{
    cpu::arm9::{instructions::models::Instruction, Arm9},
    logger,
};

pub fn parse_immediate(_arm9: &Arm9, inst: &Instruction) -> u32 {
    inst.get_word(0, 12)
}

pub fn parse_register(arm9: &Arm9, inst: &Instruction) -> u32 {
    let rm = inst.get_byte(0, 3);
    let rm = arm9.eru(rm);

    let shift = inst.get_byte(5, 6);
    let shift_imm = inst.get_word(7, 11);
    match shift {
        0b00 => {
            // LSL
            rm << shift_imm
        }
        0b01 => {
            // LSR
            if shift_imm == 0 {
                // LSR #32
                0
            } else {
                rm >> shift_imm
            }
        }
        0b10 => {
            // ASR
            if shift_imm == 0 {
                // ASR #32
                if rm & (1 << 31) != 0 {
                    // technically bit 31 is supposed to equal 1 but we cheat this
                    0xFFFFFFFF
                } else {
                    0
                }
            } else {
                ((rm as i32) >> shift_imm) as u32
            }
        }
        0b11 => {
            if shift_imm == 0 {
                // RRX
                logger::debug(logger::LogSource::Arm9, "the funny actually happened"); // TODO: remove
                (arm9.cpsr.get_carry() as u32) << 31 | rm >> 1
            } else {
                // ROR
                rm.rotate_right(shift_imm)
            }
        }
        _ => unreachable!(),
    }
}
