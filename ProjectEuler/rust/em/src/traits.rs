use num_traits::Num;

/// This trait describes modular arithmetic on a integer
pub trait ModInt : Num {
    /// Return (self * rhs) % m
    fn mul_mod(self, rhs: Self, m: &Self) -> Self;

    /// Return (self ^ exp) % m
    fn pow_mod(self, exp: Self, m: &Self) -> Self;
}

/// This trait describes arithmetic functions on a integer
pub trait Arithmetic : ModInt {
    /// Test if the integer is a strong probable prime
    fn is_sprp(&self, witness: Self) -> bool;

    /// Generate a factor of the integer using Pollard's Rho algorithm
    fn pollard_rho(&self, offset: Self, trials: u32) -> Option<Self>;
}
