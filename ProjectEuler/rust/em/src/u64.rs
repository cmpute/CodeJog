// u64 version of the math utilities
use std::collections::{HashMap, HashSet};

/// Returns floor(log(2, target))
#[inline]
pub fn lb(target: u64) -> u8 {
    assert_ne!(target, 0);
    (target as f32).log2() as u8 // hardware acceled
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
    if target < 10000000000 {
        (target as f32).sqrt() as u64
    }
    else {
        num_integer::sqrt(target)
    }
}

#[inline]
pub fn gcd(a: u64, b: u64) -> u64 {
    num_integer::gcd(a, b)
}

/// Return (a * b) % mod, even works for very large numbers
pub fn mulmod(a: u64, b: u64, m: u64) -> u64 {
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

pub fn randrange(a: u64, b: u64) -> u64 {
    let diff = b - a;
    rand::random::<u64>() % diff + a
}

// ----- Prime utilities -----
// TODO: improve based on `primal` and `primes` crates
pub struct PrimeBuffer {
    list: Vec<u64>,
    current: u64
}

impl PrimeBuffer {
    #[inline]
    pub fn new() -> Self {
        let list = vec![2]; // Add more primes here to fasten sieve
        PrimeBuffer { list , current: 3 }
    }

    pub fn fprime(&self, target: u64) -> usize {
        let mut lo: usize = 0;
        let mut hi: usize = self.list.len();

        while lo < hi {
            let mid = (lo + hi) / 2;
            if self.list[mid] < target { lo = mid + 1; }
            else { hi = mid; }
        }
        lo
    }

    /// Return whether target is a prime, even works for very large numbers and very fast
    /// # Parameter
    /// confidence: The larger it is, the result is more reliable
    /// 
    /// # Reference
    /// Miller–Rabin primality test
    /// http://www.cnblogs.com/vongang/archive/2012/03/15/2398626.html
    pub fn isprime(&self, target: u64, confidence: Option<i8>) -> bool {
        let confidence = confidence.unwrap_or(5);
        assert!(target > 1);

        // first find the prime list
        let idx = self.fprime(target);
        if idx != self.list.len() {
            return self.list[idx] == target;
        }

        let mut u = target - 1;
        let mut shift = 0;
        while u & 1 > 0 {
            shift += 1;
            u >>= 1;
        }

        for _ in 0..confidence {
            let mut x = randrange(2, target);
            if x % target == 0 {
                continue;
            }
            x = powmod(x, u, target);
            let mut pre = x;

            for _ in 0..shift {
                x = mulmod(x, x, target);
                if x == 1 && pre != 1 && pre != target - 1 {
                    return false;
                }
                pre = x;
            }

            if x != 1 {
                return false;
            }
        }
        true
    }

    pub fn factors(&mut self, target: u64) -> HashMap<u64, u64> {
        if self.isprime(target, None) {
            let mut result = HashMap::new();
            result.insert(target, 1);
            return result;
        }

        self.factors_naive(target)
        // if lb(target) < 1000000 {
        //     self.factors_naive(target)
        // }
        // else {
        //     self.factors_divide(target)
        // }
    }

    fn factors_naive(&mut self, target: u64) -> HashMap<u64, u64> {
        let mut t = target;
        let mut result: HashMap<u64, u64> = HashMap::new();
        for p in self.primes(sqrt(target) + 1).iter() {
            while t % p == 0 {
                t = t / p;
                let count = result.entry(*p).or_insert(0);
                *count += 1;
            }
            if t == 1 {
                break
            }
        }

        if t != 1 {
            result.insert(t, 1);
        }
        result
    }

    // fn factors_divide(&mut self, target: u64) -> HashMap<u64, u64> {
        
    // }

    /// Returns all primes **below** limit. The primes are sorted.
    /// 
    /// # Reference
    /// https://stackoverflow.com/questions/2068372/fastest-way-to-list-all-primes-below-n/3035188#3035188
    /// 
    /// # Note
    /// List primes works very slow for limit larger than 2^25 and won't work if limit > 2^30
    fn primes(&mut self, limit: u64) -> &[u64] {
        if limit < self.current {
            return &self.list[..self.fprime(limit)]
        }

        // create sieve and filter with existing primes
        let mut sieve: HashSet<_> = ((self.current | 1) .. limit).step_by(2).collect();
        for p in self.list.iter().skip(1) { // skip pre-filtered 2
            let start = if p * p < self.current {
                p * ((self.current / p) | 1)
            } else {
                p * p
            };
            for multi in (start..limit).step_by(2 * (*p as usize)) {
                if sieve.contains(&multi) {
                    sieve.remove(&multi);
                }
            }
        }

        // sieve with new primes
        let new_start = self.current | 1;
        let new_end = sqrt(limit) + 1;
        for p in (new_start..new_end).step_by(2) {
            if !sieve.contains(&p) {
                continue;
            }
            for multi in (p * p .. limit).step_by(2 * (p as usize)) {
                sieve.remove(&multi);
            }
        }

        // sort the sieve
        let mut primes: Vec<_> = sieve.into_iter().collect();
        primes.sort();
        self.list.extend(primes.into_iter());
        self.current = limit;

        &self.list[..]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn random_test() {
        let a = rand::random::<u64>() % 100000;
        let b = rand::random::<u64>() % 100000;
        assert_eq!(lb(a), (a as f32).log2() as u8);
        assert_eq!(log(a, 8), (a as f32).log(8.) as u8);
        assert_eq!(sqrt(a), (a as f32).sqrt() as u64);
    }

    #[test]
    fn prime_test(){
        let mut pb = PrimeBuffer::new();
        assert!(pb.isprime(6469693333, None));
        let prime50 = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
        let prime100 = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
        assert_eq!(pb.primes(50), prime50);
        assert_eq!(pb.primes(100), prime100);
        let fac123456789 = HashMap::from_iter([(3, 2), (3803, 1), (3607, 1)]);
        let fac = pb.factors(123456789);
        assert_eq!(fac, fac123456789);
    }
}
