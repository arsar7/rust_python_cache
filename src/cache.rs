use std::collections::{HashMap, VecDeque};

pub struct StringStringCache {
    data: HashMap<String, String>,
    capacity: usize,
    access_times: VecDeque<String>,
}

impl StringStringCache {
    pub fn new(capacity: usize) -> Self {
        StringStringCache {
            data: HashMap::new(),
            capacity: capacity,
            access_times: VecDeque::new(),
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&String> {
        match self.data.get_mut(key) {
            Some(value) => {
                // move the key to the back of the queue
                self.access_times.retain(|k| k != key);
                self.access_times.push_back(key.to_string());
                Some(value)
            }
            None => None,
        }
    }

    pub fn put(&mut self, key: String, value: String) {
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


#[cfg(test)]
fn test_string_values() {

    let mut cache = StringStringCache::new(3);

    // add some items to the cache
    cache.put("a".to_string(), "apple".to_string());
    cache.put("b".to_string(), "banana".to_string());
    cache.put("c".to_string(), "cherry".to_string());

    // check that we can retrieve the items
    assert_eq!(cache.get("a"), Some(&"apple".to_string()));
    assert_eq!(cache.get("b"), Some(&"banana".to_string()));
    assert_eq!(cache.get("c"), Some(&"cherry".to_string()));

    // add a fourth item to the cache, which should remove the least recently used item ("a")
    cache.put("d".to_string(), "date".to_string());

    // check that item "a" has been removed from the cache
    assert_eq!(cache.get("a"), None);

    // add a fifth item to the cache, which should remove the least recently used item ("b")
    cache.put("e".to_string(), "elderberry".to_string());

    // check that item "b" has been removed from the cache
    assert_eq!(cache.get("b"), None);

    // check that the remaining items are still in the cache
    assert_eq!(cache.get("c"), Some(&"cherry".to_string()));
    assert_eq!(cache.get("d"), Some(&"date".to_string()));
    assert_eq!(cache.get("e"), Some(&"elderberry".to_string()));


}
