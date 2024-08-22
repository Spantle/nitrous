use models::{IPCSYNC, KEYINPUT, POWCNT1};

use super::{
    arm::bus::{bus7::Bus7, bus9::Bus9},
    cartridge::Cartridge,
    dma::DMA,
    gpu::gpu2d::Gpu2d,
};

pub struct Shared {
    pub cart: Cartridge,
    pub gpu2d_a: Gpu2d,
    pub gpu2d_b: Gpu2d,
    pub psram: Vec<u8>,
    pub wram: Vec<u8>, // 32kb

    pub keyinput: KEYINPUT, // 0x04000130
    pub vramcnt: [u8; 10],  // 0x04000240 - 0x04000249, 0x04000247 is wramcnt
    pub ipcsync: IPCSYNC,   // 0x04000180
    pub powcnt1: POWCNT1,   // 0x04000304

    pub dma9: DMA<Bus9>,
    pub dma7: DMA<Bus7>,

    pub vram_lcdc_alloc: Vec<u8>, // 0x06800000
}

impl Default for Shared {
    fn default() -> Self {
        Self {
            cart: Cartridge::default(),
            gpu2d_a: Gpu2d::default(),
            gpu2d_b: Gpu2d::default(),
            psram: vec![0; 1024 * 1024 * 4],
            wram: vec![0; 1024 * 32],

            keyinput: KEYINPUT::default(),
            vramcnt: [0; 10],
            ipcsync: IPCSYNC::default(),
            powcnt1: POWCNT1::default(),

            dma9: DMA::default(),
            dma7: DMA::default(),

            vram_lcdc_alloc: vec![0; 1024 * 656],
        }
    }
}

impl Shared {
    pub fn new_fake() -> Self {
        Self {
            cart: Cartridge::default(),
            gpu2d_a: Gpu2d::default(),
            gpu2d_b: Gpu2d::default(),
            psram: vec![0; 0],
            wram: vec![0; 0],

            keyinput: KEYINPUT::default(),
            vramcnt: [0; 10],
            ipcsync: IPCSYNC::default(),
            powcnt1: POWCNT1::default(),

            dma9: DMA::default(),
            dma7: DMA::default(),

            vram_lcdc_alloc: vec![0; 0],
        }
    }

    pub fn reset(&mut self) {
        self.gpu2d_a = Gpu2d::default();
        self.psram = vec![0; 1024 * 1024 * 4];
        self.wram = vec![0; 1024 * 32];

        self.vramcnt = [0; 10];
        self.ipcsync = IPCSYNC::default();
        self.powcnt1 = POWCNT1::default();

        self.vram_lcdc_alloc = vec![0; 1024 * 656];
    }
}

pub mod models;
