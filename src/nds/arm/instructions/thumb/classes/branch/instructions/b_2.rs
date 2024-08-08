use crate::nds::{
    arm::{
        arm::ArmTrait,
        instructions::{conditions::calculate_cond, thumb::Instruction},
        models::{Context, ContextTrait, DisassemblyTrait},
    },
    Bits,
};

// B (2)
pub fn b_2(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("B");

    let pc = (ctx.arm.r()[15] + 4) as i32;
    let signed_immed_11 = ctx.inst.get_word(0, 11);
    let signed_immed_11 = (pc + (sign_extend_11_to_32(signed_immed_11) << 1)) as u32;
    ctx.dis.push_word_arg(signed_immed_11);

    ctx.arm.set_r(15, signed_immed_11);

    1 // TODO: this is wrong
}

fn sign_extend_11_to_32(value: u32) -> i32 {
    let sign_bit = value.get_bit(1);

    let extended_value = if sign_bit {
        (value | 0xFFFFF800) as i32
    } else {
        (value & 0x000007FF) as i32
    };

    extended_value
}
