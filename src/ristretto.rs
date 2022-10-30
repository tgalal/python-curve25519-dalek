use pyo3::prelude::*;
use pyo3::types::PyBytes;
use curve25519_dalek::ristretto::RistrettoPoint as _RistrettoPoint;
use curve25519_dalek::ristretto::CompressedRistretto as _CompressedRistretto;
use crate::scalar::Scalar;

#[pyclass]
pub struct RistrettoPoint(pub _RistrettoPoint);
#[pyclass]
pub struct CompressedRistretto(pub _CompressedRistretto);

#[pymethods]
impl RistrettoPoint {
    pub fn mul(&self, s : &Scalar) -> RistrettoPoint {
        let p = self.0 * s.0;
        RistrettoPoint(p)
    }

    pub fn compress(&self) -> CompressedRistretto {
        CompressedRistretto(self.0.compress())
    }
}

#[pymethods]
impl CompressedRistretto {
    #[new]
    pub fn new(data: &[u8]) -> CompressedRistretto {
        CompressedRistretto(_CompressedRistretto::from_slice(data))
    }
    pub fn as_bytes(&self, py: Python) -> PyObject {
        PyBytes::new(py, self.0.as_bytes()).into()
    }
}

pub fn module(_py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(_py, "ristretto")?;
    m.add_class::<RistrettoPoint>()?;
    m.add_class::<CompressedRistretto>()?;
    Ok(m)
}

// pub fn setup(m: &PyModule) -> PyResult<()> {
//     m.add_class::<RistrettoPoint>()?;
//     m.add_class::<CompressedRistretto>()?;
//     Ok(())
// }
