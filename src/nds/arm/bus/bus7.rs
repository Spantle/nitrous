use crate::nds::{arm::ArmKind, dma::DMA, logger, shared::Shared, Bits, Bytes};

use super::BusTrait;

#[derive(Default)]
pub struct Bus7 {}

impl BusTrait for Bus7 {
    const KIND: ArmKind = ArmKind::ARM7;

    fn load_bios(&mut self, _bios: Vec<u8>) {
        logger::error(logger::LogSource::Bus7, "BIOS loading not implemented");
    }
    fn load_bios_from_path(&mut self, _path: &str) {
        logger::error(
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
                let addr = (addr - 0x02000000) % 0x400000;
                bytes.copy_from_slice(&shared.psram[addr..addr + T]);
                bytes
            }
            0x03000000..=0x037FFFFF => {
                let addr = (addr - 0x03000000) % 0x8000;
                bytes.copy_from_slice(&shared.wram[addr..addr + T]);
                bytes
            }
            0x04000004..=0x04000005 => shared.gpu2d_a.dispstat.value().to_bytes::<T>(),
            0x04000130..=0x04000131 => shared.keyinput.value().to_bytes::<T>(),
            0x04000180..=0x04000183 => shared.ipcsync.value::<false>().to_bytes::<T>(),
            _ => {
                if let Some(bytes) = shared.dma7.read_slice::<T>(addr) {
                    return bytes;
                }

                logger::warn(
                    logger::LogSource::Bus7,
                    format!("Invalid read {} byte(s) at address {:#010X}", T, addr),
                );
                bytes
            }
        }
    }

    #[inline(always)]
    fn write_slice<const T: usize>(&mut self, shared: &mut Shared, addr: u32, value: [u8; T]) {
        let addr = addr as usize / T * T;
        match addr {
            0x02000000..=0x02FFFFFF => {
                let addr = (addr - 0x02000000) % 0x400000;
                shared.psram[addr..addr + T].copy_from_slice(&value);
            }
            0x03000000..=0x037FFFFF => {
                let addr = (addr - 0x03000000) % 0x8000;
                shared.wram[addr..addr + T].copy_from_slice(&value);
            }
            0x04000180..=0x04000183 => {
                shared.ipcsync.set::<false>(value.into_word());
            }
            _ => {
                let success = shared.dma7.write_slice::<T>(addr, value);
                if !success {
                    logger::warn(
                        logger::LogSource::Bus7,
                        format!(
                            "Invalid write {} byte(s) at address {:#010X}: {:#010X}",
                            T,
                            addr,
                            value.into_word()
                        ),
                    );
                }
            }
        };
    }
}
