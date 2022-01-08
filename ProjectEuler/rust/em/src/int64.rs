use num_integer::sqrt;
use crate::traits::{ModInt, ArithmeticHelpers};

/// Returns floor(log(2, target))
#[inline]
pub fn lb(target: u64) -> u8 {
    assert_ne!(target, 0);
    63 - target.leading_zeros() as u8
}

/// Returns floor(log(base, target))
#[inline]
pub fn log(target: u64, base: u64) -> u8 {
    assert_ne!(target, 0);
    match base {
        2 => lb(target),
        _ => (target as f32).log(base as f32) as u8
    }
}

// Check whether input is a square number
#[inline]
pub fn is_sq(target: u64) -> bool {
    let s = sqrt(target);
    s * s == target
}

impl ArithmeticHelpers for u64 {
    /// Returns greatest common divisor between a, b
    #[inline]
    fn trailing_zeros(&self) -> usize { u64::trailing_zeros(*self) as usize }
}

impl ModInt<&u64, &u64> for &u64 {
    type Output = u64;

    fn mul_mod(self, rhs: &u64, m: &u64) -> u64 {
        if let Some(ab) = self.checked_mul(*rhs) {
            return ab % m
        }

        let mut a = self % m;
        let mut b = rhs % m;

        if let Some(ab) = a.checked_mul(b) {
            return ab % m
        }

        let mut result: u64 = 0;
        while b > 0 {
            if b & 1 > 0 {
                result += a;
                result %= m;
            }
            a <<= 1;
            if a >= *m {
                a %= m;
            }
            b >>= 1;
        }
        result
    }

    fn pow_mod(self, exp: &u64, m: &u64) -> u64 {
        if *exp == 1 {
            return self % m;
        }

        if *exp < (u32::MAX as u64) {
            if let Some(ae) = self.checked_pow(*exp as u32) {
                return ae % m;
            }
        }

        let mut multi = self % m;
        let mut exp = *exp;
        let mut result = 1;
        while exp > 0 {
            if exp & 1 > 0 {
                result = result.mul_mod(&multi, m);
            }
            multi = multi.mul_mod(&multi, m);
            exp >>= 1;
        }
        result
    }
}

impl ModInt<u64, &u64> for &u64 {
    type Output = u64;
    #[inline]
    fn mul_mod(self, rhs: u64, m: &u64) -> u64 { self.mul_mod(&rhs, m) }
    #[inline]
    fn pow_mod(self, exp: u64, m: &u64) -> u64 { self.pow_mod(&exp, m) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_test() {
        let a = rand::random::<u64>() % 100000;
        // let b = rand::random::<u64>() % 100000;
        assert_eq!(lb(a), (a as f32).log2() as u8);
        assert_eq!(log(a, 8), (a as f32).log(8.) as u8);
    }

    #[test]
    fn mod_test() {
        let a = rand::random::<u64>() % 100000;
        let m = rand::random::<u64>() % 100000;
        assert_eq!(a.mul_mod(a, &m), (a * a) % m);
        assert_eq!(a.pow_mod(3, &m), a.pow(3) % m);
    }
}
