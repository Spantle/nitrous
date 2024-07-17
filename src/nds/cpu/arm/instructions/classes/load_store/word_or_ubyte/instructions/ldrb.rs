use crate::nds::cpu::arm::{
    arm::ArmTrait,
    instructions::classes::load_store::word_or_ubyte::LoadStoreInstruction,
    models::{Context, ContextTrait},
};

// LDRB
#[inline(always)]
pub fn ldrb(ctx: &mut Context<LoadStoreInstruction, impl ContextTrait>, address: u32) -> u32 {
    let (arm, inst) = (&mut ctx.arm, &ctx.inst);

    arm.set_r(
        inst.destination_register,
        arm.read_byte(ctx.bus, ctx.shared, address) as u32,
    );

    1
}
