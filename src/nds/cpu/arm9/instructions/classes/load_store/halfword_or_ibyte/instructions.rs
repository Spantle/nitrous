use crate::nds::cpu::{
    arm9::{
        arm9::Arm9Trait,
        models::{Context, ContextTrait},
    },
    bus::BusTrait,
};

use super::LoadStoreInstruction;

// LDRH
#[inline(always)]
pub fn ldrh(ctx: &mut Context<LoadStoreInstruction, impl ContextTrait>, address: u32) -> u32 {
    // if bit 0 of the address is 1, the data is UNPREDICTABLE

    let data = ctx.bus.read_halfword(address);
    ctx.arm9.r()[ctx.inst.destination_register] = data as u32;

    1
}

// STRH
#[inline(always)]
pub fn strh(ctx: &mut Context<LoadStoreInstruction, impl ContextTrait>, address: u32) -> u32 {
    // if bit 0 of the address is 1, the data is UNPREDICTABLE

    let data = ctx.arm9.er(ctx.inst.destination_register) as u16;
    ctx.bus.write_halfword(address, data);

    1
}
