// NOTE: do NOT use these for generics
// i don't know why but it ruins the generic magic

pub trait Bits<T> {
    fn get_bit(&self, offset: T) -> bool;
    fn get_bits(&self, offset: T, end: T) -> T;
    fn set_bit(&mut self, offset: T, value: bool);
    fn set_bits(&mut self, offset: T, end: T, value: T);
}

impl Bits<u32> for u32 {
    #[inline(always)]
    fn get_bit(&self, offset: u32) -> bool {
        (self >> offset) & 1 == 1
    }

    #[inline(always)]
    fn set_bit(&mut self, offset: u32, value: bool) {
        *self = (*self & !(1 << offset)) | ((value as u32) << offset);
    }

    #[inline(always)]
    fn get_bits(&self, offset: u32, end: u32) -> u32 {
        (self >> offset) & ((1 << (end - offset + 1)) - 1)
    }

    #[inline(always)]
    fn set_bits(&mut self, offset: u32, end: u32, value: u32) {
        let len = end - offset + 1;
        *self = (*self & !((1 << len) - 1)) | (value << offset);
    }
}

impl Bits<u16> for u16 {
    #[inline(always)]
    fn get_bit(&self, offset: u16) -> bool {
        (self >> offset) & 1 == 1
    }

    #[inline(always)]
    fn set_bit(&mut self, offset: u16, value: bool) {
        *self = (*self & !(1 << offset)) | ((value as u16) << offset);
    }

    #[inline(always)]
    fn get_bits(&self, offset: u16, end: u16) -> u16 {
        (self >> offset) & ((1 << (end - offset + 1)) - 1)
    }

    #[inline(always)]
    fn set_bits(&mut self, offset: u16, end: u16, value: u16) {
        let len = end - offset + 1;
        *self = (*self & !((1 << len) - 1)) | (value << offset);
    }
}

impl Bits<u8> for u8 {
    #[inline(always)]
    fn get_bit(&self, offset: u8) -> bool {
        (self >> offset) & 1 == 1
    }

    #[inline(always)]
    fn set_bit(&mut self, offset: u8, value: bool) {
        *self = (*self & !(1 << offset)) | ((value as u8) << offset);
    }

    #[inline(always)]
    fn get_bits(&self, offset: u8, end: u8) -> u8 {
        (self >> offset) & ((1 << (end - offset + 1)) - 1)
    }

    #[inline(always)]
    fn set_bits(&mut self, offset: u8, end: u8, value: u8) {
        let len = end - offset + 1;
        *self = (*self & !((1 << len) - 1)) | (value << offset);
    }
}
