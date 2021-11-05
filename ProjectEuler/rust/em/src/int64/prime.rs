// u64 version of prime related functions
// TODO: improve based on `primal`, `primes` and even `concurrent_prime_sieve` crates

use std::collections::{HashMap, HashSet};
use super::integer::*;

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
    pub fn is_prime(&self, target: u64, confidence: Option<i8>) -> bool {
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
        if self.is_prime(target, None) {
            let mut result = HashMap::new();
            result.insert(target, 1);
            return result;
        }

        const FACTOR_THRESHOLD: u64 = 1 << 10;
        if target < FACTOR_THRESHOLD {
            self.factors_naive(target)
        }
        else {
            self.factors_divide(target)
        }
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

    /// Find the factors by dividing the target by a proper divider recursively
    fn factors_divide(&mut self, target: u64) -> HashMap<u64, u64> {
        // find a proper divisor
        let p = loop {
            if let Ok(d) = self.divisor(target) {
                break d;
            }
        };

        let mut f1 = self.factors(p);
        let f2 = self.factors(target / p);
        for (factor, exponent) in &f2 {
            *f1.entry(*factor).or_insert(0) += exponent;
        }
        f1
    }

    /// Return a proper divisor of target (randomly), even works for very large numbers
    fn divisor(&mut self, target: u64) -> Result<u64, ()> {
        if self.is_prime(target, None) {
            return Err(())
        }

        const DIVISOR_THRESHOLD: u64 = 1 << 40;
        if target < DIVISOR_THRESHOLD {
            self.divisor_naive(target)
        }
        else {
            self.divisor_rho(target)
        }
    }

    fn divisor_naive(&mut self, target: u64) -> Result<u64, ()> {
        for p in self.primes(sqrt(target)) {
            if target % p == 0 {
                return Ok(*p);
            }
        }
        Err(())
    }

    /// Pollard's rho algorithm
    /// http://blog.csdn.net/z690933166/article/details/9865755
    fn divisor_rho(&self, target: u64) -> Result<u64, ()> {
        let mut p = target;
        while p >= target {
            let mut a = randrange(1, target);
            let mut b = a;
            let t = randrange(1, target);
            let mut i = 1; let mut j = 1;
            p = loop {
                i += 1;
                a = (mulmod(a, a, target) + t) % target;
                if a == b {
                    break target;
                }
                let diff = if b > a { b - a } else { a - b };
                let d = gcd(diff, target);
                if 1 < d || d < a {
                    break d;
                }
                if i == j {
                    b = a;
                    j <<= 1;
                }
            }
        }

        if p == 1 || p == target {
            Err(())
        } else {
            Ok(p)
        }
    }

    /// Returns all primes **below** limit. The primes are sorted.
    /// 
    /// # Reference
    /// https://stackoverflow.com/questions/2068372/fastest-way-to-list-all-primes-below-n/3035188#3035188
    /// 
    /// # Note
    /// List primes works very slow for limit larger than 2^25 and won't work if limit > 2^30
    pub fn primes(&mut self, limit: u64) -> &[u64] {
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

    /// Returns primes of certain amount counting from 2. The primes are sorted.
    pub fn nprimes(&mut self, count: usize) -> &[u64] {
        loop {
            let _ = self.primes(self.current * (count as u64) / (self.list.len() as u64));
            if self.list.len() >= count {
                break &self.list[..count]
            }
        }
    }

    pub fn clear(&mut self) {
        self.list.clear();
        self.list.push(2);
        self.current = 3;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn prime_test(){
        let mut pb = PrimeBuffer::new();
        assert!(pb.is_prime(6469693333, None));
        let prime50 = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
        let prime100 = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
        assert_eq!(pb.primes(50), prime50);
        assert_eq!(pb.primes(100), prime100);
        let fac123456789 = HashMap::from_iter([(3, 2), (3803, 1), (3607, 1)]);
        let fac = pb.factors(123456789);
        assert_eq!(fac, fac123456789);
    }
}

