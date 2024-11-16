use crate::nds::arm::{
    instructions::arm::classes::load_store::word_or_ubyte::LoadStoreInstruction,
    models::{Context, ContextTrait},
    ArmTrait,
};

// LDRB
#[inline(always)]
pub fn ldrb(ctx: &mut Context<LoadStoreInstruction, impl ContextTrait>, address: u32) -> u32 {
    let (arm, inst) = (&mut ctx.arm, &ctx.inst);

    arm.set_r(
        inst.destination_register,
        arm.read_byte(ctx.bus, ctx.shared, ctx.dma, address) as u32,
    );

    1
}
