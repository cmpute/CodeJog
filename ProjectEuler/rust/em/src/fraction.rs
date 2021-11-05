use num_traits::{FromPrimitive, Zero, One};
use num_integer::{Integer, sqrt};
use num_bigint::BigInt;
use std::ops::Neg;
use crate::int64::integer::sqrt as sqrt64;

/// A type representation quadratic surd number (a + b*sqrt(r)) / c
pub struct QuadraticSurd<T> {
    a: T, b: T, c: T, r: T
}

impl<T> QuadraticSurd<T> 
where T: Integer + Neg<Output = T> + FromPrimitive + Clone
{
    pub fn new(a: T, b: T, c: T, r: T) -> Self {
        assert!(r > Zero::zero());

        let (a, b, c) = if c >= Zero::zero() {
            (a, b, c)
        } else {
            (-a, -b, -c)
        };

        let g = a.gcd(&b).gcd(&c);
        QuadraticSurd { r,
            a: a.div_floor(&g),
            b: b.div_floor(&g),
            c: c.div_floor(&g),
        }
    }

    pub fn from_sqrt(target: T) -> Self {
        QuadraticSurd {
            a: FromPrimitive::from_u64(0).unwrap(), 
            b: FromPrimitive::from_u64(1).unwrap(),
            c: FromPrimitive::from_u64(1).unwrap(),
            r: target
        }
    }

    pub fn inverse(self) -> Self {
        QuadraticSurd::new(
            -self.c.clone() * self.a.clone(),
            self.c * self.b.clone(),
            self.b.clone() * self.b * self.r.clone() - self.a.clone() * self.a,
            self.r
        )
    }
}

impl QuadraticSurd<i64> {
    pub fn floor(&self) -> i64 {
        let ar = sqrt64((self.a * self.a * self.r) as u64) as i64;
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
        let ar = sqrt(self.a.clone() * self.a.clone() * self.r.clone());
        let ar = if self.a >= BigInt::zero() { ar }
                 else { -(ar + BigInt::one()) };
        let nom = ar + self.b.clone();
        let nom = if nom >= BigInt::zero() { nom }
                  else {nom - self.c.clone() + 1};
        return nom / self.c.clone();
    }
}
