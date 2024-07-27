#![allow(dead_code)]

use crate::nds::Bits;

#[derive(Default)]
#[allow(clippy::upper_case_acronyms)]
pub struct IPCSYNC {
    value: u32,

    pub log: Vec<IpcsyncLog>,
    pub logging_enabled: bool,
}

impl From<u32> for IPCSYNC {
    fn from(value: u32) -> Self {
        Self {
            value,
            log: Vec::new(),
            logging_enabled: false,
        }
    }
}

impl IPCSYNC {
    pub fn value_quiet(&self) -> u32 {
        self.value
    }

    pub fn value(&mut self, is_arm9: bool) -> u32 {
        if self.logging_enabled {
            self.log.push(IpcsyncLog::Read(is_arm9, self.value));
        }

        self.value
    }

    pub fn set(&mut self, is_arm9: bool, value: u32) {
        if self.logging_enabled {
            self.log.push(IpcsyncLog::Write(is_arm9, value));
        }

        self.value = value;
        let input = self.value.get_bits(8, 11);
        self.value.set_bits(0, 3, input);
    }
}

pub enum IpcsyncLog {
    // is_arm9, value
    Read(bool, u32),
    Write(bool, u32),
}
