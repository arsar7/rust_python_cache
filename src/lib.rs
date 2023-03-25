use pyo3::prelude::*;
mod cache;

use crate::cache::StringStringCache;

#[pyclass]
struct Cache {
    inner: StringStringCache,
}

#[pymethods]
impl Cache {
    #[new]
    fn new(capacity: usize) -> Self {
        Cache {
            inner: StringStringCache::new(capacity),
        }
    }

    fn get(&mut self, key: &str) -> Option<String> {
        self.inner.get(key).cloned()
    }

    fn put(&mut self, key: String, value: String) {
        self.inner.put(key, value);
    }
}

#[pymodule]
fn rust_cache(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Cache>()?;
    Ok(())
}
