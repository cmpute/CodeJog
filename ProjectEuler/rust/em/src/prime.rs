// u64 version of prime related functions
// TODO: improve based on `primal`, `primes` and even `concurrent_prime_sieve` crates
//       main idea is to use bitset for sieving

use std::collections::{HashMap, HashSet};
use num_traits::{ToPrimitive};
use num_bigint::BigUint;
use rand::random;
use crate::traits::Arithmetic;
use crate::int64::integer::*;

pub struct PrimeBuffer {
    list: Vec<u64>, // list of found prime numbers
    current: u64 // all primes smaller than this value has to be in the prime list
}

impl PrimeBuffer {
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

        // Then do a deterministic Miller test
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

        self.list.iter()
            .take_while(|&x| *x <= max_a)
            .all(|&x| target.is_sprp(x))
    }

    /// Test if a big integer is a prime, this function would carry out a probability test
    pub fn is_bprime(&self, target: &BigUint) -> bool {
        if let Some(x) = target.to_u64() {
            return self.is_prime(x)
        }
        // TODO: implement
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
        } else {
            self.factors_divide(target)
        }
    }

    // TODO:
    // pub fn bfactors(&mut self, target: &BigUint) -> Result<HashMap<u64, u64>, Vec<u64>>
    // return list of found factors if not fully factored

    fn factors_naive(&mut self, target: u64) -> HashMap<u64, u64> {
        debug_assert!(!self.is_prime(target));

        let mut t = target;
        let mut result = HashMap::new();
        for &p in self.primes(sqrt(target) + 1) {
            while t % p == 0 {
                t /= p;
                *result.entry(p).or_insert(0) += 1;
            }
            if t == 1 {
                break
            }
        }

        debug_assert_eq!(t, 1); // the target should not be prime
        result
    }

    // Find the factors by dividing the target by a proper divider recursively
    fn factors_divide(&mut self, target: u64) -> HashMap<u64, u64> {
        debug_assert!(!self.is_prime(target));

        let d = self.divisor_rho(target);
        let mut f1 = self.factors(d);
        let f2 = self.factors(target / d);
        for (&factor, &exponent) in &f2 {
            *f1.entry(factor).or_insert(0) += &exponent;
        }
        f1
    }

    /// Return a proper divisor of target (randomly), even works for very large numbers
    /// Return None if it's a prime
    pub fn divisor(&mut self, target: u64) -> Option<u64> {
        if self.is_prime(target) { return None }

        const DIVISOR_THRESHOLD: u64 = 1 << 40;
        if target < DIVISOR_THRESHOLD {
            Some(self.divisor_naive(target))
        } else {
            Some(self.divisor_rho(target))
        }
    }

    // TODO
    // pub fn bdivisor(&mut self, target: BigUint) -> Option<BigUint>

    // Get a factor by naive trials
    fn divisor_naive(&mut self, target: u64) -> u64 {
        debug_assert!(!self.is_prime(target));
        *self.primes(sqrt(target) + 1).iter()
            .filter(|&x| target % x == 0)
            .next().unwrap()
    }

    // Get a factor using pollard_rho
    fn divisor_rho(&self, target: u64) -> u64 {
        debug_assert!(!self.is_prime(target));
        loop {
            let offset = random::<u64>() % target;
            if let Some(p) = target.pollard_rho(offset, 4) {
                break p
            }
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
        self.list.truncate(12); // reserve 2 ~ 37 for miller test
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

