use crate::nds::{gpu::gpu2d::Gpu2d, logger};

pub struct Bus {
    pub mem: Vec<u8>,
    pub gpu2d_a: Gpu2d,
}

impl Default for Bus {
    fn default() -> Self {
        Bus {
            mem: vec![0; 1024 * 1024 * 4],
            gpu2d_a: Gpu2d::default(),
        }
    }
}

impl Bus {
    pub fn read_word(&self, addr: u32) -> u32 {
        let addr = addr as usize;
        match addr {
            0x02000000..=0x023FFFFF => {
                let mut bytes = [0; 4];
                bytes.copy_from_slice(&self.mem[addr..addr + 4]);
                u32::from_le_bytes(bytes)
            }
            0x04000000 => self.gpu2d_a.dispcnt.value(),
            _ => {
                logger::error(format!("Invalid read address: {:#010X}", addr));
                0
            }
        }
    }

    pub fn write_word(&mut self, addr: u32, value: u32) {
        let addr = addr as usize;
        match addr {
            0x02000000..=0x023FFFFF => {
                self.mem[addr..addr + 4].copy_from_slice(&value.to_le_bytes());
            }
            0x04000000 => self.gpu2d_a.dispcnt = value.into(),
            _ => {
                logger::error(format!("Invalid write address: {:#010X}", addr));
            }
        }
    }
}
