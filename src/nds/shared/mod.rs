use models::{KEYINPUT, POWCNT1};

use super::{cartridge::Cartridge, gpu::gpu2d::Gpu2d};

pub struct Shared {
    pub cart: Cartridge,
    pub gpu2d_a: Gpu2d,
    pub psram: Vec<u8>,
    pub wram: Vec<u8>, // 32kb

    pub keyinput: KEYINPUT, // 0x04000130
    pub vramcnt: [u8; 10],  // 0x04000240 - 0x04000249, 0x04000247 is wramcnt
    pub powcnt1: POWCNT1,   // 0x04000304
}

impl Default for Shared {
    fn default() -> Self {
        Self {
            cart: Cartridge::default(),
            gpu2d_a: Gpu2d::default(),
            psram: vec![0; 1024 * 1024 * 4],
            wram: vec![0; 1024 * 32],

            keyinput: KEYINPUT::default(),
            vramcnt: [0; 10],
            powcnt1: POWCNT1::default(),
        }
    }
}

impl Shared {
    pub fn new_fake() -> Self {
        Self {
            cart: Cartridge::default(),
            gpu2d_a: Gpu2d::default(),
            psram: vec![0; 0],
            wram: vec![0; 0],

            keyinput: KEYINPUT::default(),
            vramcnt: [0; 10],
            powcnt1: POWCNT1::default(),
        }
    }
}

mod models;
