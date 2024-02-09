mod test_para_sum;
mod call_between_rust_and_python;
mod hello_rust_python;
mod hello_pyo3;
mod hello_inline_python;

use test_para_sum::*;
use pyo3::prelude::*;


/// A Python module implemented in Rust.
#[pymodule]
fn pyst(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(para_sum, m)?)?;
    m.add_function(wrap_pyfunction!(no_para_sum, m)?)?;
    // m.add_function(wrap_pyfunction!(sum_rust_python, m)?)?;

    Ok(())
}
