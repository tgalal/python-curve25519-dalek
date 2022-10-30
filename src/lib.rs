use pyo3::prelude::*;
use pyo3::types::PyBytes;    
//use pyo3::types::PyByteArray;    
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::ristretto::RistrettoPoint as _RistrettoPoint;
use curve25519_dalek::ristretto::CompressedRistretto as _CompressedRistretto;
use curve25519_dalek::scalar::Scalar as _Scalar;
use curve25519_dalek::field::FieldElement;
// use curve25519_dalek::traits::MultiscalarMul;


#[pyclass]
struct RistrettoPoint(_RistrettoPoint);
#[pyclass]
struct CompressedRistretto(_CompressedRistretto);
#[pyclass]
struct Scalar(_Scalar);

#[pymethods]
impl RistrettoPoint {
    fn mul(&self, s : &Scalar) -> RistrettoPoint {
        let p = self.0 * s.0;
        RistrettoPoint(p)
    }

    fn compress(&self) -> CompressedRistretto {
        CompressedRistretto(self.0.compress())
    }
}

#[pymethods]
impl CompressedRistretto {
    #[new]
    fn new(data: &[u8]) -> CompressedRistretto {
        CompressedRistretto(_CompressedRistretto::from_slice(data))
    }
    fn as_bytes(&self, py: Python) -> PyObject {
        PyBytes::new(py, self.0.as_bytes()).into()
    }
}

#[pymethods]
impl Scalar {
    #[staticmethod]
    fn from64(x : u64) -> Scalar {
        Scalar(_Scalar::from(x))
    }
    fn mul(&self, p : &RistrettoPoint) -> RistrettoPoint {
        let p = self.0 * p.0;
        RistrettoPoint(p)
    }
}

const BASEPOINT: RistrettoPoint = RistrettoPoint(RISTRETTO_BASEPOINT_POINT);

// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

// A Python module implemented in Rust.
#[pymodule]
fn pycurve25519_dalek(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    // m.add_class::<Person>()?;
    m.add_class::<Scalar>()?;
    m.add_class::<RistrettoPoint>()?;
    m.add_class::<CompressedRistretto>()?;
    m.add("RISTRETTO_BASEPOINT_POINT", BASEPOINT)?;
    Ok(())
}
