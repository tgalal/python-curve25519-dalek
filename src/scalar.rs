use pyo3::prelude::*;
use curve25519_dalek::scalar::Scalar as _Scalar;
use crate::ristretto::{RistrettoPoint};

#[pyclass]
pub struct Scalar(pub _Scalar);

#[pymethods]
impl Scalar {
    #[staticmethod]
    pub fn from64(x : u64) -> Scalar {
        Scalar(_Scalar::from(x))
    }

    pub fn __mul__(&self, p : &RistrettoPoint) -> RistrettoPoint {
        RistrettoPoint(self.0 * p.0)
    }
}

pub fn module(_py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(_py, "scalar")?;
    m.add_class::<Scalar>()?;
    Ok(m)
}
