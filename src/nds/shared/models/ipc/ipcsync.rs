#![allow(dead_code)]

use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::nds::Bits;

#[derive(Default)]
#[allow(clippy::upper_case_acronyms)]
pub struct IPCSYNC(u32);

impl From<u32> for IPCSYNC {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl IPCSYNC {
    pub fn value_quiet(&self) -> u32 {
        self.0
    }

    pub fn value(&self, is_arm9: bool) -> u32 {
        let mut logs = IPCSYNC_LOG.lock().unwrap();
        logs.push(IpcsyncLog::Read(is_arm9, self.0));

        self.0
    }

    pub fn set(&mut self, is_arm9: bool, value: u32) {
        let mut logs = IPCSYNC_LOG.lock().unwrap();
        logs.push(IpcsyncLog::Write(is_arm9, value));

        self.0 = value;
        let input = self.0.get_bits(8, 11);
        self.0.set_bits(0, 3, input);
    }
}

pub enum IpcsyncLog {
    // is_arm9, value
    Read(bool, u32),
    Write(bool, u32),
}

pub static IPCSYNC_LOG: Lazy<Mutex<Vec<IpcsyncLog>>> = Lazy::new(|| Mutex::new(Vec::new()));
