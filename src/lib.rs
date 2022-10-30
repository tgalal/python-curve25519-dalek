use pyo3::prelude::*;
mod ristretto;
mod scalar;
mod constants;

// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "curve25519_dalek")]
fn pycurve25519_dalek(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_submodule(ristretto::module(_py)?)?;
    m.add_submodule(scalar::module(_py)?)?;
    m.add_submodule(constants::module(_py)?)?;

    Ok(())
}
