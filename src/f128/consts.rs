use core::num::FpCategory;

use num_traits::{float::FloatCore, FloatConst};

use super::f128;

impl f128 {
    /// The radix or base of the internal representation.
    pub const RADIX: u32 = 2;

    /// Number of significant digits in base 2.
    pub const MANTISSA_DIGITS: u32 = 104;

    /// Approximate number of significant digits in base 10.
    pub const DIGITS: u32 = 31;

    /// Machine epsilon value.
    pub const EPSILON: Self = Self::new(4.930380657631323783e-32, 0.0); // 2^-(106 - 2)

    /// Smallest finite value.
    // TODO: pub const MIN: Self = todo!();

    /// Smallest positive normal value.
    pub const MIN_POSITIVE: Self = Self::new(2.0041683600089728e-292, 0.0); // = 2^(-1022 + 53)

    /// Largest finite value.
    pub const MAX: Self = Self::new(1.79769313486231570815e+308, 9.97920154767359795037e+291); // (2^53 - 1) * 2^(1023 - 52), (2^53 - 1) * 2^((1023 - 52) - 54)

    /// One greater than the minimum possible normal power of 2 exponent.
    // TODO: pub const MIN_EXP: i32 = todo!();

    /// Maximum possible power of 2 exponent.
    // TODO: pub const MAX_EXP: i32 = todo!();

    /// Minimum possible normal power of 10 exponent.
    // TODO: pub const MIN_10_EXP: i32 = todo!();

    /// Maximum possible power of 10 exponent.
    // TODO: pub const MAX_10_EXP: i32 = todo!();

    /// Not a Number (NaN).
    pub const NAN: Self = Self::new(f64::NAN, f64::NAN);

    /// Infinity (∞).
    pub const INFINITY: Self = Self::new(f64::INFINITY, f64::INFINITY);

    /// Negative infinity (−∞).
    pub const NEG_INFINITY: Self = Self::new(f64::NEG_INFINITY, f64::NEG_INFINITY);
}

impl FloatCore for f128 {
    #[inline(always)]
    fn infinity() -> Self {
        Self::INFINITY
    }

    #[inline(always)]
    fn neg_infinity() -> Self {
        Self::NEG_INFINITY
    }

    #[inline(always)]
    fn nan() -> Self {
        Self::NAN
    }

    #[inline(always)]
    fn neg_zero() -> Self {
        (-0.0, -0.0).into()
    }

    #[inline(always)]
    fn min_value() -> Self {
        todo!() // TODO: Self::MIN
    }

    #[inline(always)]
    fn min_positive_value() -> Self {
        Self::MIN_POSITIVE
    }

    #[inline(always)]
    fn epsilon() -> Self {
        Self::EPSILON
    }

    #[inline(always)]
    fn max_value() -> Self {
        Self::MAX
    }

    fn classify(self) -> FpCategory {
        todo!() /* TODO: */
    }

    fn to_degrees(self) -> Self {
        todo!() /* TODO: */
    }

    fn to_radians(self) -> Self {
        todo!() /* TODO: */
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        todo!() /* TODO: */
    }
}

impl f128 {
    /// Euler’s number.
    pub const E: Self = Self::new(2.718281828459045091e+00, 1.445646891729250158e-16);

    /// `1.0 / π`
    pub const FRAC_1_PI: Self = Self::new(0.31830988618379070e+00, -1.9678676675182486e-17);

    /// `1.0 / sqrt(2.0)`
    // TODO: pub const FRAC_1_SQRT_2: Self = todo!();

    /// `2.0 / π`
    pub const FRAC_2_PI: Self = Self::new(0.15915494309189535e+00, -9.8393383375912430e-18);

    /// `2.0 / sqrt(π)`
    // TODO: pub const FRAC_2_SQRT_PI: Self = todo!();

    /// `π / 2.0`
    pub const FRAC_PI_2: Self = Self::new(1.57079632679489660e+00, 6.1232339957367660e-17);

    /// `π / 3.0`
    pub const FRAC_PI_3: Self = Self::new(1.04719755119659790e+00, -1.0720817664510910e-16);

    /// `π / 4.0`
    pub const FRAC_PI_4: Self = Self::new(0.78539816339744830e+00, 3.0616169978683830e-17);

    /// `π / 6.0`
    pub const FRAC_PI_6: Self = Self::new(0.52359877559829890e+00, -5.3604088322554550e-17);

    /// `π / 8.0`
    pub const FRAC_PI_8: Self = Self::new(0.39269908169872414e+00, 1.5308084989341915e-17);

    /// `ln(10.0)`
    pub const LN_10: Self = Self::new(2.302585092994045901e+00, -2.170756223382249351e-16);

    /// `ln(2.0)`
    pub const LN_2: Self = Self::new(6.931471805599452862e-01, 2.319046813846299558e-17);

    /// `log10(e)`
    // TODO: pub const LOG10_E: Self = todo!();

    /// `log2(e)`
    // TODO: pub const LOG2_E: Self = todo!();

    /// `π`
    pub const PI: Self = Self::new(3.141592653589793116e+00, 1.224646799147353207e-16);

    // `sqrt(2.0)`
    // TODO: pub const SQRT_2: Self = todo!();
}

impl FloatConst for f128 {
    #[inline(always)]
    fn E() -> Self {
        Self::E
    }

    #[inline(always)]
    fn FRAC_1_PI() -> Self {
        Self::FRAC_1_PI
    }

    #[inline(always)]
    fn FRAC_1_SQRT_2() -> Self {
        todo!() // TODO: Self::FRAC_1_SQRT_2
    }

    #[inline(always)]
    fn FRAC_2_PI() -> Self {
        Self::FRAC_2_PI
    }

    #[inline(always)]
    fn FRAC_2_SQRT_PI() -> Self {
        todo!() // TODO: Self::FRAC_2_SQRT_PI
    }

    #[inline(always)]
    fn FRAC_PI_2() -> Self {
        Self::FRAC_PI_2
    }

    #[inline(always)]
    fn FRAC_PI_3() -> Self {
        Self::FRAC_PI_3
    }

    #[inline(always)]
    fn FRAC_PI_4() -> Self {
        Self::FRAC_PI_4
    }

    #[inline(always)]
    fn FRAC_PI_6() -> Self {
        Self::FRAC_PI_6
    }

    #[inline(always)]
    fn FRAC_PI_8() -> Self {
        Self::FRAC_PI_8
    }

    #[inline(always)]
    fn LN_10() -> Self {
        Self::LN_10
    }

    #[inline(always)]
    fn LN_2() -> Self {
        Self::LN_2
    }

    #[inline(always)]
    fn LOG10_E() -> Self {
        todo!() // TODO: Self::LOG10_E
    }

    #[inline(always)]
    fn LOG2_E() -> Self {
        todo!() // TODO: Self::LOG2_E
    }

    #[inline(always)]
    fn PI() -> Self {
        Self::PI
    }

    #[inline(always)]
    fn SQRT_2() -> Self {
        todo!() // TODO: Self::SQRT_2
    }
}
