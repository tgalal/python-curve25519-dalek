use pyo3::prelude::*;
use pyo3::types::PyBytes;
use curve25519_dalek::scalar::Scalar as _Scalar;
use pyo3::basic::CompareOp;

#[pyclass]
pub struct Scalar(pub _Scalar);

#[pymethods]
impl Scalar {
    #[staticmethod]
    pub fn from_u64(x : u64) -> Scalar {
        Scalar(_Scalar::from(x))
    }

    #[staticmethod]
    pub fn from_bytes_mod_order(data: &[u8]) -> Scalar {
        let mut data32 = [0u8; 32];
        data32.copy_from_slice(&data[..]);
        Scalar(_Scalar::from_bytes_mod_order(data32))
    }

    #[staticmethod]
    pub fn from_bytes_mod_order_wide(data: &[u8]) -> Scalar {
        let mut data64 = [0u8; 64];
        data64.copy_from_slice(&data[..]);
        Scalar(_Scalar::from_bytes_mod_order_wide(&data64))
    }

    pub fn __getitem__(&self, p: usize) -> u8 {
        self.0[p]
    }

    pub fn __mul__(&self, p : &Scalar) -> Scalar {
        Scalar(self.0 * p.0)
    }

    pub fn __add__(&self, p : &Scalar) -> Scalar {
        Scalar(self.0 + p.0)
    }

    pub fn __sub__(&self, p : &Scalar) -> Scalar {
        Scalar(self.0 - p.0)
    }

    pub fn __neg__(&self) -> Scalar {
        Scalar(-self.0)
    }

    pub fn invert(&self) -> Scalar {
        Scalar(self.0.invert())
    }

    pub fn __bytes__(&self, py: Python) -> PyObject {
        PyBytes::new(py, self.0.as_bytes()).into()
    }

    // Overriding comparison operators, currently only supporting == and !=
    fn __richcmp__(&self, other: PyRef<Scalar>, op: CompareOp) -> Py<PyAny> {
        let py = other.py();
        match op {
            CompareOp::Eq => (self.0 == other.0).into_py(py),
            CompareOp::Ne => (self.0 != other.0).into_py(py),
            _ => py.NotImplemented(),
        }
    }
}

pub(crate) fn module(_py: Python<'_>) -> PyResult<(&str, &PyModule)> {
    let name = "scalar";
    let m = PyModule::new(_py, name)?;
    m.add_class::<Scalar>()?;
    Ok((name, m))
}
