use crate::nds::cpu::arm9::{
    arm9::Arm9Trait,
    models::{Context, ContextTrait, DisassemblyTrait, Instruction},
};

// AKA the addressing mode
pub struct ShifterOperand {
    pub carry_out: bool,
    pub second_source_operand: u32,
}

pub fn parse_immediate(ctx: &mut Context<Instruction, impl ContextTrait>) -> ShifterOperand {
    let mut carry_out = ctx.arm9.cpsr().get_carry();

    let immed_8 = ctx.inst.get_word(0, 7);
    let rotate_imm = ctx.inst.get_word(8, 11); // NOTE: rotate_imm must be even to be "legitimate"

    let rotated = immed_8.rotate_right(rotate_imm * 2);
    if rotate_imm != 0 {
        carry_out = rotated & (1 << 31) != 0;
    }

    ctx.dis.push_word_end_arg(rotated, "");

    ShifterOperand {
        carry_out,
        second_source_operand: rotated,
    }
}

pub fn parse_register(ctx: &mut Context<Instruction, impl ContextTrait>) -> ShifterOperand {
    let (arm9, inst) = (&mut ctx.arm9, &ctx.inst);

    let mut carry_out = arm9.cpsr().get_carry();
    let rm = inst.get_byte(0, 3);
    ctx.dis.push_reg_end_arg(rm, "");
    let rm = arm9.er(rm);

    let operand = inst.get_byte(4, 6);
    let second_source_operand = match operand {
        0b000 => {
            // LSL immediate
            ctx.dis.push_str_end_arg("LSL", ", ");
            let shift_imm = inst.get_word(7, 11);
            ctx.dis.push_word_end_arg(shift_imm, " ");

            if shift_imm == 0 {
                rm
            } else {
                carry_out = (rm >> (32 - shift_imm)) & 1 != 0;
                rm << shift_imm
            }
        }
        0b001 => {
            // LSL register
            ctx.dis.push_str_end_arg("LSL", ", ");
            let rs = inst.get_byte(8, 11);
            ctx.dis.push_reg_end_arg(rs, " ");
            let rs = arm9.er(rs);

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
            ctx.dis.push_str_end_arg("LSR", ", ");
            let shift_imm = inst.get_word(7, 11);
            ctx.dis.push_word_end_arg(shift_imm, " ");

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
            ctx.dis.push_str_end_arg("LSR", ", ");
            let rs = inst.get_byte(8, 11);
            ctx.dis.push_reg_end_arg(rs, " ");
            let rs = arm9.er(rs);

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
            ctx.dis.push_str_end_arg("ASR", ", ");
            let shift_imm = inst.get_word(7, 11);
            ctx.dis.push_word_end_arg(shift_imm, " ");

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
            ctx.dis.push_str_end_arg("ASR", ", ");
            let rs = inst.get_byte(8, 11);
            ctx.dis.push_reg_end_arg(rs, " ");
            let rs = arm9.er(rs);

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
            let shift_imm = inst.get_word(7, 11);
            if shift_imm == 0 {
                // RRX
                ctx.dis.push_str_end_arg("RRX", ", ");
                ctx.dis.push_word_end_arg(shift_imm, " ");
                carry_out = rm & 1 != 0;
                (arm9.cpsr().get_carry() as u32) << 31 | rm >> 1
            } else {
                ctx.dis.push_str_end_arg("ROR", ", ");
                ctx.dis.push_word_end_arg(shift_imm, " ");
                carry_out = rm & (1 << 31) != 0;
                rm.rotate_right(shift_imm)
            }
        }
        0b111 => {
            // ROR register
            ctx.dis.push_str_end_arg("ROR", ", ");
            let rs = inst.get_byte(8, 11);
            ctx.dis.push_reg_end_arg(rs, " ");
            let rs = arm9.er(rs);

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
