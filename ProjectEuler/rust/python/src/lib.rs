use pyo3::prelude::*;
use pyo3::{ffi, AsPyPointer, IntoPy};
use pyo3::types::{PyLong};
use std::os::raw::c_int;

use num_bigint::{BigInt, BigUint};
use em::int64::{integer as int64, prime as prime64};
use em::bigint::{integer as intbig};

enum IntTypes {
    Small(i64),
    Big(BigInt),
}

fn parse_int(ob: &PyAny) -> PyResult<IntTypes> {
    let py = ob.py();
    unsafe {
        let num: Py<PyLong> = Py::from_owned_ptr_or_err(py, ffi::PyNumber_Index(ob.as_ptr()))?;
        let mut overflow: c_int = 0;
        let v = ffi::PyLong_AsLongLongAndOverflow(num.as_ptr(), &mut overflow);
        if v == -1 && PyErr::occurred(py) {
            Err(PyErr::fetch(py))
        } else if overflow != 0 {
            let num: BigInt = ob.extract()?;
            Ok(IntTypes::Big(num))
        } else {
            Ok(IntTypes::Small(v))
        }
    }
    // then use ToPyObject
}

fn int64(py: Python) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "int64")?;

    #[pyfunction]
    fn lb(target: u64) -> PyResult<u8> {
        Ok(int64::lb(target))
    }
    m.add_function(wrap_pyfunction!(lb, m)?)?;

    #[pyfunction]
    fn log(target: u64, base: u64) -> PyResult<u8> {
        Ok(int64::log(target, base))
    }
    m.add_function(wrap_pyfunction!(log, m)?)?;

    #[pyfunction]
    fn sqrt(target: u64) -> PyResult<u64> {
        Ok(int64::sqrt(target))
    }
    m.add_function(wrap_pyfunction!(sqrt, m)?)?;

    Ok(m)
}

/// Math library for Project Euler written in Rust
#[pymodule]
fn em(py: Python, m: &PyModule) -> PyResult<()> {
    let m_int64 = int64(py)?;
    let m_bigint = PyModule::new(py, "bigint")?;

    m.add_submodule(m_int64)?;
    m.add_submodule(m_bigint)?;

    /// Returns floor(log(2, target))
    #[pyfunction]
    fn lb(target: &PyAny) -> PyResult<PyObject> {
        match parse_int(target)? {
            IntTypes::Small(v) => Ok(int64::lb(v as u64).into_py(target.py())),
            IntTypes::Big(v) => Ok(intbig::lb(v.to_biguint().unwrap()).into_py(target.py()))
        }
    }
    m.add_function(wrap_pyfunction!(lb, m)?)?;

    Ok(())
}
