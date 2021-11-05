use num_bigint::{BigInt, BigUint};
use num_traits::Zero;

/// Returns floor(log(2, target))
#[inline]
pub fn lb(target: BigUint) -> u64 {
    assert_ne!(target, BigUint::zero());
    target.bits() - 1
}

#[inline]
pub fn log(target: BigUint, base: BigUint) -> u64 {
    0 // TODO: Implement
}
