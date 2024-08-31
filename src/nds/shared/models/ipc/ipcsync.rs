#![allow(dead_code)]

use crate::nds::{interrupts::Interrupts, Bits};

// I could've made this a generic, but this actually seems nicer (considering the logging + too many generics for something this simple)
#[derive(Default)]
#[allow(clippy::upper_case_acronyms)]
pub struct IPCSYNC {
    value9: u32,
    value7: u32,

    pub log: Vec<IpcsyncLog>,
    pub logging_enabled: bool,

    send_irq9to7: bool,
    send_irq7to9: bool,
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
        let send_irq = value.get_bit(13);
        let receive_irq_enable = value.get_bit(14);
        match ARM_BOOL {
            true => {
                self.value9.set_bits(4, 31, etc);
                self.value7.set_bits(0, 3, input);

                self.send_irq9to7 = send_irq;
                self.value9.set_bit(14, receive_irq_enable);
            }
            false => {
                self.value7.set_bits(4, 31, etc);
                self.value9.set_bits(0, 3, input);

                self.send_irq7to9 = send_irq;
                self.value7.set_bit(14, receive_irq_enable);
            }
        };
    }

    pub fn update_interrupts(
        &mut self,
        interrupts9: &mut Interrupts,
        interrupts7: &mut Interrupts,
    ) {
        interrupts9
            .f
            .set_ipcsync(self.value9.get_bit(14) && self.send_irq7to9);
        interrupts7
            .f
            .set_ipcsync(self.value7.get_bit(14) && self.send_irq9to7);

        self.send_irq9to7 = false;
        self.send_irq7to9 = false;
    }
}

pub enum IpcsyncLog {
    // is_arm9, value
    Read(bool, u32),
    Write(bool, u32),
}
