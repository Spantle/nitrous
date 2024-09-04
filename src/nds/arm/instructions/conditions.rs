use crate::nds::arm::{
    models::{Context, ContextTrait, DisassemblyTrait},
    ArmTrait,
};

#[inline(always)]
pub fn calculate_cond<T>(cond: u8, ctx: &mut Context<T, impl ContextTrait>) -> bool {
    let s = ctx.arm.cpsr();
    match cond {
        0b0000 => {
            ctx.dis.set_cond(['E', 'Q']);
            s.get_zero()
        }
        0b0001 => {
            ctx.dis.set_cond(['N', 'E']);
            !s.get_zero()
        }
        0b0010 => {
            ctx.dis.set_cond(['C', 'S']);
            s.get_carry()
        }
        0b0011 => {
            ctx.dis.set_cond(['C', 'C']);
            !s.get_carry()
        }
        0b0100 => {
            ctx.dis.set_cond(['M', 'I']);
            s.get_negative()
        }
        0b0101 => {
            ctx.dis.set_cond(['P', 'L']);
            !s.get_negative()
        }
        0b0110 => {
            ctx.dis.set_cond(['V', 'S']);
            s.get_overflow()
        }
        0b0111 => {
            ctx.dis.set_cond(['V', 'C']);
            !s.get_overflow()
        }
        0b1000 => {
            ctx.dis.set_cond(['H', 'I']);
            s.get_carry() && !s.get_zero()
        }
        0b1001 => {
            ctx.dis.set_cond(['L', 'S']);
            !s.get_carry() || s.get_zero()
        }
        0b1010 => {
            ctx.dis.set_cond(['G', 'E']);
            s.get_negative() == s.get_overflow()
        }
        0b1011 => {
            ctx.dis.set_cond(['L', 'T']);
            s.get_negative() != s.get_overflow()
        }
        0b1100 => {
            ctx.dis.set_cond(['G', 'T']);
            !s.get_zero() && s.get_negative() == s.get_overflow()
        }
        0b1101 => {
            ctx.dis.set_cond(['L', 'E']);
            s.get_zero() || s.get_negative() != s.get_overflow()
        }
        0b1110 => true,
        0b1111 => true,
        _ => unreachable!(),
    }
}
