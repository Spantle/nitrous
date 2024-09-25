mod gpu2d;
mod models;

use gpu2d::Gpu2d;
use models::DispStat;

use crate::nds::bus::{bus7::Bus7, bus9::Bus9};

#[derive(Default)]
pub struct Gpus {
    pub dispstat: DispStat, // 0x04000004
    pub vcount: u16,        // 0x04000006

    pub a: Gpu2d,
    pub b: Gpu2d,

    x: u32, // TODO: this isn't real. ideally the clock function should just do an entire row at a time but I cannot be bothered touching cycle stuff rn. performance will suffer.
}

impl Gpus {
    pub fn clock(&mut self, bus9: &mut Bus9, bus7: &mut Bus7) {
        self.x = (self.x + 1) % (256 + 99);
        self.vcount = (self.vcount + (self.x == 0) as u16) % (192 + 71);

        let hblanking = self.x >= 256;
        let hblank_start = self.x == 256;
        let vblanking = self.vcount >= 192 && self.vcount != 262;
        let vblank_start = self.vcount == 192;
        self.dispstat.set_hblank_flag(hblanking);
        self.dispstat.set_vblank_flag(vblanking);

        if hblank_start && self.dispstat.get_hblank_irq_enable() {
            bus9.interrupts.f.set_lcd_hblank(true);
            bus7.interrupts.f.set_lcd_hblank(true);
        }
        if vblank_start && self.dispstat.get_vblank_irq_enable() {
            bus9.interrupts.f.set_lcd_vblank(true);
            bus7.interrupts.f.set_lcd_vblank(true);
        }

        let vcount_setting = self.dispstat.get_vcount_setting();
        let is_vcounter_match = vcount_setting == self.vcount;
        self.dispstat.set_vcounter_flag(is_vcounter_match);
        if is_vcounter_match && self.dispstat.get_vcounter_irq_enable() {
            bus9.interrupts.f.set_lcd_vcounter_match(true);
            bus7.interrupts.f.set_lcd_vcounter_match(true);
        }
    }
}
