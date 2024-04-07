mod addressing_mode;
mod instructions;
mod lookup;

pub use lookup::lookup;

use crate::nds::cpu::arm9::models::{Bits, Context, ContextTrait, DisassemblyTrait, Instruction};

pub struct LoadStoreMultipleInstruction {
    pub register_list: u16, // bits[15:0] result
    pub start_address: u32, // cheating
    pub end_address: u32,   // cheating
}

impl LoadStoreMultipleInstruction {
    fn new(inst_set: u16, ctx: &mut Context<Instruction, impl ContextTrait>) -> Self {
        let (start_address, end_address) = addressing_mode::parse(inst_set, ctx);
        let register_list = ctx.inst.get_halfword(0, 15);

        ctx.dis.push_str_end_arg("", Some("{"));
        let mut prefix = "";
        for i in 0..=15 {
            if register_list.get_bit(i as u16) {
                ctx.dis.push_reg_end_arg(i, Some(prefix));
                prefix = ",";
            }
        }
        ctx.dis.push_str_end_arg("", Some("}"));

        LoadStoreMultipleInstruction {
            register_list,
            start_address,
            end_address,
        }
    }
}
