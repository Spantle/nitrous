use crate::nds::arm::{
    instructions::arm::classes::load_store::halfword_or_ibyte::LoadStoreInstruction,
    models::{Context, ContextTrait},
    ArmTrait,
};

// STRH
#[inline(always)]
pub fn strh(ctx: &mut Context<LoadStoreInstruction, impl ContextTrait>, address: u32) -> u32 {
    // if bit 0 of the address is 1, the data is UNPREDICTABLE

    let data = ctx.arm.er(ctx.inst.destination_register) as u16;
    ctx.arm
        .write_halfword(ctx.bus, ctx.shared, ctx.dma, address, data);

    1
}
