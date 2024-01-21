use crate::nds::logger;

pub struct Bus {
    pub mem: Vec<u8>,
}

impl Default for Bus {
    fn default() -> Self {
        Bus {
            mem: vec![0; 1024 * 1024 * 4],
        }
    }
}

impl Bus {
    pub fn read_word(&self, addr: u32) -> u32 {
        let addr = addr as usize;
        let mut bytes = [0; 4];
        match addr {
            0x00000000..=0x00400000 => {
                bytes.copy_from_slice(&self.mem[addr..addr + 4]);
            }
            _ => {
                logger::error(format!("Invalid read address: {:#010X}", addr));
            }
        }

        u32::from_le_bytes(bytes)
    }
}
