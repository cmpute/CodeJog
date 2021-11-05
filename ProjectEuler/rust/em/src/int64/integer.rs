// u64 version of the math utilities

/// Returns floor(log(2, target))
#[inline]
pub fn lb(target: u64) -> u8 {
    assert_ne!(target, 0);
    (target as f32).log2() as u8
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

/// Return (a * b) % mod, even works for very large numbers
pub fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    match a.checked_mul(b) {
        Some(ab) => ab % m,
        None => {
            let mut result: u64 = 0;
            let mut a = a % m;
            let mut b = b % m;
            while b > 0 {
                if b & 1 > 0 {
                    result += a;
                    result %= m;
                }
                a <<= 1;
                if a >= m {
                    a %= m;
                }
                b >>= 1;
            }
            result
        }
    }
}

/// Return (a ^ exp) % mod, even works for very large numbers
pub fn powmod(a: u64, exp: u64, m: u64) -> u64 {
    if exp == 1 {
        return a % m;
    }

    let mut multi = a % m;
    let mut exp = exp;
    let mut result = 1;
    while exp > 0 {
        if exp & 1 > 0 {
            result = mulmod(result, multi, m);
        }
        multi = mulmod(multi, multi, m);
        exp >>= 1;
    }
    result
}

/// Return random integer between a and b
pub fn randrange(a: u64, b: u64) -> u64 {
    let diff = b - a;
    rand::random::<u64>() % diff + a
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
}
