use super::Bits;

#[derive(Default)]
pub struct DividerUnit {
    pub control: DivisionControl,

    pub numerator: u64,
    pub denominator: u64,

    pub result: u64,
    pub remainder: u64,

    running: bool,
    cycles: u8,
}

impl DividerUnit {
    pub fn set_control(&mut self, value: u32) {
        self.control.set(value);
        self.start();
    }

    // TODO: improve
    pub fn set_numerator<const LO: bool>(&mut self, value: u32) {
        if LO {
            self.numerator = (self.numerator & 0xFFFFFFFF00000000) | value as u64;
        } else {
            self.numerator = (self.numerator & 0x00000000FFFFFFFF) | ((value as u64) << 32);
        }
        self.start();
    }

    // TODO: improve
    pub fn set_denominator<const LO: bool>(&mut self, value: u32) {
        if LO {
            self.denominator = (self.denominator & 0xFFFFFFFF00000000) | value as u64;
        } else {
            self.denominator = (self.denominator & 0x00000000FFFFFFFF) | ((value as u64) << 32);
        }
        self.control.set_div_by_zero(self.denominator == 0);
        self.start();
    }

    fn start(&mut self) {
        self.cycles = match self.control.get_mode() {
            0 => 18,
            1 => 34,
            2 => 34,
            3 => 34,
            _ => unreachable!(),
        };
        self.running = true;
        self.control.set_busy(true);
    }

    pub fn clock(&mut self) {
        if !self.running {
            return;
        }

        self.cycles = self.cycles.saturating_sub(1);

        if self.cycles == 0 {
            if self.control.get_div_by_zero() {
                self.remainder = self.numerator;
                if self.numerator as i64 >= 0 {
                    self.result = -1_i64 as u64;
                } else {
                    self.result = 1;
                }
            } else if (self.numerator as i32) == -0x80000000 && self.denominator == -1_i64 as u64 {
                self.result = -0x80000000_i64 as u64;
            } else {
                println!(
                    "numerator: {}, denominator: {}",
                    self.numerator, self.denominator
                );
                (self.result, self.remainder) = match self.control.get_mode() {
                    0 => (
                        (self.numerator as i32).wrapping_div(self.denominator as i32) as u64,
                        (self.numerator as i32).wrapping_rem(self.denominator as i32) as u64,
                    ),
                    1 | 3 => (
                        self.numerator.wrapping_div(self.denominator as u32 as u64),
                        self.numerator.wrapping_rem(self.denominator as u32 as u64) as u32 as u64,
                    ),
                    2 => (
                        self.numerator.wrapping_div(self.denominator),
                        self.numerator.wrapping_rem(self.denominator),
                    ),
                    _ => unreachable!(),
                }
            }

            self.running = false;
            self.control.set_busy(false);
        }
    }
}

#[derive(Default)]
pub struct DivisionControl(u32);

impl DivisionControl {
    const MODE_START: u32 = 0;
    const MODE_END: u32 = 1;

    const DIV_BY_ZERO_OFFSET: u32 = 14;
    const BUSY_OFFSET: u32 = 15;

    pub fn value(&self) -> u32 {
        self.0
    }

    fn set(&mut self, value: u32) {
        self.0 = value;
    }

    fn get_mode(&self) -> u32 {
        self.0.get_bits(Self::MODE_START, Self::MODE_END)
    }

    fn get_div_by_zero(&self) -> bool {
        self.0.get_bit(Self::DIV_BY_ZERO_OFFSET)
    }

    fn set_div_by_zero(&mut self, value: bool) {
        self.0.set_bit(Self::DIV_BY_ZERO_OFFSET, value);
    }

    fn get_busy(&self) -> bool {
        self.0.get_bit(Self::BUSY_OFFSET)
    }

    fn set_busy(&mut self, value: bool) {
        self.0.set_bit(Self::BUSY_OFFSET, value);
    }
}
