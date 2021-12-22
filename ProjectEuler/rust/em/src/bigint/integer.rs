use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::Zero;

/// Returns floor(log(2, target))
#[inline]
pub fn lb(target: &BigUint) -> u64 {
    assert_ne!(target, &BigUint::zero());
    target.bits() - 1
}

/// Returns floor(log(base, target))
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

// REF: https://stackoverflow.com/questions/6827516/logarithm-for-biginteger
