use crate::nds::cpu::{
    arm::{
        arm::ArmTrait,
        instructions::classes::load_store::halfword_or_ibyte::LoadStoreInstruction,
        models::{Context, ContextTrait},
    },
    bus::BusTrait,
};

// LDRH
#[inline(always)]
pub fn ldrh(ctx: &mut Context<LoadStoreInstruction, impl ContextTrait>, address: u32) -> u32 {
    // if bit 0 of the address is 1, the data is UNPREDICTABLE

    let data = ctx.bus.read_halfword(address);
    ctx.arm.r()[ctx.inst.destination_register] = data as u32;

    1
}
