use core::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num_traits::{Inv, One};

use crate::utils::{quick_two_sum, two_diff, two_prod, two_sum};

use super::f128;

/* Unary operators. */

impl Neg for f128 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self::Output::new(-self[0], -self[1])
    }
}

impl Inv for f128 {
    type Output = Self;

    #[inline(always)]
    fn inv(self) -> Self::Output {
        Self::one() / self
    }
}

/* Double-Double + Double Ops */

impl Add<f64> for f128 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: f64) -> Self::Output {
        let (s, e) = two_sum(self[0], rhs);

        Self::Output::with_normalized(s, e, self[1])
    }
}

impl Add<f128> for f64 {
    type Output = f128;

    #[inline(always)]
    fn add(self, rhs: f128) -> Self::Output {
        rhs + self
    }
}

impl Sub<f64> for f128 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: f64) -> Self::Output {
        let (s, e) = two_diff(self[0], rhs);

        Self::Output::with_normalized(s, e, self[1])
    }
}

impl Sub<f128> for f64 {
    type Output = f128;

    #[inline(always)]
    fn sub(self, rhs: f128) -> Self::Output {
        let (s, e) = two_diff(self, rhs[0]);

        Self::Output::with_normalized(s, e, -rhs[1])
    }
}

impl Mul<f64> for f128 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f64) -> Self::Output {
        let (s, e) = two_prod(self[0], rhs);

        Self::Output::with_normalized(s, e, self[1] * rhs)
    }
}

impl Mul<f128> for f64 {
    type Output = f128;

    #[inline(always)]
    fn mul(self, rhs: f128) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for f128 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        // Compute first approximation.
        let s = self[0] / rhs;

        // Compute second approximation.
        let (p0, p1) = two_prod(s, rhs);
        let (p0, e) = two_diff(self[0], p0);
        let e = (p0 + (e + self[1] - p1)) / rhs;

        Self::Output::with_normalized(s, e, 0.0)
    }
}

impl Div<f128> for f64 {
    type Output = f128;

    #[inline(always)]
    fn div(self, rhs: f128) -> Self::Output {
        Self::Output::new(self, 0.0) / rhs
    }
}

impl Rem<f64> for f128 {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: f64) -> Self::Output {
        self % Self::Output::new(rhs, 0.0)
    }
}

/* Double-Double + Double-Double Ops */

impl Add<f128> for f128 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (s0, e0) = two_sum(self[0], rhs[0]);
        let (s1, e1) = two_sum(self[1], rhs[1]);

        let (s0, s1) = quick_two_sum(s0, s1 + e0);

        Self::Output::with_normalized(s0, s1, e1)
    }
}

impl Sub<f128> for f128 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self::Output {
        todo!() /* TODO: */
    }
}

impl Mul<f128> for f128 {
    type Output = Self;

    fn mul(self, _rhs: Self) -> Self::Output {
        todo!() /* TODO: */
    }
}

impl Div<f128> for f128 {
    type Output = Self;

    fn div(self, _rhs: Self) -> Self::Output {
        todo!() /* TODO: */
    }
}

impl Rem<f128> for f128 {
    type Output = Self;

    fn rem(self, _rhs: Self) -> Self::Output {
        todo!() /* TODO: */
    }
}
