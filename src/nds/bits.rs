// NOTE: do NOT use these for generics
// i don't know why but it ruins the generic magic

use num_traits::PrimInt;

pub trait Bits<T> {
    fn get_bit(&self, offset: Self) -> bool;
    fn get_bits(&self, offset: Self, end: Self) -> Self;
    fn set_bit(&mut self, offset: Self, value: bool);
    fn set_bits(&mut self, offset: Self, end: Self, value: Self);

    fn to_bytes<const B: usize>(&self) -> [u8; B];

    fn sign_extend(&self, from: u32) -> i32;
}

impl<T> Bits<T> for T
where
    T: PrimInt,
{
    #[inline(always)]
    fn get_bit(&self, offset: T) -> bool {
        (*self >> offset.to_usize().unwrap()) & T::one() == T::one()
    }

    #[inline(always)]
    fn set_bit(&mut self, offset: T, value: bool) {
        *self = (*self & !(T::one() << offset.to_usize().unwrap()))
            | ((T::from(value as usize).unwrap()) << offset.to_usize().unwrap());
    }

    #[inline(always)]
    fn get_bits(&self, offset: T, end: T) -> T {
        (*self >> offset.to_usize().unwrap())
            & ((T::one() << (end - offset + T::one()).to_usize().unwrap()) - T::one())
    }

    #[inline(always)]
    fn set_bits(&mut self, offset: T, end: T, value: T) {
        let mask = ((T::one() << (end - offset + T::one()).to_usize().unwrap()) - T::one())
            << offset.to_usize().unwrap();
        *self = (*self & !mask) | ((value << offset.to_usize().unwrap()) & mask);
    }

    #[inline(always)]
    fn to_bytes<const B: usize>(&self) -> [u8; B] {
        let mut bytes = [0; B];
        let len = B.min(8);
        bytes[..len].copy_from_slice(&self.to_usize().unwrap().to_le_bytes()[..len]);
        bytes
    }

    #[inline(always)]
    fn sign_extend(&self, from: u32) -> i32 {
        let shift = 32 - from;
        ((self.to_usize().unwrap() << shift) as i32) >> shift
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
