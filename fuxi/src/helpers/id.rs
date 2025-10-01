use pyo3::pyfunction;

#[pyfunction(name = "new_id")]
#[inline]
pub fn new() -> String {
    uuid::Uuid::now_v7().simple().to_string()
}
