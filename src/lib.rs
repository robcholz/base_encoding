mod base64;

use pyo3::exceptions::PyUnicodeError;
use pyo3::prelude::*;

#[pyfunction]
fn base64_encode(binary: String) -> PyResult<String> {
    Ok(base64::base64_encode(binary.into_bytes().as_ref()))
}

#[pyfunction]
fn base64_decode(data: String) -> PyResult<String> {
    let res = base64::base64_decode(data.as_ref());
    if res.is_err() {
        return Err(PyUnicodeError::new_err(res.err().unwrap()));
    }
    Ok(String::from_utf8(res.unwrap())?)
}

#[pymodule]
fn base_encoding(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(base64_encode, m)?)?;
    m.add_function(wrap_pyfunction!(base64_decode, m)?)?;
    Ok(())
}
