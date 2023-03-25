use pyo3::prelude::*;
mod cache;

use crate::cache::StringCache;


#[pyclass]
struct Cache {
    inner: StringCache,
}

#[pymethods]
impl Cache {
    #[new]
    fn new(capacity: usize) -> Self {
        Cache {
            inner: StringCache::new(capacity),
        }
    }

    fn get(&mut self, key: &str) -> Option<String> {
        self.inner.get(key).cloned()
    }

    fn put(&mut self, key: String, value: String) {
        self.inner.put(key, value);
    }

    fn len(&mut self) -> usize {
        self.inner.data.len()
    }

    fn capacity(&mut self) -> usize {
        self.inner.capacity
    }

}

#[pymodule]
fn rust_cache(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Cache>()?;
    Ok(())
}
