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

    pub fn mul(&self, p : &RistrettoPoint) -> RistrettoPoint {
        let p = self.0 * p.0;
        RistrettoPoint(p)
    }
}

pub fn module(_py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(_py, "scalar")?;
    m.add_class::<Scalar>()?;
    Ok(m)
}
// pub fn setup(m: &PyModule) -> PyResult<()> {
//     m.add_class::<Scalar>()?;
//     Ok(())
// }
