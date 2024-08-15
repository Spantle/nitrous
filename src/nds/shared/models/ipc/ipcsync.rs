#![allow(dead_code)]

use crate::nds::Bits;

// I could've made this a generic, but this actually seems nicer (considering the logging + too many generics for something this simple)
#[derive(Default)]
#[allow(clippy::upper_case_acronyms)]
pub struct IPCSYNC {
    value9: u32,
    value7: u32,

    pub log: Vec<IpcsyncLog>,
    pub logging_enabled: bool,
}

impl IPCSYNC {
    pub fn value_quiet<const ARM_BOOL: bool>(&self) -> u32 {
        if ARM_BOOL {
            self.value9
        } else {
            self.value7
        }
    }

    pub fn value<const ARM_BOOL: bool>(&mut self) -> u32 {
        let value = self.value_quiet::<ARM_BOOL>();
        if self.logging_enabled {
            self.log.push(IpcsyncLog::Read(ARM_BOOL, value));
        }

        value
    }

    pub fn set<const ARM_BOOL: bool>(&mut self, value: u32) {
        if self.logging_enabled {
            self.log.push(IpcsyncLog::Write(ARM_BOOL, value));
        }

        let etc = value.get_bits(4, 31);
        let input = value.get_bits(8, 11);
        match ARM_BOOL {
            true => {
                self.value9.set_bits(4, 31, etc);
                self.value7.set_bits(0, 3, input);
            }
            false => {
                self.value7.set_bits(4, 31, etc);
                self.value9.set_bits(0, 3, input);
            }
        };
    }
}

pub enum IpcsyncLog {
    // is_arm9, value
    Read(bool, u32),
    Write(bool, u32),
}
