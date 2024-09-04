// thank you for the help with this Leo (@Arduano)

use crate::nds::{arm::ArmTrait, bus::BusTrait, logger::LoggerTrait, shared::Shared};

use super::disassembly::DisassemblyTrait;

pub struct Context<'a, Inst, Ctx: ContextTrait> {
    pub inst: Inst,
    pub arm: &'a mut Ctx::Arm,
    pub bus: &'a mut Ctx::Bus,
    pub shared: &'a mut Shared,

    pub dis: &'a mut Ctx::Dis,
    pub logger: &'a mut Ctx::Logger,
}

pub trait ContextTrait {
    type Arm: ArmTrait<Self::Bus>;
    type Bus: BusTrait;

    type Dis: DisassemblyTrait;
    type Logger: LoggerTrait;
}

impl<'a, Inst, Arm: ArmTrait<Bus>, Bus: BusTrait, Dis: DisassemblyTrait, Logger: LoggerTrait>
    Context<'a, Inst, ContextItems<Arm, Bus, Dis, Logger>>
{
    pub fn new(
        inst: Inst,
        arm: &'a mut Arm,
        bus: &'a mut Bus,
        shared: &'a mut Shared,
        dis: &'a mut Dis,
        logger: &'a mut Logger,
    ) -> Self {
        Context {
            inst,
            arm,
            bus,
            shared,

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

pub struct ContextItems<
    Arm: ArmTrait<Bus>,
    Bus: BusTrait,
    Dis: DisassemblyTrait,
    Logger: LoggerTrait,
> {
    _phantom: std::marker::PhantomData<(Arm, Bus, Dis, Logger)>,
}

impl<Arm: ArmTrait<Bus>, Bus: BusTrait, Dis: DisassemblyTrait, Logger: LoggerTrait> ContextTrait
    for ContextItems<Arm, Bus, Dis, Logger>
{
    type Arm = Arm;
    type Bus = Bus;
    type Dis = Dis;
    type Logger = Logger;
}
