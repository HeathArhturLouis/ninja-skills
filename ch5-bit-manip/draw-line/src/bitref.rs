use std::cmp::PartialEq;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitRef<'a> {
    bit: bool,
    phantom: PhantomData<&'a u8>,
}

// Caches value and writes on Drop.
#[derive(Debug, PartialEq, Eq)]
pub struct BitMut<'a> {
    bit: bool,
    source: &'a mut u8,
    idx: u8, // in [0, 8)
}

impl<'a> BitRef<'a> {
    pub fn new(source: &'a u8, idx: u8) -> Self {
        assert!(idx < 8);
        unsafe { Self::new_unchecked(source, idx) }
    }

    pub unsafe fn new_unchecked(source: &'a u8, idx: u8) -> Self {
        debug_assert!(idx < 8);
        Self {
            bit: (*source & (1 << idx)) != 0,
            phantom: PhantomData,
        }
    }
}

impl<'a> BitMut<'a> {
    pub fn new(source: &'a mut u8, idx: u8) -> Self {
        assert!(idx < 8);
        unsafe { Self::new_unchecked(source, idx) }
    }

    pub unsafe fn new_unchecked(source: &'a mut u8, idx: u8) -> Self {
        debug_assert!(idx < 8);
        Self {
            bit: (*source & (1 << idx)) != 0,
            source,
            idx,
        }
    }
}

impl<'a> Deref for BitRef<'a> {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.bit
    }
}

impl<'a> Deref for BitMut<'a> {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.bit
    }
}

impl<'a> DerefMut for BitMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bit
    }
}

impl<'a> Drop for BitMut<'a> {
    fn drop(&mut self) {
        if self.bit {
            *self.source |= 1 << self.idx;
        } else {
            *self.source &= !(1 << self.idx);
        }
    }
}

impl<'a> PartialEq<bool> for BitRef<'a> {
    fn eq(&self, other: &bool) -> bool {
        *self.deref() == *other
    }
}

impl<'a> PartialEq<bool> for BitMut<'a> {
    fn eq(&self, other: &bool) -> bool {
        *self.deref() == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_bitref() {
        let source: u8 = 0b_0011_0100;
        assert_eq!(*BitRef::new(&source, 0), false);
        assert_eq!(*BitRef::new(&source, 1), false);
        assert_eq!(*BitRef::new(&source, 2), true);
        assert_eq!(*BitRef::new(&source, 3), false);
        assert_eq!(*BitRef::new(&source, 4), true);
        assert_eq!(*BitRef::new(&source, 5), true);
        assert_eq!(*BitRef::new(&source, 6), false);
        assert_eq!(*BitRef::new(&source, 7), false);
    }

    #[test]
    // We should be able to take multiple shared references to the
    // same byte, even to the same bit.
    fn mult_bitref() {
        let source: u8 = 48;
        let mut refs = Vec::new();
        for i in 0..16 {
            refs.push(BitRef::new(&source, i % 8));
        }
    }

    #[test]
    #[should_panic]
    fn too_big_idx() {
        BitRef::new(&0, 8);
    }

    #[test]
    fn bitmut() {
        let mut source: u8 = 0b_0000_1111;
        {
            let mut b = BitMut::new(&mut source, 0);
            *b = false;
        }
        assert_eq!(source, 0b_0000_1110);
        for i in 1..8 {
            let mut b = BitMut::new(&mut source, i);
            *b = !*b;
        }
        assert_eq!(source, 0b_1111_0000);
    }
}
