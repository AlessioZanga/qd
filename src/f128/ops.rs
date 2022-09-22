use core::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num_traits::{Inv, One};

use crate::utils::{two_diff, two_prod, two_sum};

use super::{data::normalize, f128};

/* Unary operators. */

impl Neg for f128 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        (-self.0, -self.1).into()
    }
}

impl Inv for f128 {
    type Output = Self;

    #[inline(always)]
    fn inv(self) -> Self::Output {
        f128::one() / self
    }
}

/* Double-Double + Double Ops */

impl Add<f64> for f128 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: f64) -> Self::Output {
        let (s, e) = two_sum(self.0, rhs);

        normalize(s, e, self.1)
    }
}

impl Sub<f64> for f128 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: f64) -> Self::Output {
        let (s, e) = two_diff(self.0, rhs);

        normalize(s, e, self.1)
    }
}

impl Mul<f64> for f128 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f64) -> Self::Output {
        let (s, e) = two_prod(self.0, rhs);

        normalize(s, e, self.1 * rhs)
    }
}

impl Div<f64> for f128 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        // Compute first approximation.
        let s = self.0 / rhs;

        // Compute second approximation.
        let (p0, p1) = two_prod(s, rhs);
        let (p0, e) = two_diff(self.0, p0);
        let e = (p0 + (e + self.1 - p1)) / rhs;

        normalize(s, e, 0.0)
    }
}

impl Rem<f64> for f128 {
    type Output = Self;

    fn rem(self, rhs: f64) -> Self::Output {
        todo!() /* TODO: */
    }
}

/* Double-Double + Double-Double Ops */

impl Add<f128> for f128 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!() /* TODO: */
    }
}

impl Sub<f128> for f128 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!() /* TODO: */
    }
}

impl Mul<f128> for f128 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!() /* TODO: */
    }
}

impl Div<f128> for f128 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        todo!() /* TODO: */
    }
}

impl Rem<f128> for f128 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        todo!() /* TODO: */
    }
}
