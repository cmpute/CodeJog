use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::{ffi, AsPyPointer, IntoPy};
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyLong, PyType};
use std::os::raw::{c_int, c_uchar};
use std::string::ToString;
use std::convert::TryFrom;

use num_bigint::{BigInt, BigUint, ToBigInt, ToBigUint};
use em::prime::PrimeBuffer as PrimeBuffer64;
use em::{int64, intbig};
use em::fraction as fraction;

// TODO: implement unified API for both int and bigint

enum IntTypes {
    Small(i64),
    Big(BigInt),
}

impl IntoPy<PyObject> for IntTypes {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            Self::Small(val) => val.into_py(py),
            Self::Big(val) => val.into_py(py)
        }
    }
}

impl ToBigInt for IntTypes {
    fn to_bigint(&self) -> Option<BigInt> {
        match self {
            Self::Small(val) => val.to_bigint(),
            Self::Big(val) => val.to_bigint()
        }
    }
}

impl ToBigUint for IntTypes {
    fn to_biguint(&self) -> Option<BigUint> {
        match self {
            Self::Small(val) => val.to_biguint(),
            Self::Big(val) => val.to_biguint()
        }
    }
}

// copied from https://github.com/PyO3/pyo3/blob/main/src/err/mod.rs
unsafe fn pylong_extract(ob: &PyLong, buffer: &mut [c_uchar]) -> PyResult<()> {
    match ffi::_PyLong_AsByteArray(
        ob.as_ptr() as *mut ffi::PyLongObject,
        buffer.as_mut_ptr(),
        buffer.len(),
        1, 1,
    ) {
        -1 => Err(PyErr::fetch(ob.py())),
        _ => Ok(())
    }
}

impl<'source> FromPyObject<'source> for IntTypes {
    fn extract(ob: &'source PyAny) -> PyResult<IntTypes> {
        let py = ob.py();
        unsafe {
            let num: Py<PyLong> = Py::from_owned_ptr_or_err(py, ffi::PyNumber_Index(ob.as_ptr()))?;
            let mut overflow: c_int = 0;
            let v = ffi::PyLong_AsLongLongAndOverflow(num.as_ptr(), &mut overflow);
            if v == -1 && PyErr::occurred(py) {
                Err(PyErr::fetch(py))
            } else if overflow != 0 {
                // copied from https://github.com/PyO3/pyo3/blob/main/src/conversions/num_bigint.rs
                let n_bits = ffi::_PyLong_NumBits(num.as_ptr());
                let n_bytes = if n_bits == -1 {
                    return Err(PyErr::fetch(py));
                } else if n_bits == 0 {
                    0
                } else {
                    (n_bits as usize) / 8 + 1
                };
                let num_big = if n_bytes <= 128 {
                    let mut buffer = [0; 128];
                    pylong_extract(num.as_ref(py), &mut buffer[..n_bytes])?;
                    BigInt::from_signed_bytes_le(&buffer[..n_bytes])
                } else {
                    let mut buffer = vec![0; n_bytes];
                    pylong_extract(num.as_ref(py), &mut buffer)?;
                    BigInt::from_signed_bytes_le(&buffer)
                };
                Ok(IntTypes::Big(num_big))
            } else {
                Ok(IntTypes::Small(v))
            }
        }
    }
}

type QuadraticSurdInt64 = fraction::QuadraticSurd<i64>;
type QuadraticSurdBig = fraction::QuadraticSurd<BigInt>;

enum QuadraticSurdUnified {
    D64(QuadraticSurdInt64),
    DBig(QuadraticSurdBig)
}

#[pyclass]
struct QuadraticSurd { // TODO: store normal version and bigint version in same Python struct
    data: QuadraticSurdUnified
}

#[pymethods]
impl QuadraticSurd {
    #[new]
    fn __new__(a: i64, b: i64, c: i64, r: i64) -> Self {
        QuadraticSurd { data: QuadraticSurdUnified::D64(QuadraticSurdInt64::new(a, b, c, r)) }
    }

    fn __str__(&self) -> PyResult<String> {
        match &self.data {
            QuadraticSurdUnified::D64(d) => Ok(d.to_string()),
            QuadraticSurdUnified::DBig(d) => Ok(d.to_string())
        }
    }

    #[classmethod]
    fn from_sqrt(_cls: &PyType, target: i64) -> PyResult<QuadraticSurd> {
        Ok(QuadraticSurd { data: QuadraticSurdUnified::D64(QuadraticSurdInt64::from_sqrt(target)) })
    }
}

#[pyclass]
struct PrimeBuffer {
    data: PrimeBuffer64
}

#[pymethods]
impl PrimeBuffer {
    #[new]
    fn __new__() -> Self {
        PrimeBuffer { data: PrimeBuffer64::new() }
    }

    fn primes(&mut self, limit: u64) -> PyResult<Vec<u64>> {
        Ok(self.data.primes(limit).to_vec())
    }

    fn nprimes(&mut self, count: usize) -> PyResult<Vec<u64>> {
        Ok(self.data.nprimes(count).to_vec())
    }

    fn is_prime(&self, target: u64) -> PyResult<bool> {
        Ok(self.data.is_prime(target))
    }

    fn factors(&mut self, target: u64) -> PyResult<HashMap<u64, u64>> {
        Ok(self.data.factors(target))
    }

    fn divisor(&mut self, target: u64) -> PyResult<Option<u64>> {
        Ok(self.data.divisor(target))
    }

    fn clear(&mut self) -> PyResult<()> {
        self.data.clear();
        Ok(())
    }
}

/// Returns floor(log(2, target))
#[pyfunction]
fn lb(target: &PyAny) -> PyResult<PyObject> {
    let py = target.py();
    match target.extract()? {
        // TODO: correctly handle the case where target < 0
        IntTypes::Small(v) => Ok(int64::lb(v as u64).into_py(py)),
        IntTypes::Big(v) => Ok(intbig::lb(&v.to_biguint().unwrap()).into_py(py))
    }
}

#[pyfunction]
fn log(target: &PyAny, base: &PyAny) -> PyResult<PyObject> {
    let py = target.py();
    match target.extract()? {
        IntTypes::Small(v) => match base.extract()? {
            IntTypes::Small(b) => Ok(int64::log(v as u64, b as u64).into_py(py)),
            IntTypes::Big(b) => Ok(intbig::log(&v.to_biguint().unwrap(), &b.to_biguint().unwrap()).into_py(py)),
        },
        IntTypes::Big(v) => {
            let uv = v.to_biguint().unwrap();
            let b: IntTypes = base.extract()?;
            let ub = b.to_biguint().unwrap();
            Ok(intbig::log(&uv, &ub).into_py(py))
        }
    }
}

#[pyfunction]
fn sqrt(target: &PyAny) -> PyResult<PyObject> {
    let py = target.py();
    match target.extract()? {
        IntTypes::Small(v) => match u64::try_from(v) {
            Ok(uv) => Ok(int64::sqrt(uv).into_py(py)),
            Err(_) => Err(PyValueError::new_err("math domain error"))
        },
        IntTypes::Big(v) => match v.to_biguint() {
            Some(uv) => Ok(intbig::sqrt(&uv).into_py(py)),
            None => Err(PyValueError::new_err("math domain error"))
        }
    }
}

// // The following function is slower than the one above
// #[pyfunction]
// fn lb_v2(target: IntTypes) -> PyResult<IntTypes> {
//     match target {
//         IntTypes::Small(v) => Ok(IntTypes::Small(int64::lb(v as u64) as i64)),
//         IntTypes::Big(v) => Ok(IntTypes::Small(intbig::lb(&v.to_biguint().unwrap()) as i64))
//     }
// }

/// Math library for Project Euler written in Rust
#[pymodule]
fn em(_py: Python, m: &PyModule) -> PyResult<()> {
    
    m.add_class::<QuadraticSurd>()?;
    m.add_class::<PrimeBuffer>()?;

    m.add_function(wrap_pyfunction!(lb, m)?)?;
    m.add_function(wrap_pyfunction!(log, m)?)?;
    m.add_function(wrap_pyfunction!(sqrt, m)?)?;

    Ok(())
}
