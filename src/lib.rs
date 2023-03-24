use pyo3::prelude::*;
use std::collections::{HashMap, VecDeque};

struct IntIntCache {
    data: HashMap<i32, i32>,
    capacity: usize,
    access_times: VecDeque<i32>,
}

impl IntIntCache {
    fn new(capacity: usize) -> Self {
        IntIntCache {
            data: HashMap::new(),
            capacity: capacity,
            access_times: VecDeque::new(),
        }
    }

    fn get(&mut self, key: &i32) -> Option<&i32> {
        match self.data.get_mut(key) {
            Some(value) => {
                // move the key to the back of the queue
                self.access_times.retain(|k| k != key);
                self.access_times.push_back(key.clone());
                Some(value)
            }
            None => None,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        // check if the cache is full
        if self.data.len() == self.capacity {
            // remove the least recently used key
            if let Some(key_to_remove) = self.access_times.pop_front() {
                self.data.remove(&key_to_remove);
            }
        }

        // add the new key-value pair
        self.data.insert(key.clone(), value);
        self.access_times.push_back(key);
    }
}

#[pyclass]
struct PyIntIntCache {
    inner: IntIntCache,
}

#[pymethods]
impl PyIntIntCache {
    #[new]
    fn new(capacity: usize) -> Self {
        PyIntIntCache {
            inner: IntIntCache::new(capacity),
        }
    }

    fn get(&mut self, key: i32) -> Option<i32> {
        self.inner.get(&key).cloned()
    }

    fn put(&mut self, key: i32, value: i32) {
        self.inner.put(key, value);
    }
}

#[pymodule]
fn rust_cache(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyIntIntCache>()?;
    Ok(())
}
