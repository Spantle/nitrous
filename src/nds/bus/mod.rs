#![allow(dead_code)]

use crate::nds::shared::Shared;

use super::{arm::ArmKind, dma::Dma, interrupts::Interrupts};

pub trait BusTrait {
    const KIND: ArmKind;

    fn reset(&mut self);
    fn load_state(&mut self, bus: Self);

    fn load_bios(&mut self, bios: Vec<u8>);
    fn load_bios_from_path(&mut self, path: &str);
    fn load_firmware(&mut self, firmware: Vec<u8>);
    fn load_firmware_from_path(&mut self, path: &str);

    fn is_requesting_interrupt(&self) -> bool;
    fn get_interrupts(&mut self) -> &mut Interrupts;

    fn read_byte(&self, shared: &mut Shared, dma: &mut Option<&mut Dma>, addr: u32) -> u8;
    fn read_halfword(&self, shared: &mut Shared, dma: &mut Option<&mut Dma>, addr: u32) -> u16;
    fn read_word(&self, shared: &mut Shared, dma: &mut Option<&mut Dma>, addr: u32) -> u32;
    fn read_bulk(
        &self,
        shared: &mut Shared,
        dma: &mut Option<&mut Dma>,
        addr: u32,
        len: u32,
    ) -> Vec<u8> {
        let mut bytes = vec![];
        for i in 0..len {
            bytes.push(self.read_byte(shared, dma, addr + i));
        }
        bytes
    }
    fn read_slice<const T: usize>(
        &self,
        shared: &mut Shared,
        dma: &mut Option<&mut Dma>,
        addr: u32,
    ) -> [u8; T];

    fn write_byte(&mut self, shared: &mut Shared, dma: &mut Option<&mut Dma>, addr: u32, value: u8);
    fn write_halfword(
        &mut self,
        shared: &mut Shared,
        dma: &mut Option<&mut Dma>,
        addr: u32,
        value: u16,
    );
    fn write_word(
        &mut self,
        shared: &mut Shared,
        dma: &mut Option<&mut Dma>,
        addr: u32,
        value: u32,
    );
    fn write_bulk(
        &mut self,
        shared: &mut Shared,
        dma: &mut Option<&mut Dma>,
        addr: u32,
        data: Vec<u8>,
    ) {
        (0..data.len()).for_each(|i| {
            self.write_byte(shared, dma, addr + i as u32, data[i]);
        });
    }
    fn write_slice<const T: usize>(
        &mut self,
        shared: &mut Shared,
        dma: &mut Option<&mut Dma>,
        addr: u32,
        value: [u8; T],
    );
}

#[derive(Default)]
pub struct FakeBus {
    interrupts: Interrupts,
}
impl BusTrait for FakeBus {
    const KIND: ArmKind = ArmKind::Arm9;

    fn reset(&mut self) {}
    fn load_state(&mut self, _bus: Self) {}

    fn load_bios(&mut self, _bios: Vec<u8>) {}
    fn load_bios_from_path(&mut self, _path: &str) {}
    fn load_firmware(&mut self, _firmware: Vec<u8>) {}
    fn load_firmware_from_path(&mut self, _path: &str) {}

    fn is_requesting_interrupt(&self) -> bool {
        false
    }
    fn get_interrupts(&mut self) -> &mut Interrupts {
        &mut self.interrupts
    }

    fn read_byte(&self, _shared: &mut Shared, _dma: &mut Option<&mut Dma>, _addr: u32) -> u8 {
        0
    }
    fn read_halfword(&self, _shared: &mut Shared, _dma: &mut Option<&mut Dma>, _addr: u32) -> u16 {
        0
    }
    fn read_word(&self, _shared: &mut Shared, _dma: &mut Option<&mut Dma>, _addr: u32) -> u32 {
        0
    }
    fn read_bulk(
        &self,
        _shared: &mut Shared,
        _dma: &mut Option<&mut Dma>,
        _addr: u32,
        _len: u32,
    ) -> Vec<u8> {
        vec![]
    }
    fn read_slice<const T: usize>(
        &self,
        _shared: &mut Shared,
        _dma: &mut Option<&mut Dma>,
        _addr: u32,
    ) -> [u8; T] {
        [0; T]
    }

    fn write_byte(
        &mut self,
        _shared: &mut Shared,
        _dma: &mut Option<&mut Dma>,
        _addr: u32,
        _value: u8,
    ) {
    }
    fn write_halfword(
        &mut self,
        _shared: &mut Shared,
        _dma: &mut Option<&mut Dma>,
        _addr: u32,
        _value: u16,
    ) {
    }
    fn write_word(
        &mut self,
        _shared: &mut Shared,
        _dma: &mut Option<&mut Dma>,
        _addr: u32,
        _value: u32,
    ) {
    }
    fn write_bulk(
        &mut self,
        _shared: &mut Shared,
        _dma: &mut Option<&mut Dma>,
        _addr: u32,
        _data: Vec<u8>,
    ) {
    }
    fn write_slice<const T: usize>(
        &mut self,
        _shared: &mut Shared,
        _dma: &mut Option<&mut Dma>,
        _addr: u32,
        _value: [u8; T],
    ) {
    }
}

pub mod bus7;
pub mod bus9;
