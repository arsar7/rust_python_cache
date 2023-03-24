use std::collections::{HashMap, VecDeque};

pub struct Cache<K, V>
where
    K: Eq + std::hash::Hash,
{
    data: HashMap<K, V>,
    capacity: usize,
    access_times: VecDeque<K>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
{
    pub fn new(capacity: usize) -> Cache<K, V> {
        Cache {
            data: HashMap::new(),
            capacity: capacity,
            access_times: VecDeque::new(),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
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

    pub fn put(&mut self, key: K, value: V) {
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
