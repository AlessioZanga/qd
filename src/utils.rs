/// Computes s = fl(a + b) and e = err(a + b), assuming |a| >= |b|.
#[inline(always)]
pub fn quick_two_sum(a: f64, b: f64) -> (f64, f64) {
    // Assuming |a| >= |b|.
    debug_assert!(a.abs() >= b.abs(), "|a| < |b|");

    let s = a + b;
    let e = b - (s - a);

    (s, e)
}

/// Computes s = fl(a + b) and e = err(a + b).
#[inline(always)]
pub fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let v = s - a;
    let e = (a - (s - v)) + (b - v);

    (s, e)
}

/// Computes s = fl(a - b) and e = err(a - b).
#[inline(always)]
pub fn two_diff(a: f64, b: f64) -> (f64, f64) {
    let s = a - b;
    let v = s - a;
    let e = (a - (s - v)) + (b + v);

    (s, e)
}

/// Splits a double precision floating point number into a_hi and a_lo.
#[inline(always)]
pub fn split(a: f64) -> (f64, f64) {
    let t = (2.0_f64.powi(27) + 1.0) * a;
    let a_hi = t - (t - a);
    let a_lo = a - a_hi;

    (a_hi, a_lo)
}

/// Computes p = fl(a * b) and e = err(a * b).
#[inline(always)]
pub fn two_prod(a: f64, b: f64) -> (f64, f64) {
    let p = a * b;
    let (a_hi, a_lo) = split(a);
    let (b_hi, b_lo) = split(b);

    // Compute the following using FMA.
    // let e = ((a_hi * b_hi - p) + a_hi * b_lo + a_lo * b_hi) + a_lo * b_lo;

    let e = -p;
    let e = f64::mul_add(a_hi, b_hi, e);
    let e = f64::mul_add(a_hi, b_lo, e);
    let e = f64::mul_add(a_lo, b_hi, e);
    let e = f64::mul_add(a_lo, b_lo, e);

    (p, e)
}
