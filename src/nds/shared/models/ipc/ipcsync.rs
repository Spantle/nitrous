use crate::nds::{interrupts::Interrupts, Bits, Bytes};

// I could've made this a generic, but this actually seems nicer (considering the logging + too many generics for something this simple)
#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct IpcSync {
    value9: u32,
    value7: u32,

    pub log: Vec<IpcsyncLog>,
    pub logging_enabled: bool,

    send_irq9to7: bool,
    send_irq7to9: bool,
}

impl IpcSync {
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

    pub fn set<const ARM_BOOL: bool, const T: usize>(&mut self, base_addr: usize, value: [u8; T]) {
        if self.logging_enabled {
            self.log
                .push(IpcsyncLog::Write(ARM_BOOL, value.into_word()));
        }

        // thanks libnds for making me commit this crime
        (0..T).for_each(|i| {
            let addr = base_addr + i;
            match addr {
                0 => {
                    let etc = value[i].get_bits(4, 7) as u32;
                    match ARM_BOOL {
                        true => self.value9.set_bits(4, 7, etc),
                        false => self.value7.set_bits(4, 7, etc),
                    }
                }
                1 => {
                    let etc = value[i] as u32; // 8-15
                    let input = value[i].get_bits(0, 3) as u32; // 8-11
                    let send_irq = value[i].get_bit(5); // 13
                    let receive_irq_enable = value[i].get_bit(6); // 14
                    match ARM_BOOL {
                        true => {
                            self.value9.set_bits(8, 15, etc);
                            self.value7.set_bits(0, 3, input);

                            self.send_irq9to7 = send_irq;
                            self.value9.set_bit(14, receive_irq_enable);
                        }
                        false => {
                            self.value7.set_bits(8, 15, etc);
                            self.value9.set_bits(0, 3, input);

                            self.send_irq7to9 = send_irq;
                            self.value7.set_bit(14, receive_irq_enable);
                        }
                    }
                }
                2 => {
                    let etc = value[i] as u32; // 16-23
                    match ARM_BOOL {
                        true => self.value9.set_bits(16, 23, etc),
                        false => self.value7.set_bits(16, 23, etc),
                    }
                }
                3 => {
                    let etc = value[i] as u32; // 24-31
                    match ARM_BOOL {
                        true => self.value9.set_bits(24, 31, etc),
                        false => self.value7.set_bits(24, 31, etc),
                    }
                }
                _ => unreachable!(),
            }
        });
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

#[derive(serde::Deserialize, serde::Serialize)]
pub enum IpcsyncLog {
    // is_arm9, value
    Read(bool, u32),
    Write(bool, u32),
}
