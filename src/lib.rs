use pyo3::prelude::*;
use pyo3::types::PyDict;
mod ristretto;
mod scalar;
mod constants;

// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }
//

fn make_submodule(parent: &PyModule, subinfo: (&str, &PyModule),
    sys_modules: &PyDict) -> PyResult<()> {

    let (name, submodule) = subinfo;

    parent.add_submodule(submodule)?;
    sys_modules.set_item(format!("curve25519_dalek.{}", name),
        parent.getattr(name)?)?;
    Ok(())
}

// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "curve25519_dalek")]
fn pycurve25519_dalek(_py: Python, m: &PyModule) -> PyResult<()> {
    let sys = PyModule::import(_py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;

    make_submodule(m, ristretto::module(_py)?, sys_modules)?;
    make_submodule(m, scalar::module(_py)?, sys_modules)?;
    make_submodule(m, constants::module(_py)?, sys_modules)?;


    Ok(())
}
