// u64 version of the math utilities

pub mod integer;

pub mod fraction {
    use crate::fraction::QuadraticSurd as QuadraticSurdGeneric;
    pub type QuadraticSurd = QuadraticSurdGeneric<i64>;
}
