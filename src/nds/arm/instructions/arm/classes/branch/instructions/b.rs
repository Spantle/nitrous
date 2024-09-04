use crate::nds::{
    arm::{
        instructions::arm::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
        ArmTrait,
    },
    Bits,
};

// B, BL, BLX (1)
pub fn b<const L: bool, const X: bool>(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    match (L, X) {
        (true, true) => ctx.dis.set_inst("BLX"),
        (true, false) => ctx.dis.set_inst("BL"),
        (false, _) => ctx.dis.set_inst("B"),
    }

    let (arm, inst) = (&mut ctx.arm, &ctx.inst);
    if L {
        arm.set_r(14, arm.r()[15].wrapping_add(4));
    }

    let signed_immed_24 = inst.get_word(0, 23).sign_extend(24);
    let signed_immed_24 = if X {
        let h: i32 = inst.get_bit(24).into();
        (signed_immed_24 << 2) + (h << 1)
    } else {
        signed_immed_24 << 2
    };
    let result = (arm.er(15) as i32).wrapping_add(signed_immed_24) as u32; // TODO: probably not the best conversion?
    ctx.dis.push_word_arg(result);
    arm.set_r(15, result);

    3
}
