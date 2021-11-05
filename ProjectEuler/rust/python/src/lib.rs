use pyo3::prelude::*;
use em::int64::{integer as int64, prime as prime64};

fn int64(py: Python) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "int64")?;

    /// Returns floor(log(2, target))
    #[pyfunction]
    fn lb(target: u64) -> PyResult<u8> {
        Ok(int64::lb(target))
    }
    m.add_function(wrap_pyfunction!(lb, m)?)?;

    /// Returns floor(log(base, target))
    #[pyfunction]
    fn log(target: u64, base: u64) -> PyResult<u8> {
        Ok(int64::log(target, base))
    }
    m.add_function(wrap_pyfunction!(log, m)?)?;

    /// Returns floor(sqrt(target))
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
    Ok(())
}
