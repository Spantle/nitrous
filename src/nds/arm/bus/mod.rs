use crate::nds::shared::Shared;

use super::ArmKind;

pub trait BusTrait {
    fn kind() -> ArmKind;

    fn load_bios(&mut self, bios: Vec<u8>);
    fn load_bios_from_path(&mut self, path: &str);

    fn read_byte(&self, shared: &mut Shared, addr: u32) -> u8;
    fn read_halfword(&self, shared: &mut Shared, addr: u32) -> u16;
    fn read_word(&self, shared: &mut Shared, addr: u32) -> u32;
    fn read_bulk(&self, shared: &mut Shared, addr: u32, len: u32) -> Vec<u8> {
        let mut bytes = vec![];
        for i in 0..len {
            bytes.push(self.read_byte(shared, addr + i));
        }
        bytes
    }
    fn read_slice<const T: usize>(&self, shared: &mut Shared, addr: u32) -> [u8; T];

    fn write_byte(&mut self, shared: &mut Shared, addr: u32, value: u8);
    fn write_halfword(&mut self, shared: &mut Shared, addr: u32, value: u16);
    fn write_word(&mut self, shared: &mut Shared, addr: u32, value: u32);
    fn write_bulk(&mut self, shared: &mut Shared, addr: u32, data: Vec<u8>) {
        (0..data.len()).for_each(|i| {
            self.write_byte(shared, addr + i as u32, data[i]);
        });
    }
    fn write_slice<const T: usize>(&mut self, shared: &mut Shared, addr: u32, value: [u8; T]);
}

#[derive(Default)]
pub struct FakeBus;
impl BusTrait for FakeBus {
    fn kind() -> ArmKind {
        ArmKind::ARM9
    }

    fn load_bios(&mut self, _bios: Vec<u8>) {}
    fn load_bios_from_path(&mut self, _path: &str) {}

    fn read_byte(&self, _shared: &mut Shared, _addr: u32) -> u8 {
        0
    }
    fn read_halfword(&self, _shared: &mut Shared, _addr: u32) -> u16 {
        0
    }
    fn read_word(&self, _shared: &mut Shared, _addr: u32) -> u32 {
        0
    }
    fn read_bulk(&self, _shared: &mut Shared, _addr: u32, _len: u32) -> Vec<u8> {
        vec![]
    }
    fn read_slice<const T: usize>(&self, _shared: &mut Shared, _addr: u32) -> [u8; T] {
        [0; T]
    }

    fn write_byte(&mut self, _shared: &mut Shared, _addr: u32, _value: u8) {}
    fn write_halfword(&mut self, _shared: &mut Shared, _addr: u32, _value: u16) {}
    fn write_word(&mut self, _shared: &mut Shared, _addr: u32, _value: u32) {}
    fn write_bulk(&mut self, _shared: &mut Shared, _addr: u32, _data: Vec<u8>) {}
    fn write_slice<const T: usize>(&mut self, _shared: &mut Shared, _addr: u32, _value: [u8; T]) {}
}

pub mod bus7;
pub mod bus9;
