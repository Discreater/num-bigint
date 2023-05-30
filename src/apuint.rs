mod addition;
mod convert;

use core::cmp::Ordering;
use core::hash;

use core::fmt;
use num_traits::Zero;

use crate::big_digit::{BigDigit, BITS as INNER_BITS};
use crate::biguint::cmp_slice;
use crate::biguint::IntDigits;
use crate::std_alloc::{String, Vec};
use crate::BigUint;

const BITS: usize = INNER_BITS as usize;

pub struct ApUint {
    data: Vec<BigDigit>,
    len: usize,
}

impl ApUint {
    pub(crate) fn limb_len(&self) -> usize {
        self.len + BITS - 1 / BITS
    }
    pub(crate) fn tail_len(&self) -> usize {
        self.len % BITS
    }

    pub(crate) fn normalize(&mut self) {
        self.data.truncate(self.limb_len());
        let self_limb_len = self.limb_len();
        if self.data.len() >= self_limb_len {
            self.data[self_limb_len] &= (1 << self.tail_len()) - 1;
        }
    }

    pub(crate) fn normalized(&self) -> bool {
        self.data.last() != Some(&0)
            && self.data.len() <= self.limb_len()
            && self
                .data
                .last()
                .map(|&x| x.leading_zeros() as usize + self.tail_len() >= BITS)
                .unwrap_or(true)
    }

    #[inline]
    pub fn to_str_radix(&self, radix: u32) -> String {
        let mut v: BigUint = self.into();
        v.to_str_radix(radix)
    }
}

// Note: derived `Clone` doesn't specialize `clone_from`,
// but we want to keep the allocation in `data`.
impl Clone for ApUint {
    #[inline]
    fn clone(&self) -> Self {
        ApUint {
            data: self.data.clone(),
            len: self.len,
        }
    }

    #[inline]
    fn clone_from(&mut self, other: &Self) {
        self.data.clone_from(&other.data);
        self.normalize();
    }
}

impl hash::Hash for ApUint {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        debug_assert!(self.normalized());
        self.data.hash(state);
    }
}

impl PartialEq for ApUint {
    #[inline]
    fn eq(&self, other: &ApUint) -> bool {
        debug_assert!(self.normalized());
        debug_assert!(self.normalized());
        self.data == other.data
    }
}
impl Eq for ApUint {}

impl PartialOrd for ApUint {
    #[inline]
    fn partial_cmp(&self, other: &ApUint) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ApUint {
    #[inline]
    fn cmp(&self, other: &ApUint) -> Ordering {
        debug_assert!(self.normalized());
        cmp_slice(&self.data[..], &other.data[..])
    }
}

impl Default for ApUint {
    #[inline]
    fn default() -> Self {
        Zero::zero()
    }
}

impl fmt::Debug for ApUint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for ApUint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad_integral(true, "", &self.to_str_radix(10))
    }
}

impl fmt::LowerHex for ApUint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad_integral(true, "0x", &self.to_str_radix(16))
    }
}

impl fmt::UpperHex for ApUint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = self.to_str_radix(16);
        s.make_ascii_uppercase();
        f.pad_integral(true, "0x", &s)
    }
}

impl fmt::Binary for ApUint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad_integral(true, "0b", &self.to_str_radix(2))
    }
}

impl fmt::Octal for ApUint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad_integral(true, "0o", &self.to_str_radix(8))
    }
}

impl Zero for ApUint {
    #[inline]
    fn zero() -> Self {
        ApUint {
            data: Vec::new(),
            len: 0,
        }
    }

    #[inline]
    fn set_zero(&mut self) {
        self.data.clear();
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.data.is_empty()
    }
}

impl IntDigits for ApUint {
    #[inline]
    fn digits(&self) -> &[BigDigit] {
        &self.data
    }
    #[inline]
    fn digits_mut(&mut self) -> &mut Vec<BigDigit> {
        &mut self.data
    }
    #[inline]
    fn normalize(&mut self) {
        self.normalize();
    }
    #[inline]
    fn capacity(&self) -> usize {
        self.data.capacity()
    }
    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }
}
