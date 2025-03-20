use super::Bits;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct SquareRootUnit {
    pub control: SquareRootControl,

    pub param_lo: u32,
    pub param_hi: u32,

    pub result: u32,

    running: bool,
}

impl SquareRootUnit {
    pub fn set_control(&mut self, value: u32) {
        self.control.set(value);
        self.start();
    }

    // TODO: improve
    pub fn set_param<const LO: bool>(&mut self, value: u32) {
        if LO {
            self.param_lo = value;
        } else {
            self.param_hi = value;
        }

        self.start();
    }

    fn start(&mut self) {
        self.running = true;
        self.control.set_busy(true);
    }

    pub fn clock(&mut self) {
        if !self.running {
            return;
        }

        if self.control.get_mode() {
            // 64-bit input
            self.result = merge_lo_hi(self.param_lo, self.param_hi).isqrt() as u32
        } else {
            // 32-bit input
            self.result = self.param_lo.isqrt()
        }

        self.running = false;
        self.control.set_busy(false);
    }
}

#[inline(always)]
fn merge_lo_hi(lo: u32, hi: u32) -> u64 {
    ((hi as u64) << 32) | lo as u64
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct SquareRootControl(u32);

impl SquareRootControl {
    const MODE_OFFSET: u32 = 0;

    const BUSY_OFFSET: u32 = 15;

    pub fn value(&self) -> u32 {
        self.0
    }

    fn set(&mut self, value: u32) {
        self.0 = value;
    }

    fn get_mode(&self) -> bool {
        self.0.get_bit(Self::MODE_OFFSET)
    }

    fn set_busy(&mut self, value: bool) {
        self.0.set_bit(Self::BUSY_OFFSET, value);
    }
}
