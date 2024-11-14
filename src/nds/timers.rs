use super::{interrupts::Interrupts, logger, Bits};

pub struct Timers {
    timers: [Timer; 4],
}

impl Default for Timers {
    fn default() -> Self {
        Self {
            timers: [Timer::new(0), Timer::new(1), Timer::new(2), Timer::new(3)],
        }
    }
}

impl Timers {
    pub fn get(&self, index: usize) -> &Timer {
        &self.timers[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Timer {
        &mut self.timers[index]
    }

    pub fn clock(&mut self, interrupts: &mut Interrupts) {
        self.timers[0].clock(interrupts);
        self.timers[1].clock(interrupts);
        self.timers[2].clock(interrupts);
        self.timers[3].clock(interrupts);
    }
}

pub struct Timer {
    index: u8,

    control: TmCnt,
    reload: u16,
    counter: u16,

    next_run: i16,
    next_run_prescaler: i16,
}

impl Timer {
    pub fn new(index: u8) -> Self {
        Self {
            index,

            control: TmCnt::default(),
            reload: 0,
            counter: 0,

            next_run: 1,
            next_run_prescaler: 1,
        }
    }

    // TODO: use in buses in future
    // fn value(&self) -> u32 {
    //     self.counter as u32 | ((self.control.0 as u32) << 16)
    // }

    pub fn get_counter(&self) -> u16 {
        self.counter
    }

    pub fn get_control(&self) -> u16 {
        self.control.0
    }

    pub fn set(&mut self, value: u32) {
        let old_prescaler = self.control.get_prescaler();
        let old_enable = self.control.get_timer_operating();

        self.reload = value as u16;
        self.control.set(value.get_bits(16, 31) as u16);

        self.cnt_updated(old_prescaler, old_enable);
    }

    pub fn set_l(&mut self, value: u16) {
        self.reload = value;
    }

    pub fn set_h(&mut self, value: u16) {
        let old_prescaler = self.control.get_prescaler();
        let old_enable = self.control.get_timer_operating();

        self.control.set(value);

        self.cnt_updated(old_prescaler, old_enable);
    }

    pub fn clock(&mut self, interrupts: &mut Interrupts) {
        let is_operating = self.control.get_timer_operating();
        let is_normal_mode = !self.control.get_count_up_timing();
        self.next_run -= (is_operating & is_normal_mode) as i16;
        // TODO: i could eliminate this if statement perhaps
        if self.next_run <= 0 {
            self.next_run = self.next_run_prescaler;

            // TODO: can i remove this if statement?
            // TODO: "6" is a temporary hack to make timers run at the correct speed. THIS NEEDS TO BE SET BACK TO "1" IN THE FUTURE
            let (new_counter, overflow) = self.counter.overflowing_add(6);
            self.counter = new_counter;
            if overflow {
                self.counter = self.reload;
            }

            interrupts.f.falsy_set_timer_overflow(
                self.index,
                self.control.get_overflow_irq_enable() & overflow,
            );
        }
    }

    fn cnt_updated(&mut self, old_prescaler: u16, old_enable: bool) {
        if self.control.get_count_up_timing() {
            // TODO: implement count up timing
            logger::error_once(logger::LogSource::Emu, "Count up timing not implemented");
        }

        if !old_enable && self.control.get_timer_operating() {
            self.counter = self.reload;
        }

        let new_prescaler = self.control.get_prescaler();
        if new_prescaler != old_prescaler {
            // TODO: investigate what actually happens to the next_run counter when the prescaler changes
            self.next_run_prescaler = match new_prescaler {
                0 => 1,
                1 => 64,
                2 => 256,
                3 => 1024,
                _ => unreachable!(),
            };
            self.next_run = self.next_run_prescaler;
        }
    }
}

#[derive(Default)]
struct TmCnt(u16);

impl TmCnt {
    const PRESCALER_START: u16 = 0;
    const PRESCALER_END: u16 = 1;
    const COUNT_UP_TIMING_OFFSET: u16 = 2;

    const OVERFLOW_IRQ_ENABLE_OFFSET: u16 = 6;
    const TIMER_OPERATING_OFFSET: u16 = 7;

    pub fn set(&mut self, value: u16) {
        self.0 = value;
    }

    pub fn get_prescaler(&self) -> u16 {
        self.0.get_bits(Self::PRESCALER_START, Self::PRESCALER_END)
    }

    pub fn get_count_up_timing(&self) -> bool {
        self.0.get_bit(Self::COUNT_UP_TIMING_OFFSET)
    }

    pub fn get_overflow_irq_enable(&self) -> bool {
        self.0.get_bit(Self::OVERFLOW_IRQ_ENABLE_OFFSET)
    }

    pub fn get_timer_operating(&self) -> bool {
        self.0.get_bit(Self::TIMER_OPERATING_OFFSET)
    }
}
