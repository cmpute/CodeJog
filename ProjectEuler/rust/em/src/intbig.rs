use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{Zero, ToPrimitive};
use crate::traits::{ModInt, ArithmeticHelpers};

/// Returns floor(log(2, target))
#[inline]
pub fn lb(target: &BigUint) -> u64 {
    assert_ne!(target, &BigUint::zero());
    target.bits() - 1
}

/// Returns floor(log(base, target))
/// Reference: https://stackoverflow.com/questions/6827516/logarithm-for-biginteger
/// TODO: this could be further optimized using limbs
#[inline]
pub fn log(target: &BigUint, base: &BigUint) -> u64 {
    assert_ne!(target, &BigUint::zero());

    let mut counter: u64 = 1;
    let mut v = target.div_floor(&base);
    while v > BigUint::zero() {
        v = v.div_floor(&base);
        counter += 1;
    }
    counter
}

#[inline]
pub fn is_sq(target: &BigUint) -> bool {
    let s = target.sqrt();
    s.clone() * s == *target
}

impl ArithmeticHelpers for BigUint {
    /// Returns greatest common divisor between a, b
    #[inline]
    fn trailing_zeros(&self) -> usize { 
        match BigUint::trailing_zeros(&self) {
            Some(a) => a as usize, None => 0
        }
    }
}

impl ModInt<&BigUint, &BigUint> for &BigUint {    
    type Output = BigUint;

    fn mul_mod(self, rhs: &BigUint, m: &BigUint) -> BigUint {
        let a = self % m;
        let b = rhs % m;

        if let Some(sm) = m.to_u64() {
            let sself = a.to_u64().unwrap();
            let srhs = b.to_u64().unwrap();
            return BigUint::from(sself.mul_mod(&srhs, &sm));
        }

        (a * b) % m
    }

    #[inline]
    fn pow_mod(self, exp: &BigUint, m: &BigUint) -> BigUint {
        self.modpow(&exp, m)
    }
}

impl ModInt<BigUint, &BigUint> for BigUint {
    type Output = BigUint;
    #[inline]
    fn mul_mod(self, rhs: BigUint, m: &BigUint) -> BigUint { self.mul_mod(&rhs, m) }
    #[inline]
    fn pow_mod(self, exp: BigUint, m: &BigUint) -> BigUint { self.pow_mod(&exp, m) }
}
impl ModInt<BigUint, &BigUint> for &BigUint {
    type Output = BigUint;
    #[inline]
    fn mul_mod(self, rhs: BigUint, m: &BigUint) -> BigUint { self.mul_mod(&rhs, m) }
    #[inline]
    fn pow_mod(self, exp: BigUint, m: &BigUint) -> BigUint { self.pow_mod(&exp, m) }
}
impl ModInt<&BigUint, &BigUint> for BigUint {
    type Output = BigUint;
    #[inline]
    fn mul_mod(self, rhs: &BigUint, m: &BigUint) -> BigUint { (&self).mul_mod(rhs, m) }
    #[inline]
    fn pow_mod(self, exp: &BigUint, m: &BigUint) -> BigUint { (&self).pow_mod(exp, m) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::RandBigInt;

    #[test]
    fn mod_full_test() {
        let mut rng = rand::thread_rng();
        let a = rng.gen_biguint(500); let ra = &a;
        let m = rng.gen_biguint(500); let rm = &m;
        assert_eq!(ra.mul_mod(ra, rm), (ra * ra) % rm);
        assert_eq!(ra.pow_mod(BigUint::from(3u8), rm), ra.pow(3) % rm);
    }
}
