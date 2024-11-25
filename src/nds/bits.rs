// NOTE: do NOT use these for generics
// i don't know why but it ruins the generic magic

use num_traits::{PrimInt, WrappingSub};

pub trait Bits<T> {
    fn get_bit(&self, offset: Self) -> bool;
    fn get_bits(&self, offset: Self, end: Self) -> Self;
    fn set_bit(&mut self, offset: Self, value: bool);
    fn set_bits(&mut self, offset: Self, end: Self, value: Self);

    fn to_bytes<const B: usize>(&self) -> [u8; B];

    fn sign_extend(&self, from: u32) -> i32;
    fn set_part<const B: usize>(&mut self, offset: Self, value: Self);
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

    #[inline(always)]
    fn set_part<const B: usize>(&mut self, offset: T, value: T) {
        let offset = offset.to_usize().unwrap() << 3;
        // need to use a match statement because T is an i32 for some reason or something
        let mask = match B {
            1 => T::from(0xFF).unwrap(),
            2 => T::from(0xFFFF).unwrap(),
            4 => T::from(0xFFFFFFFF_u32).unwrap(),
            _ => unreachable!("invalid byte size {}", B),
        } << offset;
        *self = (*self & !mask) | ((value << offset) & mask)
    }
}

pub trait Bytes {
    fn into_word(self) -> u32;
    fn into_halfword(self) -> u16;
    fn into_byte(self) -> u8;
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

    #[inline(always)]
    fn into_byte(self) -> u8 {
        if T > 1 {
            panic!("attempted to convert {} bytes into a byte", T);
        }

        self[0]
    }
}

pub trait IfElse<T> {
    fn if_else(&self, true_val: T, false_val: T) -> T;
}

impl<T> IfElse<T> for bool
where
    T: PrimInt + WrappingSub,
{
    // leo taught me this fast conditional strat like a year ago
    #[inline(always)]
    fn if_else(&self, true_val: T, false_val: T) -> T {
        let cond_mask = T::from(*self as usize).unwrap().wrapping_sub(&T::one());
        (!cond_mask & true_val) | (cond_mask & false_val)
    }
}
