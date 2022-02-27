use num_traits::{FromPrimitive, Zero, One, RefNum, NumRef};
use num_integer::{Integer, sqrt};
use num_bigint::BigInt;
use std::ops::Neg;
use std::fmt;

/// A type representation quadratic surd number (a + b*sqrt(r)) / c
#[derive(PartialEq, Eq, Hash)]
pub struct QuadraticSurd<T> {
    a: T, b: T, c: T, r: T
}

impl<T> QuadraticSurd<T> {
    #[inline]
    pub const fn new_raw(a: T, b: T, c: T, r: T) -> QuadraticSurd<T> {
        QuadraticSurd { a, b, c, r }
    }
}

impl<T> QuadraticSurd<T> 
where T: Integer + NumRef,
// for<'r> &'r T: RefNum<T>
{
    // TODO: add method to reduce root r

    fn reduce(&mut self) {
        if self.c.is_zero() {
            panic!("denominator == 0");
        }

        // reduce common divisor
        let g = self.a.gcd(&self.b).gcd(&self.c);
        self.a = self.a / &g;
        self.b = self.b / &g;
        self.c = self.c / g;

        // keep denom positive
        if self.c < T::zero() {
            self.a = T::zero() - &self.a;
            self.b = T::zero() - &self.b;
            self.c = T::zero() - &self.c;
        }
    }

    pub fn new(a: T, b: T, c: T, r: T) -> Self {
        assert!(r > Zero::zero()); // TODO: it's possible to support r < 0, but we might need complex number

        let mut ret = QuadraticSurd::new_raw(a, b, c, r);
        ret.reduce();
        ret
    }

    pub fn from_sqrt(target: T) -> Self {
        QuadraticSurd {
            a: T::zero(), b: T::one(), c: T::one(), r: target
        }
    }

    pub fn recip(self) -> Self {
        let bb = &self.b * &self.b;
        QuadraticSurd::new(
            T::zero() - &self.c * &self.a,
            &self.c * &self.b,
            &bb * &self.r - &self.a * &self.a,
            self.r
        )
    }
}

impl<T> fmt::Display for QuadraticSurd<T> 
where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} + {}√{}) / {}", self.a, self.b, self.r, self.c)
    }
}

impl QuadraticSurd<i64> {
    pub fn floor(&self) -> i64 {
        let ar = sqrt((self.a * self.a * self.r) as u64) as i64;
        let ar = if self.a >= 0 { ar } else { -(ar + 1) };
        let nom = ar + self.b;
        let nom = if nom >= 0 { nom } else {nom - self.c + 1};
        return nom / self.c;
    }

    pub fn value(&self) -> f64 {
        let fa = self.a as f64;
        let fb = self.b as f64;
        let fc = self.c as f64;
        ((self.r as f64).sqrt() * fa + fb) / fc
    }
}

impl QuadraticSurd<BigInt> {
    pub fn floor(&self) -> BigInt {
        let ar = sqrt(&self.a * &self.a * &self.r);
        let ar = if self.a >= BigInt::zero() { ar }
                 else { -(ar + BigInt::one()) };
        let nom = ar + &self.b;
        let nom = if nom >= BigInt::zero() { nom }
                  else {nom - &self.c + 1};
        return nom / &self.c;
    }
}
