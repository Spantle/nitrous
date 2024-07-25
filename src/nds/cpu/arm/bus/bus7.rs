use crate::nds::{cpu::arm::ArmKind, logger, shared::Shared};

use super::BusTrait;

#[derive(Default)]
pub struct Bus7 {}

impl BusTrait for Bus7 {
    fn kind() -> ArmKind {
        ArmKind::ARM7
    }

    fn load_bios(&mut self, _bios: Vec<u8>) {
        logger::warn(logger::LogSource::Bus7, "BIOS loading not implemented");
    }
    fn load_bios_from_path(&mut self, _path: &str) {
        logger::warn(
            logger::LogSource::Bus7,
            "BIOS loading (path) not implemented",
        );
    }

    fn read_byte(&self, shared: &mut Shared, addr: u32) -> u8 {
        self.read_slice::<1>(shared, addr)[0]
    }
    fn read_halfword(&self, shared: &mut Shared, addr: u32) -> u16 {
        let bytes = self.read_slice::<2>(shared, addr);
        u16::from_le_bytes(bytes)
    }
    fn read_word(&self, shared: &mut Shared, addr: u32) -> u32 {
        let bytes = self.read_slice::<4>(shared, addr);
        u32::from_le_bytes(bytes)
    }

    fn write_byte(&mut self, shared: &mut Shared, addr: u32, value: u8) {
        self.write_slice::<1>(shared, addr, [value]);
    }
    fn write_halfword(&mut self, shared: &mut Shared, addr: u32, value: u16) {
        self.write_slice::<2>(shared, addr, value.to_le_bytes());
    }
    fn write_word(&mut self, shared: &mut Shared, addr: u32, value: u32) {
        self.write_slice::<4>(shared, addr, value.to_le_bytes());
    }

    #[inline(always)]
    fn read_slice<const T: usize>(&self, shared: &mut Shared, addr: u32) -> [u8; T] {
        let addr = addr as usize / T * T;
        let mut bytes = [0; T];
        match addr {
            0x02000000..=0x02FFFFFF => {
                let addr = (addr - 0x02000000) % 400000;
                bytes.copy_from_slice(&shared.psram[addr..addr + T]);
            }
            0x03000000..=0x037FFFFF => {
                let addr = (addr - 0x03000000) % 0x8000;
                bytes.copy_from_slice(&shared.wram[addr..addr + T]);
            }
            0x04000180..=0x04000183 => {
                let value = shared.ipcsync.value(false).to_le_bytes();
                let len = T.min(value.len());
                bytes[..len].copy_from_slice(&value[..len]);
            }
            _ => {
                logger::warn(
                    logger::LogSource::Bus7,
                    format!("Invalid read {} byte(s) address: {:#010X}", T, addr),
                );
            }
        };

        bytes
    }

    #[inline(always)]
    fn write_slice<const T: usize>(&mut self, shared: &mut Shared, addr: u32, value: [u8; T]) {
        let addr = addr as usize / T * T;
        match addr {
            0x02000000..=0x02FFFFFF => {
                let addr = (addr - 0x02000000) % 400000;
                shared.psram[addr..addr + T].copy_from_slice(&value);
            }
            0x03000000..=0x037FFFFF => {
                let addr = (addr - 0x03000000) % 0x8000;
                shared.wram[addr..addr + T].copy_from_slice(&value);
            }
            0x04000180..=0x04000183 => {
                shared.ipcsync.set(
                    false,
                    self.update_reg_value(shared.ipcsync.value(false), value),
                );
            }
            _ => {
                logger::warn(
                    logger::LogSource::Bus7,
                    format!("Invalid write {} byte(s) address: {:#010X}", T, addr),
                );
            }
        };
    }
}

impl Bus7 {
    #[inline(always)]
    fn update_reg_value<const T: usize>(&mut self, reg_value: u32, new_value: [u8; T]) -> u32 {
        // TODO: check if this is cursed
        let len = T.min(4);
        let mut dispcnt = reg_value.to_le_bytes();
        dispcnt[..len].copy_from_slice(&new_value[..len]);
        u32::from_le_bytes(dispcnt)
    }
}
