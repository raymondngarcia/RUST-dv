// bits.rs
#![allow(dead_code)]

use std::ops::{BitAnd, BitOr, BitXor, Not};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bits<const N: u8> {
    data: u128,
}

impl<const N: u8> Bits<N> {
    const MASK: u128 = if N == 128 { u128::MAX } else { (1u128 << N) - 1 };

    /// Create a new Bits<N> with value, automatically masked
    pub fn new(val: u128) -> Self {
        assert!(N <= 128, "Bits<N> only supports N <= 128");
        Bits {
            data: val & Self::MASK,
        }
    }

    /// Get the value of bit i
    pub fn get(&self, i: u8) -> bool {
        assert!(i < N);
        (self.data & (1u128 << i)) != 0
    }

    /// Set the value of bit i
    pub fn set(&mut self, i: u8, val: bool) {
        assert!(i < N);
        if val {
            self.data |= 1u128 << i;
        } else {
            self.data &= !(1u128 << i);
        }
    }

    /// Return the full value as u128
    pub fn value(&self) -> u128 {
        self.data
    }

    /// Display as hex string
    pub fn to_hex(&self) -> String {
        format!("{:X}", self.data)
    }
}

// Bitwise operations
impl<const N: u8> BitAnd for Bits<N> {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Bits::new(self.data & rhs.data)
    }
}

impl<const N: u8> BitOr for Bits<N> {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Bits::new(self.data | rhs.data)
    }
}

impl<const N: u8> BitXor for Bits<N> {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Bits::new(self.data ^ rhs.data)
    }
}

impl<const N: u8> Not for Bits<N> {
    type Output = Self;
    fn not(self) -> Self {
        Bits::new(!self.data)
    }
}

// Debug display
impl<const N: u8> std::fmt::Debug for Bits<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0width$X}", self.data, width = ((N + 3) / 4) as usize)
    }
}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut a = Bits::<5>::new(0b10101);
        assert_eq!(a.get(0), true);
        assert_eq!(a.get(1), false);
        a.set(1, true);
        assert_eq!(a.value(), 0b10111);

        let b = Bits::<12>::new(0xABC);
        assert_eq!(b.value(), 0xABC);

        let c = Bits::<100>::new(0xFFFF_FFFF);
        assert_eq!(c.get(0), true);
        assert_eq!(c.get(31), true);
    }
}