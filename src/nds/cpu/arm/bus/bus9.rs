use crate::nds::{cpu::arm::ArmKind, logger, shared::Shared};

use super::BusTrait;

#[derive(Default)]
pub struct Bus9 {
    pub bios: Vec<u8>,
}

impl BusTrait for Bus9 {
    fn kind() -> ArmKind {
        ArmKind::ARM9
    }

    fn load_bios(&mut self, bios: Vec<u8>) {
        logger::info(logger::LogSource::Arm9(0), "Successfully loaded BIOS");
        self.bios = bios;
    }

    fn load_bios_from_path(&mut self, path: &str) {
        let file = std::fs::read(path);
        match file {
            Ok(bios) => self.load_bios(bios),
            Err(e) => logger::error(
                logger::LogSource::Arm9(0),
                format!("Failed to load BIOS: {}", e),
            ),
        };
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
            0x04000000..=0x04000003 => {
                let value = shared.gpu2d_a.dispcnt.value().to_le_bytes();
                let len = T.min(value.len());
                bytes[..len].copy_from_slice(&value[..len]);
            }
            0x04000004..=0x04000005 => {
                let value = shared.gpu2d_a.dispstat.value().to_le_bytes();
                let len = T.min(value.len());
                bytes[..len].copy_from_slice(&value[..len]);
            }
            0x04000130..=0x04000131 => {
                let value = shared.keyinput.value().to_le_bytes();
                let len = T.min(value.len());
                bytes[..len].copy_from_slice(&value[..len]);
            }
            0x04000180..=0x04000183 => {
                let value = shared.ipcsync.value(true).to_le_bytes();
                let len = T.min(value.len());
                bytes[..len].copy_from_slice(&value[..len]);
            }
            0x04000304..=0x04000307 => {
                let value = shared.powcnt1.value().to_le_bytes();
                let len = T.min(value.len());
                bytes[..len].copy_from_slice(&value[..len]);
            }
            0xFFFF0000..=0xFFFF7FFF => {
                let addr = addr - 0xFFFF0000;
                bytes.copy_from_slice(&self.bios[addr..addr + T]);
            }
            _ => {
                logger::warn(
                    logger::LogSource::Bus9,
                    format!("Invalid read {} byte(s) address: {:#010X}", T, addr),
                );
            }
        }

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
            0x04000000..=0x04000003 => {
                shared.gpu2d_a.dispcnt = self
                    .update_reg_value(shared.gpu2d_a.dispcnt.value(), value)
                    .into();
            }
            0x04000180..=0x04000183 => {
                shared.ipcsync.set(
                    true,
                    self.update_reg_value(shared.ipcsync.value(true), value),
                );
            }
            0x04000304..=0x04000307 => {
                shared.powcnt1 = self.update_reg_value(shared.powcnt1.value(), value).into();
            }
            0x04000240..=0x04000249 => {
                let len = T.min(shared.vramcnt.len());
                shared.vramcnt[..len].copy_from_slice(&value[..len]);
            }
            0x06800000..=0x068A4000 => {
                let addr = addr - 0x06800000;
                shared.gpu2d_a.vram_lcdc_alloc[addr..addr + T].copy_from_slice(&value);
            }
            0xFFFF0000..=0xFFFF7FFF => {
                let addr = addr - 0xFFFF0000;
                self.bios[addr..addr + T].copy_from_slice(&value);
            }
            _ => {
                logger::warn(
                    logger::LogSource::Bus9,
                    format!("Invalid write {} byte(s) address: {:#010X}", T, addr),
                );
            }
        }
    }
}

impl Bus9 {
    #[inline(always)]
    fn update_reg_value<const T: usize>(&mut self, reg_value: u32, new_value: [u8; T]) -> u32 {
        // TODO: check if this is cursed
        let len = T.min(4);
        let mut dispcnt = reg_value.to_le_bytes();
        dispcnt[..len].copy_from_slice(&new_value[..len]);
        u32::from_le_bytes(dispcnt)
    }
}
