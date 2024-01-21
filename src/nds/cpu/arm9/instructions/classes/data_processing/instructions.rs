use crate::nds::cpu::arm9::Arm9;

use super::DataProcessingInstruction;

// MOV, MOVS
pub fn mov<const S: bool>(inst: DataProcessingInstruction, arm9: &mut Arm9) -> u32 {
    arm9.r[inst.destination_register] = inst.second_source_operand;

    if S {
        if inst.destination_register == 15 {
            arm9.cpsr = arm9.get_spsr();
        } else {
            arm9.cpsr
                .set_negative(inst.second_source_operand & (1 << 31) != 0);
            arm9.cpsr.set_zero(inst.second_source_operand == 0);
            arm9.cpsr.set_carry(inst.carry_out);
        }
    }

    1 // TODO: figure out how many cycles MOV is
}
