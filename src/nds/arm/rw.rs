use crate::nds::{bus::BusTrait, shared::Shared};

use super::{Arm, ArmKind, ArmTrait};

pub trait ArmInternalRW<Bus: BusTrait> {
    fn read_bulk(&self, bus: &mut Bus, shared: &mut Shared, addr: u32, len: u32) -> Vec<u8>;
    fn write_bulk(&mut self, bus: &mut Bus, shared: &mut Shared, addr: u32, data: Vec<u8>);
    fn read_slice<const T: usize>(&self, bus: &mut Bus, shared: &mut Shared, addr: u32) -> [u8; T];
    fn write_slice<const T: usize>(
        &mut self,
        bus: &mut Bus,
        shared: &mut Shared,
        addr: u32,
        value: [u8; T],
    );
}

impl<Bus: BusTrait> ArmInternalRW<Bus> for Arm<Bus> {
    fn read_bulk(&self, bus: &mut Bus, shared: &mut Shared, addr: u32, len: u32) -> Vec<u8> {
        let mut bytes = vec![];
        for i in 0..len {
            bytes.push(self.read_byte(bus, shared, addr + i));
        }
        bytes
    }

    fn write_bulk(&mut self, bus: &mut Bus, shared: &mut Shared, addr: u32, data: Vec<u8>) {
        (0..data.len()).for_each(|i| {
            self.write_byte(bus, shared, addr + i as u32, data[i]);
        });
    }

    #[inline(always)]
    fn read_slice<const T: usize>(
        &self,
        bus: &mut Bus,
        shared: &mut Shared,
        orig_addr: u32,
    ) -> [u8; T] {
        let addr = orig_addr as usize / T * T;
        let mut bytes = [0; T];

        let (data_tcm_base, data_tcm_size, inst_tcm_base, inst_tcm_size) = (
            self.cp15.data_tcm_base as usize,
            self.cp15.data_tcm_size as usize,
            self.cp15.inst_tcm_base as usize,
            self.cp15.inst_tcm_size as usize,
        );
        let (data_tcm_end, inst_tcm_end) =
            (data_tcm_base + data_tcm_size, inst_tcm_base + inst_tcm_size);
        match Bus::KIND {
            ArmKind::Arm9 => {
                if !self.cp15.control_register.get_instruction_tcm_load_mode()
                    && addr >= inst_tcm_base
                    && addr < inst_tcm_end
                {
                    let addr = (addr - inst_tcm_base) % self.cp15.inst_tcm.len();
                    bytes.copy_from_slice(&self.cp15.inst_tcm[addr..addr + T]);
                    return bytes;
                }
                if !self.cp15.control_register.get_data_tcm_load_mode()
                    && addr >= data_tcm_base
                    && addr < data_tcm_end
                {
                    let addr = (addr - data_tcm_base) % self.cp15.data_tcm.len();
                    bytes.copy_from_slice(&self.cp15.data_tcm[addr..addr + T]);
                    return bytes;
                }

                bus.read_slice::<T>(shared, orig_addr)
            }
            ArmKind::Arm7 => match addr {
                0x03800000..=0x03FFFFFF => {
                    let addr = (addr - 0x03800000) % 0x10000;
                    bytes.copy_from_slice(&self.wram7[addr..addr + T]);
                    bytes
                }
                _ => bus.read_slice::<T>(shared, orig_addr),
            },
        }
    }

    #[inline(always)]
    fn write_slice<const T: usize>(
        &mut self,
        bus: &mut Bus,
        shared: &mut Shared,
        orig_addr: u32,
        value: [u8; T],
    ) {
        let addr = orig_addr as usize / T * T;

        match Bus::KIND {
            ArmKind::Arm9 => {
                let (data_tcm_base, data_tcm_size, inst_tcm_base, inst_tcm_size) = (
                    self.cp15.data_tcm_base as usize,
                    self.cp15.data_tcm_size as usize,
                    self.cp15.inst_tcm_base as usize,
                    self.cp15.inst_tcm_size as usize,
                );
                let (data_tcm_end, inst_tcm_end) =
                    (data_tcm_base + data_tcm_size, inst_tcm_base + inst_tcm_size);

                if addr >= inst_tcm_base && addr < inst_tcm_end {
                    let addr = (addr - inst_tcm_base) % self.cp15.inst_tcm.len();
                    self.cp15.inst_tcm[addr..addr + T].copy_from_slice(&value);
                    return;
                }
                if addr >= data_tcm_base && addr < data_tcm_end {
                    let addr = (addr - data_tcm_base) % self.cp15.data_tcm.len();
                    self.cp15.data_tcm[addr..addr + T].copy_from_slice(&value);
                    return;
                }

                bus.write_slice::<T>(shared, orig_addr, value)
            }
            ArmKind::Arm7 => match addr {
                0x03800000..=0x03FFFFFF => {
                    let addr = (addr - 0x03800000) % 0x10000;
                    self.wram7[addr..addr + T].copy_from_slice(&value);
                }
                _ => bus.write_slice::<T>(shared, orig_addr, value),
            },
        };
    }
}