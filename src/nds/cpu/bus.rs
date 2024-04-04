use crate::nds::{cartridge::Cartridge, gpu::gpu2d::Gpu2d, logger};

use super::arm9::models::POWCNT1;

pub struct Bus {
    pub arm9_bios: Vec<u8>,
    pub cart: Cartridge,
    pub mem: Vec<u8>,
    pub gpu2d_a: Gpu2d,

    // TODO: move these, it shouldn't be accessible by the DMA
    pub inst_tcm: Vec<u8>,
    pub data_tcm: [u8; 1024 * 16],

    pub vramcnt: [u8; 10], // 0x04000240 - 0x04000249, 0x04000247 is wramcnt
    pub powcnt1: POWCNT1,  // 0x04000304
}

pub trait BusTrait {
    fn read_byte(&mut self, addr: u32) -> u8;
    fn read_halfword(&mut self, addr: u32) -> u16;
    fn read_word(&self, addr: u32) -> u32;
    fn read_bulk(&self, addr: u32, len: u32) -> Vec<u8>;

    fn write_byte(&mut self, addr: u32, value: u8);
    fn write_halfword(&mut self, addr: u32, value: u16);
    fn write_word(&mut self, addr: u32, value: u32);
    fn write_bulk(&mut self, addr: u32, data: Vec<u8>);
}

impl Default for Bus {
    fn default() -> Self {
        Bus {
            arm9_bios: vec![0; 1024 * 32],
            cart: Cartridge::default(),
            mem: vec![0; 1024 * 1024 * 4],
            gpu2d_a: Gpu2d::default(),

            inst_tcm: vec![0; 1024 * 32],
            data_tcm: [0; 1024 * 16],

            vramcnt: [0; 10],
            powcnt1: POWCNT1::default(),
        }
    }
}

impl BusTrait for Bus {
    fn read_byte(&mut self, addr: u32) -> u8 {
        let addr = addr as usize;
        match addr {
            0x00000000..=0x00007FFF => self.inst_tcm[addr],
            0x02000000..=0x023FFFFF => {
                let addr = addr - 0x02000000;
                self.mem[addr]
            }
            _ => {
                logger::warn(
                    logger::LogSource::Bus9,
                    format!("Invalid read byte address: {:#010X}", addr),
                );
                0
            }
        }
    }

    fn read_halfword(&mut self, addr: u32) -> u16 {
        let addr = addr as usize;
        match addr {
            0x02000000..=0x023FFFFF => {
                let addr = addr - 0x02000000;
                let mut bytes = [0; 2];
                bytes.copy_from_slice(&self.mem[addr..addr + 2]);
                u16::from_le_bytes(bytes)
            }
            0x04000004 => self.gpu2d_a.dispstat.value(),
            _ => {
                logger::warn(
                    logger::LogSource::Bus9,
                    format!("Invalid read halfword address: {:#010X}", addr),
                );
                0
            }
        }
    }

    fn read_word(&self, addr: u32) -> u32 {
        let addr = addr as usize;
        match addr {
            0x00000000..=0x00007FFF => {
                let mut bytes = [0; 4];
                bytes.copy_from_slice(&self.inst_tcm[addr..addr + 4]);
                u32::from_le_bytes(bytes)
            }
            0x00800000..=0x00803FFF => {
                let addr = addr - 0x00800000;
                let mut bytes = [0; 4];
                bytes.copy_from_slice(&self.data_tcm[addr..addr + 4]);
                u32::from_le_bytes(bytes)
            }
            0x02000000..=0x023FFFFF => {
                let addr = addr - 0x02000000;
                let mut bytes = [0; 4];
                bytes.copy_from_slice(&self.mem[addr..addr + 4]);
                u32::from_le_bytes(bytes)
            }
            0x04000000 => self.gpu2d_a.dispcnt.value(),
            0x04000304 => self.powcnt1.value(),
            _ => {
                logger::warn(
                    logger::LogSource::Bus9,
                    format!("Invalid read word address: {:#010X}", addr),
                );
                0
            }
        }
    }

    fn read_bulk(&self, addr: u32, len: u32) -> Vec<u8> {
        let addr = addr as usize;
        let len = len as usize;
        match addr {
            0x02000000..=0x023FFFFF => {
                let addr = addr - 0x02000000;
                self.mem[addr..addr + len].to_vec()
            }
            _ => {
                logger::warn(
                    logger::LogSource::Bus9,
                    format!(
                        "Invalid read bulk address: {:#010X}-{:#010X}",
                        addr,
                        addr + len
                    ),
                );
                vec![0; len]
            }
        }
    }

    fn write_byte(&mut self, addr: u32, value: u8) {
        let addr = addr as usize;
        match addr {
            0x02000000..=0x023FFFFF => {
                let addr = addr - 0x02000000;
                self.mem[addr] = value;
            }
            0x04000240..=0x04000249 => self.vramcnt[addr - 0x04000240] = value,
            _ => {
                logger::warn(
                    logger::LogSource::Bus9,
                    format!("Invalid write byte address: {:#010X}", addr),
                );
            }
        }
    }

    fn write_halfword(&mut self, addr: u32, value: u16) {
        let addr = addr as usize;
        match addr {
            0x02000000..=0x023FFFFF => {
                let addr = addr - 0x02000000;
                self.mem[addr..addr + 2].copy_from_slice(&value.to_le_bytes());
            }
            0x06800000..=0x0680A3FF => {
                let addr = addr - 0x06800000;
                self.gpu2d_a.vram_lcdc_alloc[addr..addr + 2].copy_from_slice(&value.to_le_bytes());
            }
            _ => {
                logger::warn(
                    logger::LogSource::Bus9,
                    format!("Invalid write halfword address: {:#010X}", addr),
                );
            }
        }
    }

    fn write_word(&mut self, addr: u32, value: u32) {
        let addr = addr as usize;
        match addr {
            0x00800000..=0x00803FFF => {
                let addr = addr - 0x00800000;
                self.data_tcm[addr..addr + 4].copy_from_slice(&value.to_le_bytes());
            }
            0x02000000..=0x023FFFFF => {
                let addr = addr - 0x02000000;
                self.mem[addr..addr + 4].copy_from_slice(&value.to_le_bytes());
            }
            0x04000000 => self.gpu2d_a.dispcnt = value.into(),
            0x04000304 => self.powcnt1 = value.into(),
            _ => {
                logger::warn(
                    logger::LogSource::Bus9,
                    format!("Invalid write word address: {:#010X}", addr),
                );
            }
        }
    }

    // it's 1am i don't know what to call this
    fn write_bulk(&mut self, addr: u32, data: Vec<u8>) {
        let addr = addr as usize;
        match addr {
            0x02000000..=0x023FFFFF => {
                let addr = addr - 0x02000000;
                self.mem[addr..addr + data.len()].copy_from_slice(&data);
            }
            _ => {
                logger::warn(
                    logger::LogSource::Bus9,
                    format!("Invalid write bulk address: {:#010X}", addr),
                );
            }
        }
    }
}

pub struct FakeBus;

impl BusTrait for FakeBus {
    fn read_byte(&mut self, _addr: u32) -> u8 {
        0
    }
    fn read_halfword(&mut self, _addr: u32) -> u16 {
        0
    }
    fn read_word(&self, _addr: u32) -> u32 {
        0
    }
    fn read_bulk(&self, _addr: u32, _len: u32) -> Vec<u8> {
        vec![]
    }

    fn write_byte(&mut self, _addr: u32, _value: u8) {}
    fn write_halfword(&mut self, _addr: u32, _value: u16) {}
    fn write_word(&mut self, _addr: u32, _value: u32) {}
    fn write_bulk(&mut self, _addr: u32, _data: Vec<u8>) {}
}
