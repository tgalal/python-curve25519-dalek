use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::exceptions::PyValueError;
use sha2::Sha256;
use curve25519_dalek::ristretto::RistrettoPoint as _RistrettoPoint;
use curve25519_dalek::ristretto::CompressedRistretto as _CompressedRistretto;
use curve25519_dalek::scalar::Scalar as _Scalar;

use curve25519_dalek::traits::MultiscalarMul;
use crate::scalar::Scalar;

#[pyclass]
pub struct RistrettoPoint(pub _RistrettoPoint);
#[pyclass]
pub struct CompressedRistretto(pub _CompressedRistretto);

#[pymethods]
impl RistrettoPoint {
    pub fn __mul__(&self, other : &Scalar) -> RistrettoPoint {
        RistrettoPoint(self.0 * other.0)
    }

    pub fn __add__(&self, other : &RistrettoPoint) -> RistrettoPoint {
        RistrettoPoint(self.0 + other.0)
    }

    pub fn compress(&self) -> CompressedRistretto {
        CompressedRistretto(self.0.compress())
    }

    #[staticmethod]
    pub fn lizard_encode_sha256(data: &[u8]) -> RistrettoPoint {
        let mut data16 : [u8; 16] = Default::default();
        data16.copy_from_slice(&data[..]);
        RistrettoPoint(_RistrettoPoint::lizard_encode::<Sha256>(&data16))
    }

    pub fn lizard_decode_sha256(&self, py: Python) -> PyResult<PyObject> {
        match self.0.lizard_decode::<Sha256>() {
            Some(data) => Ok(PyBytes::new(py, &data).into()),
            None => Err(PyValueError::new_err("Decode failed"))
        }
    }

    #[staticmethod]
    pub fn multiscalar_mul(scalars: Vec<PyRef<Scalar>>,
        points: Vec<PyRef<RistrettoPoint>>) -> RistrettoPoint
    {
        let scalars : Vec<_Scalar> = scalars.iter()
            .map(|scalar| scalar.0)
            .collect();
        let points : Vec<_RistrettoPoint> = points.iter()
            .map(|point| point.0)
            .collect();

        RistrettoPoint(
            _RistrettoPoint::multiscalar_mul(scalars, points))
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

    pub fn to_bytes(&self, py: Python) -> PyObject {
        PyBytes::new(py, &self.0.to_bytes()).into()
    }

    pub fn decompress(&self) -> PyResult<RistrettoPoint> {
        let decompressed = self.0.decompress();
        match decompressed {
            Some(point) => Ok(RistrettoPoint(point)),
            None => Err(PyValueError::new_err("Decompress failed"))
        }
    }

    #[staticmethod]
    pub fn from_slice(bytes: &[u8]) -> CompressedRistretto {
        CompressedRistretto(_CompressedRistretto::from_slice(bytes))
    }
}

pub fn module(_py: Python<'_>) -> PyResult<(&str, &PyModule)> {
    let name = "ristretto";
    let m = PyModule::new(_py, name)?;
    m.add_class::<RistrettoPoint>()?;
    m.add_class::<CompressedRistretto>()?;
    Ok((name, m))
}
