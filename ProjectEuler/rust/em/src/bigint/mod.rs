// bigint version of the math utilities

pub mod integer;

mod fraction {
    use num_bigint::BigInt;
    use crate::fraction::QuadraticSurd as QuadraticSurdGeneric;
    pub type QuadraticSurd = QuadraticSurdGeneric<BigInt>;
}
