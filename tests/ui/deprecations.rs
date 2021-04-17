#![deny(deprecated)]

use pyo3::prelude::*;

#[pyclass]
struct TestClass {
    num: u32,
}

#[pymethods]
impl TestClass {
    #[classattr]
    #[name = "num"]
    const DEPRECATED_NAME_CONSTANT: i32 = 0;

    #[name = "num"]
    fn deprecated_name_pymethod(&self) { }

    #[staticmethod]
    #[name = "custom_static"]
    fn deprecated_name_staticmethod() {}
}

#[pyfunction]
#[name = "foo"]
fn deprecated_name_pyfunction() { }

fn main() {

}


// TODO: ensure name deprecated on #[pyfunction] and #[pymodule]
