use crate::nds::cpu::arm9::models::Context;

// AKA the addressing mode
pub struct ShifterOperand {
    pub carry_out: bool,
    pub second_source_operand: u32,
}

pub fn parse_immediate(ctx: &Context) -> ShifterOperand {
    let mut carry_out = ctx.arm9.cpsr.get_carry();

    let immed_8 = ctx.inst.get_word(0, 7);
    let rotate_imm = ctx.inst.get_word(8, 11); // NOTE: rotate_imm must be even to be "legitimate"

    let rotated = immed_8.rotate_right(rotate_imm * 2);
    if rotate_imm != 0 {
        carry_out = rotated & (1 << 31) != 0;
    }

    ShifterOperand {
        carry_out,
        second_source_operand: rotated,
    }
}

pub fn parse_register(ctx: &Context) -> ShifterOperand {
    let (arm9, inst) = (&ctx.arm9, &ctx.inst);

    let mut carry_out = arm9.cpsr.get_carry();

    let operand = inst.get_byte(4, 6);
    let second_source_operand = match operand {
        0b000 => {
            // LSL immediate
            let rm = arm9.er(inst.get_byte(0, 3));
            let shift_imm = inst.get_word(7, 11);
            if shift_imm == 0 {
                rm
            } else {
                carry_out = (rm >> (32 - shift_imm)) & 1 != 0;
                rm << shift_imm
            }
        }
        0b001 => {
            // LSL register
            let rm = arm9.er(inst.get_byte(0, 3));
            let rs = arm9.er(inst.get_byte(8, 11));
            let least_significant_byte = rs & 0b11111111;
            if least_significant_byte == 0 {
                rm
            } else if least_significant_byte < 32 {
                carry_out = (rm >> (32 - least_significant_byte)) & 1 != 0;
                rm << least_significant_byte
            } else if least_significant_byte == 32 {
                carry_out = rm & 1 != 0;
                0
            } else {
                carry_out = false;
                0
            }
        }
        0b010 => {
            // LSR immediate
            let rm = arm9.er(inst.get_byte(0, 3));
            let shift_imm = inst.get_word(7, 11);
            if shift_imm == 0 {
                carry_out = rm & (1 << 31) != 0;
                0
            } else {
                carry_out = (rm >> (shift_imm - 1)) & 1 != 0;
                rm >> shift_imm
            }
        }
        0b011 => {
            // LSR register
            let rm = arm9.er(inst.get_byte(0, 3));
            let rs = arm9.er(inst.get_byte(8, 11));
            let least_significant_byte = rs & 0b11111111;
            if least_significant_byte == 0 {
                rm
            } else if least_significant_byte < 32 {
                carry_out = rm & (1 << (least_significant_byte - 1)) != 0;
                rm >> least_significant_byte
            } else if least_significant_byte == 32 {
                carry_out = rm & (1 << 31) != 0;
                0
            } else {
                carry_out = false;
                0
            }
        }
        0b100 => {
            // ASR immediate
            let rm = arm9.er(inst.get_byte(0, 3));
            let shift_imm = inst.get_word(7, 11);
            if shift_imm == 0 {
                let sign_bit = rm & (1 << 31) != 0;
                carry_out = sign_bit;
                if sign_bit {
                    0xFFFFFFFF
                } else {
                    0
                }
            } else {
                carry_out = (rm >> (shift_imm - 1)) & 1 != 0;
                ((rm as i32) >> shift_imm) as u32
            }
        }
        0b101 => {
            // ASR register
            let rm = arm9.er(inst.get_byte(0, 3));
            let rs = arm9.er(inst.get_byte(8, 11));
            let least_significant_byte = rs & 0b11111111;
            if least_significant_byte == 0 {
                rm
            } else if least_significant_byte < 32 {
                carry_out = rm & (1 << (least_significant_byte - 1)) != 0;
                ((rm as i32) >> least_significant_byte) as u32
            } else {
                let sign_bit = rm & (1 << 31) != 0;
                carry_out = sign_bit;
                if sign_bit {
                    0xFFFFFFFF
                } else {
                    0
                }
            }
        }
        0b110 => {
            // ROR immediate
            let rm = arm9.er(inst.get_byte(0, 3));
            let shift_imm = inst.get_word(7, 11);
            if shift_imm == 0 {
                // RRX
                carry_out = rm & 1 != 0;
                (arm9.cpsr.get_carry() as u32) << 31 | rm >> 1
            } else {
                carry_out = rm & (1 << 31) != 0;
                rm.rotate_right(shift_imm)
            }
        }
        0b111 => {
            // ROR register
            let rm = arm9.er(inst.get_byte(0, 3));
            let rs = arm9.er(inst.get_byte(8, 11));
            let least_significant_byte = rs & 0b11111111;
            let least_significant_bits = rs & 0b1111; // this is not explained lol
            if least_significant_byte == 0 {
                rm
            } else if least_significant_bits == 0 {
                carry_out = rm & (1 << 31) != 0;
                rm
            } else {
                carry_out = rm & (1 << (least_significant_bits - 1)) != 0;
                rm.rotate_right(least_significant_bits)
            }
        }
        _ => unreachable!(),
    };

    ShifterOperand {
        carry_out,
        second_source_operand,
    }
}
