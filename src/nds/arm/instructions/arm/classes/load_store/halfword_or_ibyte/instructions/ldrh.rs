use crate::nds::arm::{
    instructions::arm::classes::load_store::halfword_or_ibyte::LoadStoreInstruction,
    models::{Context, ContextTrait},
    ArmTrait,
};

// LDRH
#[inline(always)]
pub fn ldrh(ctx: &mut Context<LoadStoreInstruction, impl ContextTrait>, address: u32) -> u32 {
    // if bit 0 of the address is 1, the data is UNPREDICTABLE

    let data = ctx.arm.read_halfword(ctx.bus, ctx.shared, ctx.dma, address);
    ctx.arm.set_r(ctx.inst.destination_register, data as u32);

    1
}
