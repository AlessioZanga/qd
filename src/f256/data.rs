/// Quad-double data structure.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct f256([f64; 4]);

impl f256 {
    /// Build a new f256 from a quadruple of f64.
    #[inline(always)]
    pub const fn new(a0: f64, a1: f64, a2: f64, a3: f64) -> Self {
        Self([a0, a1, a2, a3])
    }

    /// From normalized f256 representation.
    #[inline(always)]
    pub fn with_normalized(_a0: f64, _a1: f64, _a2: f64, _a3: f64, _a4: f64) -> Self {
        todo!() /* TODO: */
    }
}

impl From<(f64, f64, f64, f64)> for f256 {
    #[inline(always)]
    fn from((a0, a1, a2, a3): (f64, f64, f64, f64)) -> Self {
        Self::new(a0, a1, a2, a3)
    }
}
