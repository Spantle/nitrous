// thank you for the help with this Leo (@Arduano)

use crate::nds::{
    cpu::{arm9::arm9::Arm9Trait, bus::BusTrait},
    logger::LoggerTrait,
};

use super::disassembly::DisassemblyTrait;

pub struct Context<'a, Inst, Ctx: ContextTrait> {
    pub inst: Inst,
    pub arm9: &'a mut Ctx::Arm9,
    pub bus: &'a mut Ctx::Bus,

    pub dis: &'a mut Ctx::Dis,
    pub logger: &'a mut Ctx::Logger,
}

pub trait ContextTrait {
    type Arm9: Arm9Trait;
    type Bus: BusTrait;

    type Dis: DisassemblyTrait;
    type Logger: LoggerTrait;
}

impl<'a, Inst, Arm9: Arm9Trait, Bus: BusTrait, Dis: DisassemblyTrait, Logger: LoggerTrait>
    Context<'a, Inst, ContextItems<Arm9, Bus, Dis, Logger>>
{
    pub fn new(
        inst: Inst,
        arm9: &'a mut Arm9,
        bus: &'a mut Bus,
        dis: &'a mut Dis,
        logger: &'a mut Logger,
    ) -> Self {
        Context {
            inst,
            arm9,
            bus,

            dis,
            logger,
        }
    }
}

// impl<'a, Inst, Ctx: ContextTrait> Context<'a, Inst, Ctx> {
//     pub fn with_inst<NewInst>(&mut self, inst: NewInst) -> Context<'a, NewInst, Ctx> {
//         Context {
//             inst,
//             arm9: &mut self.arm9,
//             bus: &mut self.bus,
//             dis: &mut self.dis,
//         }
//     }
// }

pub struct ContextItems<Arm9: Arm9Trait, Bus: BusTrait, Dis: DisassemblyTrait, Logger: LoggerTrait>
{
    _phantom: std::marker::PhantomData<(Arm9, Bus, Dis, Logger)>,
}

impl<Arm9: Arm9Trait, Bus: BusTrait, Dis: DisassemblyTrait, Logger: LoggerTrait> ContextTrait
    for ContextItems<Arm9, Bus, Dis, Logger>
{
    type Arm9 = Arm9;
    type Bus = Bus;
    type Dis = Dis;
    type Logger = Logger;
}
