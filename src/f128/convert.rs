use num_traits::{NumCast, ToPrimitive};

use super::f128;

impl ToPrimitive for f128 {
    fn to_i64(&self) -> Option<i64> {
        todo!() /* TODO: */
    }

    fn to_u64(&self) -> Option<u64> {
        todo!() /* TODO: */
    }
}

impl NumCast for f128 {
    #[inline(always)]
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        n.to_f64().and_then(|n| Some((n, 0.0).into()))
    }
}
