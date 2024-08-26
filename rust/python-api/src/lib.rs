use pyo3::prelude::*;
use app::add;

/// Prints a message.
#[pyfunction]
fn hello() -> PyResult<String> {
    Ok(format!("1 + 1 = {}", add(1, 1)))
}

/// A Python module implemented in Rust.
#[pymodule]
fn _lowlevel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    Ok(())
}
