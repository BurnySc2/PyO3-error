use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3::wrap_pyfunction;
use pyo3::PyObjectProtocol;

#[pyclass]
struct MyClass {
    num: i32,
}

#[pymethods]
impl MyClass {
    #[new]
    fn new(num: i32) -> Self {
        MyClass { num }
    }
}

#[pymodule]
fn my_library(_py: Python, m: &PyModule) -> PyResult<()> {
    // Classes to be exported
    m.add_class::<MyClass>()?;
    Ok(())
}
