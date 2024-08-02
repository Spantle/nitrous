use crate::nds::arm::{
    arm::ArmTrait,
    bus::BusTrait,
    instructions::arm::Instruction,
    models::{Bits, Context, ContextTrait, DisassemblyTrait},
};

// AKA the addressing mode
pub struct ShifterOperand {
    pub carry_out: bool,
    pub second_source_operand: u32,
}

pub fn parse_immediate(ctx: &mut Context<Instruction, impl ContextTrait>) -> ShifterOperand {
    let mut carry_out = ctx.arm.cpsr().get_carry();

    let immed_8 = ctx.inst.get_word(0, 7);
    let rotate_imm = ctx.inst.get_word(8, 11); // NOTE: rotate_imm must be even to be "legitimate"

    let rotated = immed_8.rotate_right(rotate_imm * 2);
    if rotate_imm != 0 {
        carry_out = rotated.get_bit(31);
    }

    ctx.dis.push_word_end_arg(rotated, None);

    ShifterOperand {
        carry_out,
        second_source_operand: rotated,
    }
}

pub fn parse_register(ctx: &mut Context<Instruction, impl ContextTrait>) -> ShifterOperand {
    let (arm, inst) = (&mut ctx.arm, &ctx.inst);

    let mut carry_out = arm.cpsr().get_carry();
    let rm = inst.get_byte(0, 3);
    ctx.dis.push_reg_end_arg(rm, None);

    let operand = inst.get_byte(4, 6);
    let second_source_operand = match operand {
        0b000 => {
            // LSL immediate
            let rm = arm.er(rm);
            let shift_imm = inst.get_word(7, 11);

            // the default if you don't want any shifting is to LSL shift 0
            // so in this scenario we exclude the LSL from the disassembly
            if shift_imm != 0 {
                ctx.dis.push_str_end_arg("LSL", Some(", "));
                ctx.dis.push_word_end_arg(shift_imm, Some(" "));
            }

            if shift_imm == 0 {
                rm
            } else {
                carry_out = rm.get_bit(32 - shift_imm);
                rm << shift_imm
            }
        }
        0b001 => {
            // LSL register
            let rm = arm.eru(rm);
            ctx.dis.push_str_end_arg("LSL", Some(", "));
            let rs = inst.get_byte(8, 11);
            ctx.dis.push_reg_end_arg(rs, Some(" "));
            let rs = arm.er(rs);

            let least_significant_byte = rs & 0b11111111;
            if least_significant_byte == 0 {
                rm
            } else if least_significant_byte < 32 {
                carry_out = rm.get_bit(32 - least_significant_byte);
                rm << least_significant_byte
            } else if least_significant_byte == 32 {
                carry_out = rm.get_bit(0);
                0
            } else {
                carry_out = false;
                0
            }
        }
        0b010 => {
            // LSR immediate
            let rm = arm.er(rm);
            ctx.dis.push_str_end_arg("LSR", Some(", "));
            let shift_imm = inst.get_word(7, 11);
            ctx.dis.push_word_end_arg(shift_imm, Some(" "));

            if shift_imm == 0 {
                carry_out = rm.get_bit(31);
                0
            } else {
                carry_out = rm.get_bit(shift_imm - 1);
                rm >> shift_imm
            }
        }
        0b011 => {
            // LSR register
            let rm = arm.eru(rm);
            ctx.dis.push_str_end_arg("LSR", Some(", "));
            let rs = inst.get_byte(8, 11);
            ctx.dis.push_reg_end_arg(rs, Some(" "));
            let rs = arm.er(rs);

            let least_significant_byte = rs & 0b11111111;
            if least_significant_byte == 0 {
                rm
            } else if least_significant_byte < 32 {
                carry_out = rm.get_bit(least_significant_byte - 1);
                rm >> least_significant_byte
            } else if least_significant_byte == 32 {
                carry_out = rm.get_bit(31);
                0
            } else {
                carry_out = false;
                0
            }
        }
        0b100 => {
            // ASR immediate
            let rm = arm.er(rm);
            ctx.dis.push_str_end_arg("ASR", Some(", "));
            let shift_imm = inst.get_word(7, 11);
            ctx.dis.push_word_end_arg(shift_imm, Some(" "));

            if shift_imm == 0 {
                let sign_bit = rm.get_bit(31);
                carry_out = sign_bit;
                if sign_bit {
                    0xFFFFFFFF
                } else {
                    0
                }
            } else {
                carry_out = rm.get_bit(shift_imm - 1);
                ((rm as i32) >> shift_imm) as u32
            }
        }
        0b101 => {
            // ASR register
            let rm = arm.eru(rm);
            ctx.dis.push_str_end_arg("ASR", Some(", "));
            let rs = inst.get_byte(8, 11);
            ctx.dis.push_reg_end_arg(rs, Some(" "));
            let rs = arm.er(rs);

            let least_significant_byte = rs & 0b11111111;
            if least_significant_byte == 0 {
                rm
            } else if least_significant_byte < 32 {
                carry_out = rm.get_bit(least_significant_byte - 1);
                ((rm as i32) >> least_significant_byte) as u32
            } else {
                let sign_bit = rm.get_bit(31);
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
            let rm = arm.er(rm);
            let shift_imm = inst.get_word(7, 11);
            if shift_imm == 0 {
                // RRX
                ctx.dis.push_str_end_arg("RRX", Some(", "));
                ctx.dis.push_word_end_arg(shift_imm, Some(" "));
                carry_out = rm.get_bit(0);
                (arm.cpsr().get_carry() as u32) << 31 | rm >> 1
            } else {
                ctx.dis.push_str_end_arg("ROR", Some(", "));
                ctx.dis.push_word_end_arg(shift_imm, Some(" "));
                carry_out = rm.get_bit(31);
                rm.rotate_right(shift_imm)
            }
        }
        0b111 => {
            // ROR register
            let rm = arm.eru(rm);
            ctx.dis.push_str_end_arg("ROR", Some(", "));
            let rs = inst.get_byte(8, 11);
            ctx.dis.push_reg_end_arg(rs, Some(" "));
            let rs = arm.er(rs);

            let least_significant_byte = rs & 0b11111111;
            let least_significant_bits = rs & 0b1111; // this is not explained lol
            if least_significant_byte == 0 {
                rm
            } else if least_significant_bits == 0 {
                carry_out = rm.get_bit(31);
                rm
            } else {
                carry_out = rm.get_bit(least_significant_bits - 1);
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

// "unpredictable" behaviour
trait Funny<Bus: BusTrait> {
    fn eru(&mut self, r: u8) -> u32;
}

impl<T, Bus: BusTrait> Funny<Bus> for T
where
    T: ArmTrait<Bus>,
{
    fn eru(&mut self, r: u8) -> u32 {
        match r {
            15 => self.r()[15] + 12,
            _ => self.r()[r],
        }
    }
}
