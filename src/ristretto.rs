use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::types::PyList;
use pyo3::types::PyTuple;
use pyo3::exceptions::PyValueError;
use pyo3::basic::CompareOp;
use  pyo3::conversion::IntoPy;
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

    pub fn __sub__(&self, other : &RistrettoPoint) -> RistrettoPoint {
        RistrettoPoint(self.0 - other.0)
    }

    pub fn __neg__(&self) -> RistrettoPoint {
        RistrettoPoint(-self.0)
    }

    // Overriding comparison operators, currently only supporting == and !=
    fn __richcmp__(&self, other: PyRef<RistrettoPoint>, op: CompareOp) -> Py<PyAny> {
        let py = other.py();
        match op {
            CompareOp::Eq => (self.0 == other.0).into_py(py),
            CompareOp::Ne => (self.0 != other.0).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    pub fn compress(&self) -> CompressedRistretto {
        CompressedRistretto(self.0.compress())
    }

    #[staticmethod]
    pub fn from_uniform_bytes(data: &[u8]) -> RistrettoPoint {
        let mut data64 =  [0u8; 64];
        data64.copy_from_slice(&data[..]);
        RistrettoPoint(_RistrettoPoint::from_uniform_bytes(&data64))
    }

    #[staticmethod]
    pub fn from_uniform_bytes_single_elligator(data: &[u8]) -> RistrettoPoint {
        let mut data32 : [u8; 32] = Default::default();
        data32.copy_from_slice(&data[..]);
        RistrettoPoint(
            _RistrettoPoint::from_uniform_bytes_single_elligator(&data32))
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

    pub fn decode_253_bits(&self, py: Python) -> PyObject {
        let ret : (u8, [[u8; 32]; 8]) = self.0.decode_253_bits();
        let one : PyObject = ret.0.into_py(py);
        let two : PyObject = PyList::new(py, [
                PyBytes::new(py,  &ret.1[0]),
                PyBytes::new(py,  &ret.1[1]),
                PyBytes::new(py,  &ret.1[2]),
                PyBytes::new(py,  &ret.1[3]),
                PyBytes::new(py,  &ret.1[4]),
                PyBytes::new(py,  &ret.1[5]),
                PyBytes::new(py,  &ret.1[6]),
                PyBytes::new(py,  &ret.1[7]),
            ]).into();
        PyTuple::new(py, [one, two]).into()
    }
}

#[pymethods]
impl CompressedRistretto {
    #[new]
    pub fn new(data: &[u8]) -> CompressedRistretto {
        CompressedRistretto(_CompressedRistretto::from_slice(data))
    }

    pub fn __bytes__(&self, py: Python) -> PyObject {
        PyBytes::new(py, self.0.as_bytes()).into()
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

pub(crate) fn module(_py: Python<'_>) -> PyResult<(&str, &PyModule)> {
    let name = "ristretto";
    let m = PyModule::new(_py, name)?;
    m.add_class::<RistrettoPoint>()?;
    m.add_class::<CompressedRistretto>()?;
    Ok((name, m))
}
