// NOTE: do NOT use these for generics
// i don't know why but it ruins the generic magic

pub trait Bits<T> {
    fn get_bit(&self, offset: Self) -> bool;
    fn get_bits(&self, offset: Self, end: Self) -> Self;
    fn set_bit(&mut self, offset: Self, value: bool);
    fn set_bits(&mut self, offset: Self, end: Self, value: Self);

    fn to_bytes<const B: usize>(&self) -> [u8; B];

    fn sign_extend(&self, from: u32) -> i32;
}

impl Bits<u64> for u64 {
    #[inline(always)]
    fn get_bit(&self, offset: u64) -> bool {
        (self >> offset) & 1 == 1
    }

    #[inline(always)]
    fn set_bit(&mut self, offset: u64, value: bool) {
        *self = (*self & !(1 << offset)) | ((value as u64) << offset);
    }

    #[inline(always)]
    fn get_bits(&self, offset: u64, end: u64) -> u64 {
        (self >> offset) & ((1 << (end - offset + 1)) - 1)
    }

    #[inline(always)]
    fn set_bits(&mut self, offset: u64, end: u64, value: u64) {
        let mask = ((1 << (end - offset + 1)) - 1) << offset;
        *self = (*self & !mask) | ((value << offset) & mask);
    }

    #[inline(always)]
    fn to_bytes<const B: usize>(&self) -> [u8; B] {
        let mut bytes = [0; B];
        let len = B.min(8);
        bytes[..len].copy_from_slice(&self.to_le_bytes()[..len]);
        bytes
    }

    #[inline(always)]
    fn sign_extend(&self, from: u32) -> i32 {
        let shift = 32 - from;
        ((*self << shift) as i32) >> shift
    }
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
        let mask = ((1 << (end - offset + 1)) - 1) << offset;
        *self = (*self & !mask) | ((value << offset) & mask);
    }

    #[inline(always)]
    fn to_bytes<const B: usize>(&self) -> [u8; B] {
        let mut bytes = [0; B];
        let len = B.min(4);
        bytes[..len].copy_from_slice(&self.to_le_bytes()[..len]);
        bytes
    }

    #[inline(always)]
    fn sign_extend(&self, from: u32) -> i32 {
        let shift = 32 - from;
        ((*self << shift) as i32) >> shift
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
        let mask = ((1 << (end - offset + 1)) - 1) << offset;
        *self = (*self & !mask) | ((value << offset) & mask);
    }

    #[inline(always)]
    fn to_bytes<const B: usize>(&self) -> [u8; B] {
        let mut bytes = [0; B];
        let len = B.min(2);
        bytes[..len].copy_from_slice(&self.to_le_bytes()[..len]);
        bytes
    }

    #[inline(always)]
    fn sign_extend(&self, from: u32) -> i32 {
        let shift = 16 - from;
        ((*self << shift) as i32) >> shift
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
        let mask = ((1 << (end - offset + 1)) - 1) << offset;
        *self = (*self & !mask) | ((value << offset) & mask);
    }

    #[inline(always)]
    fn to_bytes<const B: usize>(&self) -> [u8; B] {
        let mut bytes = [0; B];
        let len = B.min(1);
        bytes[..len].copy_from_slice(&self.to_le_bytes()[..len]);
        bytes
    }

    #[inline(always)]
    fn sign_extend(&self, from: u32) -> i32 {
        let shift = 8 - from;
        ((*self << shift) as i32) >> shift
    }
}

pub trait Bytes {
    fn into_word(self) -> u32;
    fn into_halfword(self) -> u16;
}

impl<const T: usize> Bytes for [u8; T] {
    #[inline(always)]
    fn into_word(self) -> u32 {
        let mut bytes = [0; 4];
        let len = T.min(4);
        bytes[..len].copy_from_slice(&self[..len]);
        u32::from_le_bytes(bytes)
    }

    #[inline(always)]
    fn into_halfword(self) -> u16 {
        let mut bytes = [0; 2];
        let len = T.min(2);
        bytes[..len].copy_from_slice(&self[..len]);
        u16::from_le_bytes(bytes)
    }
}
