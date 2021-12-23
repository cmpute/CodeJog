use rand::{Rng, thread_rng};
use rand::distributions::Uniform;
use crate::traits::{ModInt, Arithmetic};

/// Returns floor(log(2, target))
#[inline]
pub fn lb(target: u64) -> u8 {
    assert_ne!(target, 0);
    (target as f32).log2() as u8
    // TODO: convert target to f32 may lose precision
    // TODO: or 64 - x.leading_zeros()?
}

/// Returns floor(log(base, target))
#[inline]
pub fn log(target: u64, base: u64) -> u8 {
    assert_ne!(target, 0);
    (target as f32).log(base as f32) as u8
}

/// Returns floor(sqrt(target))
#[inline]
pub fn sqrt(target: u64) -> u64 {
    if target < (1 << 15) {
        (target as f32).sqrt() as u64
    }
    else {
        num_integer::sqrt(target)
    }
}

// Check whether input is a square number
#[inline]
pub fn is_sq(target: u64) -> bool {
    let s = sqrt(target);
    s * s == target
}

#[inline]
/// Returns greatest common divisor between a, b
pub fn gcd(a: u64, b: u64) -> u64 {
    num_integer::gcd(a, b)
}

impl ModInt for u64 {
    fn mul_mod(self: u64, rhs: u64, m: &u64) -> u64 {
        if let Some(ab) = self.checked_mul(rhs) {
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

    fn pow_mod(self: u64, exp: u64, m: &u64) -> u64 {
        if exp == 1 {
            return self % m;
        }

        if exp < (u32::MAX as u64) {
            if let Some(ae) = self.checked_pow(exp as u32) {
                return ae % m;
            }
        }

        let mut multi = self % m;
        let mut exp = exp;
        let mut result = 1;
        while exp > 0 {
            if exp & 1 > 0 {
                result = result.mul_mod(multi, m);
            }
            multi = multi.mul_mod(multi, m);
            exp >>= 1;
        }
        result
    }
}

// /// Return random integer between a and b
// pub fn randrange(a: u64, b: u64) -> u64 {
//     let diff = b - a;
//     rand::random::<u64>() % diff + a
// }

impl Arithmetic for u64 {
    fn is_sprp(&self, witness: u64) -> bool {
        // find 2^shift*u + 1 = n
        let tm1 = self - 1;
        let shift = tm1.trailing_zeros();
        let u = tm1 >> shift;

        let mut x = witness.pow_mod(u, self);
        if x == 1 || x == tm1 { return true }

        for _ in 0..shift {
            x = x.mul_mod(x, self);
            if x == tm1 { return true }
        }

        x == 1
    }

    fn pollard_rho(&self, offset: Self, trials: u32) -> Option<Self> {
        let mut rng = thread_rng();
        let mut trials = trials;
        'trial_loop: while trials > 0 {
            let mut a = rng.sample(Uniform::new(2, self));
            let mut b = a;
            let mut i = 1; let mut j = 2;
            loop {
                i += 1;
                a = (a.mul_mod(a, self) + offset) % self;
                if a == b {
                    trials -= 1;
                    continue 'trial_loop
                }
                let diff = if b > a { b - a } else { a - b }; // abs_diff
                let d = gcd(diff, *self);
                if 1 < d && d < a {
                    return Some(d)
                }
                if i == j {
                    b = a;
                    j <<= 1;
                }
            }
        }
        None
    }
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
        assert_eq!(sqrt(a), (a as f32).sqrt() as u64);
    }

    #[test]
    fn mod_test() {
        let a = rand::random::<u64>() % 100000;
        let m = rand::random::<u64>() % 100000;
        assert_eq!(a.mul_mod(a, &m), (a * a) % m);
        assert_eq!(a.pow_mod(3, &m), a.pow(3) % m);
    }
}
