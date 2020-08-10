use pyo3::prelude::*;

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
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "MyClass ( num: {:?} )",
            self.num
        ))
    }
}

#[pyclass]
struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

#[pymethods]
impl Point3d {
    #[new]
    fn new(x: i32, y: i32, z: i32) -> Self {
        Point3d { x, y, z }
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "Point3d ( x: {:?}, y: {:?}, z: {:?} )",
            self.x, self.y, self.z
        ))
    }
}

#[pymodule]
fn my_library(_py: Python, m: &PyModule) -> PyResult<()> {
    // Classes to be exported
    m.add_class::<MyClass>()?;
    m.add_class::<Point3d>()?;
    Ok(())
}
