use crate::nds::arm::{
    instructions::arm::classes::load_store::halfword_or_ibyte::LoadStoreInstruction,
    models::{Context, ContextTrait},
    ArmTrait,
};

// LDRSH
#[inline(always)]
pub fn ldrsh(ctx: &mut Context<LoadStoreInstruction, impl ContextTrait>, address: u32) -> u32 {
    // if bit 0 of the address is 1, the data is UNPREDICTABLE

    let data = ctx.arm.read_halfword(ctx.bus, ctx.shared, address) as i16 as u32;
    ctx.arm.set_r(ctx.inst.destination_register, data);

    1
}
