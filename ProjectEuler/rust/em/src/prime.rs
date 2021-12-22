// u64 version of prime related functions
// TODO: improve based on `primal`, `primes` and even `concurrent_prime_sieve` crates

use std::collections::{HashMap, HashSet};
use num_traits::ToPrimitive;
use num_bigint::BigUint;
use crate::int64::integer::*;

pub struct PrimeBuffer {
    list: Vec<u64>, // list of found prime numbers
    current: u64 // all primes smaller than this value has to be in the prime list
}

impl PrimeBuffer { // TODO: this should be implemented for both int and bigint in same class
    #[inline]
    pub fn new() -> Self {
        // store at least enough primes for miller test
        let list = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
        PrimeBuffer { list , current: 41 }
    }

    /// Return whether target is a prime. It uses Miller test so even works
    /// for very large numbers and it's very fast
    pub fn is_prime(&self, target: u64) -> bool {
        assert!(target > 1);

        // first find in the prime list
        // TODO: this might be slow if target is quite large
        if target < self.current {
            return self.list.binary_search(&target).is_ok();
        }

        // find 2^shift*u + 1 = n
        let tm1 = target - 1;
        let shift = tm1.trailing_zeros();
        let u = tm1 >> shift;

        let max_a = match target {  // https://oeis.org/A014233
            0..=2047 => 2,
            2048..=1373653 => 3,
            1373654..=25326001 => 5,
            25326002..=3251031751 => 7,
            3251031752..=2152302898747 => 11,
            2152302898748..=3474749660383 => 13,
            3474749660384..=341550071728321 => 17,
            341550071728322..=3825123056546413051 => 23,
            3825123056546413052.. => 37
        };

        'witnessloop: for a in &self.list {
            if *a > max_a { break }

            let mut x = powmod(*a, u, target);
            if x == 1 || x == target - 1 { continue }

            for _ in 0..shift {
                x = mulmod(x, x, target);
                if x == target - 1 {
                    continue 'witnessloop
                }
            }

            if x != 1 {
                return false
            }
        }

        true
    }

    /// Test if a big integer is a prime, this function would carry out a probability test
    pub fn is_bprime(&self, target: &BigUint) -> bool {
        if let Some(x) = target.to_u64() {
            return self.is_prime(x)
        }

        false
    }

    pub fn factors(&mut self, target: u64) -> HashMap<u64, u64> {
        if self.is_prime(target) {
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

    // TODO:
    // pub fn bfactors(&mut self, target: &BigUint) -> HashMap<u64, u64>

    fn factors_naive(&mut self, target: u64) -> HashMap<u64, u64> {
        let mut t = target;
        let mut result: HashMap<u64, u64> = HashMap::new();
        for p in self.primes(sqrt(target) + 1) {
            while t % p == 0 {
                t = t / p;
                *result.entry(*p).or_insert(0) += 1;
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
    /// This function assumes target is not prime
    fn factors_divide(&mut self, target: u64) -> HashMap<u64, u64> {
        // find a proper divisor
        let mut lower_bound = 0;
        let p = loop {
            match self.divisor(target, Some(lower_bound)) {
                Ok(d) => break d,
                Err(l) => lower_bound = l
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
    /// If lower_bound is given, then return a divisor larger than lower_bound
    /// If no divisor is found, then error value could tell that there's no divisor under that number
    pub fn divisor(&mut self, target: u64, lower_bound: Option<u64>) -> Result<u64, u64> {
        let lower_bound = lower_bound.unwrap_or(0);
        const DIVISOR_THRESHOLD: u64 = 1 << 40;
        if target < DIVISOR_THRESHOLD {
            self.divisor_naive(target, lower_bound)
        }
        else {
            match self.divisor_rho(target) {
                Some(d) => Ok(d), None => Err(lower_bound)
            }
        }
    }

    // TODO
    // pub fn bdivisor(&mut self, target: BigUint) -> Result<BigUint, BigUint>

    fn divisor_naive(&mut self, target: u64, lower_bound: u64) -> Result<u64, u64> {
        let upper_bound = sqrt(target) + 1;
        match self.primes(upper_bound).iter()
            .skip_while(|x| **x <= lower_bound)
            .filter(|x| target % *x == 0)
            .next() {
                Some(x) => Ok(*x), None => Err(upper_bound)
            }
    }

    /// Pollard's rho algorithm
    /// http://blog.csdn.net/z690933166/article/details/9865755
    fn divisor_rho(&self, target: u64) -> Option<u64> {
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
            None
        } else {
            Some(p)
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
            let position = match self.list.binary_search(&limit) {
                Ok(p) => p, Err(p) => p
            };
            return &self.list[..position]
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
            self.primes(self.current * (count as u64) / (self.list.len() as u64));
            if self.list.len() >= count {
                break &self.list[..count]
            }
        }
    }

    pub fn clear(&mut self) {
        self.list.truncate(12); // 2 ~ 37
        self.list.shrink_to_fit();
        self.current = 41;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn prime_generation_test(){
        let mut pb = PrimeBuffer::new();
        let prime50 = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
        let prime100 = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
        assert_eq!(pb.primes(50), prime50);
        assert_eq!(pb.primes(100), prime100);
    }
    
    #[test]
    fn prime_assertion_test() {
        let mut pb = PrimeBuffer::new();
        assert!(pb.is_prime(6469693333));
        let prime100 = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
        for x in 2..100 {
            assert_eq!(prime100.contains(&x), pb.is_prime(x));
        }

        let gprimes = pb.primes(10000).to_vec();
        for x in gprimes {
            assert!(pb.is_prime(x));
        }
    }

    #[test]
    fn factorization_test() {
        let mut pb = PrimeBuffer::new();
        let fac123456789 = HashMap::from_iter([(3, 2), (3803, 1), (3607, 1)]);
        let fac = pb.factors(123456789);
        assert_eq!(fac, fac123456789);
    }
}

