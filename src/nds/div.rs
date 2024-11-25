use super::Bits;

#[derive(Default)]
pub struct DividerUnit {
    pub control: DivisionControl,

    pub numerator_lo: u32,
    pub numerator_hi: u32,
    pub denominator_lo: u32,
    pub denominator_hi: u32,

    pub result_lo: u32,
    pub result_hi: u32,
    pub remainder_lo: u32,
    pub remainder_hi: u32,

    running: bool,
    cycles: u32,
}

impl DividerUnit {
    pub fn set_control(&mut self, value: u32) {
        self.control.set(value);
        self.start();
    }

    // TODO: improve
    pub fn set_numerator<const LO: bool>(&mut self, value: u32) {
        if LO {
            self.numerator_lo = value;
        } else {
            self.numerator_hi = value;
        }

        self.start();
    }

    // TODO: improve
    pub fn set_denominator<const LO: bool>(&mut self, value: u32) {
        if LO {
            self.denominator_lo = value;
        } else {
            self.denominator_hi = value;
        }

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

    pub fn clock(&mut self, cycles: u32) {
        if !self.running {
            return;
        }

        self.cycles = self.cycles.saturating_sub(cycles);

        // TODO: this can probably be written a lot better
        // here's hoping the compiler optimises it for now (it won't)
        if self.cycles == 0 {
            let dividing_by_zero = self.denominator_lo == 0 && self.denominator_hi == 0;
            self.control.set_div_by_zero(dividing_by_zero);

            let (result, remainder) = match self.control.get_mode() {
                0 => {
                    if self.denominator_lo == 0 {
                        let merged = self.numerator_lo as i32 as i64
                            | (self.numerator_hi as i32 as i64) << 32;

                        let result = if self.numerator_lo as i32 >= 0 {
                            merge_lo_hi(-1_i32 as u32, 0)
                        } else {
                            merge_lo_hi(1, -1_i32 as u32)
                        } as u64;
                        let remainder =
                            merge_lo_hi(self.numerator_lo, (merged >> 32) as u32) as u64;

                        (result, remainder)
                    } else if self.numerator_lo == -0x8000_0000_i32 as u32
                        && self.denominator_lo == -1_i32 as u32
                    {
                        (0x8000_0000, 0)
                    } else {
                        (
                            (self.numerator_lo as i32).wrapping_div(self.denominator_lo as i32)
                                as u64,
                            (self.numerator_lo as i32).wrapping_rem(self.denominator_lo as i32)
                                as u64,
                        )
                    }
                }
                1 | 3 => {
                    let numerator = merge_lo_hi(self.numerator_lo, self.numerator_hi);
                    if self.denominator_lo == 0 {
                        let result = if numerator >= 0 { -1_i64 as u64 } else { 1 };
                        let remainder = numerator as u64;

                        (result, remainder)
                    } else if numerator == -0x8000_0000_0000_0000_i64
                        && self.denominator_lo == -1_i32 as u32
                    {
                        (0x8000_0000_0000_0000, 0)
                    } else {
                        (
                            numerator.wrapping_div(self.denominator_lo as i32 as i64) as u64,
                            numerator.wrapping_rem(self.denominator_lo as i32 as i64) as u64,
                        )
                    }
                }
                2 => {
                    let numerator = merge_lo_hi(self.numerator_lo, self.numerator_hi);
                    let denominator = merge_lo_hi(self.denominator_lo, self.denominator_hi);
                    if denominator == 0 {
                        let result = if numerator >= 0 { -1_i64 as u64 } else { 1 };
                        let remainder = numerator as u64;

                        (result, remainder)
                    } else if numerator == -0x8000_0000_0000_0000_i64 && denominator == -1_i64 {
                        (0x8000_0000_0000_0000, 0)
                    } else {
                        (
                            numerator.wrapping_div(denominator) as u64,
                            numerator.wrapping_rem(denominator) as u64,
                        )
                    }
                }
                _ => unreachable!(),
            };

            // logger::debug(
            //     logger::LogSource::Bus9,
            //     format_debug!(
            //         "{},{} / {},{} = {},{}",
            //         self.numerator_lo as i32,
            //         self.numerator_hi as i32,
            //         self.denominator_lo as i32,
            //         self.denominator_hi as i32,
            //         result as i64,
            //         remainder as i64
            //     ),
            // );

            (self.result_lo, self.result_hi) = split_lo_hi(result);
            (self.remainder_lo, self.remainder_hi) = split_lo_hi(remainder);

            // logger::debug(
            //     logger::LogSource::Bus9,
            //     format_debug!(
            //         "{},{} / {},{} = {},{} ({},{}) {}",
            //         self.numerator_lo as i32,
            //         self.numerator_hi as i32,
            //         self.denominator_lo as i32,
            //         self.denominator_hi as i32,
            //         self.result_lo as i32,
            //         self.result_hi as i32,
            //         self.remainder_lo as i32,
            //         self.remainder_hi as i32,
            //         self.control.get_div_by_zero()
            //     ),
            // );

            self.running = false;
            self.control.set_busy(false);
        }
    }
}

#[inline(always)]
fn merge_lo_hi(lo: u32, hi: u32) -> i64 {
    ((hi as u64) << 32 | lo as u64) as i64
}

#[inline(always)]
fn split_lo_hi(value: u64) -> (u32, u32) {
    (value as u32, (value >> 32) as u32)
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

    fn set_div_by_zero(&mut self, value: bool) {
        self.0.set_bit(Self::DIV_BY_ZERO_OFFSET, value);
    }

    fn set_busy(&mut self, value: bool) {
        self.0.set_bit(Self::BUSY_OFFSET, value);
    }
}
