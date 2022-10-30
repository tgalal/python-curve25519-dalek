use pyo3::prelude::*;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT as RBP;
use crate::ristretto::RistrettoPoint;

pub const RISTRETTO_BASEPOINT_POINT: RistrettoPoint = RistrettoPoint(RBP);

// pub fn setup(m: &PyModule) -> PyResult<()> {
//     m.add("RISTRETTO_BASEPOINT_POINT", RISTRETTO_BASEPOINT_POINT)?;
//     Ok(())
// }

pub fn module(_py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(_py, "constants")?;
    m.add("RISTRETTO_BASEPOINT_POINT", RISTRETTO_BASEPOINT_POINT)?;
    Ok(m)
}
