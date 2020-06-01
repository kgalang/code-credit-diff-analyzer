use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
/// Formats the sum of two numbers as string
fn analyze(a: &str) -> PyResult<String> {
    Ok(a.to_string())
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn code_credit_diff(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(analyze))?;

    Ok(())
}
