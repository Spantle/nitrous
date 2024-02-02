use crate::nds::{cartridge::Cartridge, gpu::gpu2d::Gpu2d, logger};

use super::arm9::POWCNT1;

pub struct Bus {
    pub cart: Cartridge,
    pub mem: Vec<u8>,
    pub gpu2d_a: Gpu2d,

    pub powcnt1: POWCNT1, // 0x04000304
}

impl Default for Bus {
    fn default() -> Self {
        Bus {
            cart: Cartridge::default(),
            mem: vec![0; 1024 * 1024 * 4],
            gpu2d_a: Gpu2d::default(),

            powcnt1: POWCNT1::default(),
        }
    }
}

impl Bus {
    pub fn write_byte(&mut self, addr: u32, value: u8) {
        let addr = addr as usize;
        match addr {
            0x02000000..=0x023FFFFF => {
                let addr = addr % 0x02000000;
                self.mem[addr] = value;
            }
            _ => {
                logger::error(
                    logger::LogSource::Bus9,
                    format!("Invalid write byte address: {:#010X}", addr),
                );
            }
        }
    }

    pub fn read_word(&self, addr: u32) -> u32 {
        let addr = addr as usize;
        match addr {
            0x02000000..=0x023FFFFF => {
                let addr = addr % 0x02000000;
                let mut bytes = [0; 4];
                bytes.copy_from_slice(&self.mem[addr..addr + 4]);
                u32::from_le_bytes(bytes)
            }
            0x04000000 => self.gpu2d_a.dispcnt.value(),
            0x04000304 => self.powcnt1.value(),
            _ => {
                logger::error(
                    logger::LogSource::Bus9,
                    format!("Invalid read address: {:#010X}", addr),
                );
                0
            }
        }
    }

    pub fn write_word(&mut self, addr: u32, value: u32) {
        let addr = addr as usize;
        match addr {
            0x02000000..=0x023FFFFF => {
                let addr = addr % 0x02000000;
                self.mem[addr..addr + 4].copy_from_slice(&value.to_le_bytes());
            }
            0x04000000 => self.gpu2d_a.dispcnt = value.into(),
            0x04000304 => self.powcnt1 = value.into(),
            _ => {
                logger::error(
                    logger::LogSource::Bus9,
                    format!("Invalid write address: {:#010X}", addr),
                );
            }
        }
    }

    // it's 1am i don't know what to call this
    pub fn write_bulk(&mut self, addr: u32, data: Vec<u8>) {
        let addr = addr as usize;
        match addr {
            0x02000000..=0x023FFFFF => {
                let addr = addr % 0x02000000;
                self.mem[addr..addr + data.len()].copy_from_slice(&data);
            }
            _ => {
                logger::error(
                    logger::LogSource::Bus9,
                    format!("Invalid write bulk address: {:#010X}", addr),
                );
            }
        }
    }
}
