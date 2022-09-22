use core::cmp::Ordering;

use num_traits::{Num, One, Zero};

use crate::utils::quick_two_sum;

/// Double-double data structure.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct f128(pub f64, pub f64);

impl f128 {
    /// Build a new f128 from a pair of f64.
    #[inline(always)]
    pub const fn new(a0: f64, a1: f64) -> Self {
        Self(a0, a1)
    }
}

impl From<(f64, f64)> for f128 {
    #[inline(always)]
    fn from((a0, a1): (f64, f64)) -> Self {
        Self::new(a0, a1)
    }
}

impl PartialEq<f64> for f128 {
    #[inline(always)]
    fn eq(&self, rhs: &f64) -> bool {
        self.0 == *rhs && self.1 == 0.0
    }
}

impl PartialOrd<f64> for f128 {
    fn partial_cmp(&self, rhs: &f64) -> Option<Ordering> {
        match self.0.partial_cmp(&rhs) {
            Some(Ordering::Equal) => self.1.partial_cmp(&0.0),
            ordering => ordering,
        }
    }
}

impl Zero for f128 {
    #[inline(always)]
    fn zero() -> Self {
        (0.0, 0.0).into()
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        self.0 == 0.0 || self.0 == -0.0
    }
}

impl One for f128 {
    #[inline(always)]
    fn one() -> Self {
        (1.0, 0.0).into()
    }
}

impl Num for f128 {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!() /* TODO: */
    }
}

/// Normalize f128 representation.
#[inline(always)]
pub fn normalize(a0: f64, a1: f64, a2: f64) -> f128 {
    quick_two_sum(a0, a1 + a2).into()
}
