use super::*;

#[test]
fn test_cache() {
    let mut cache = Cache::new(3);

    // add some items to the cache
    cache.put(1, 10);
    cache.put(2, 20);
    cache.put(3, 30);

    // check that we can retrieve the items
    assert_eq!(cache.get(&1), Some(&10));
    assert_eq!(cache.get(&2), Some(&20));
    assert_eq!(cache.get(&3), Some(&30));

    // add a fourth item to the cache, which should remove the least recently used item (1)
    cache.put(4, 40);

    // check that item 1 has been removed from the cache
    assert_eq!(cache.get(&1), None);

    // add a fifth item to the cache, which should remove the least recently used item (2)
    cache.put(5, 50);

    // check that item 2 has been removed from the cache
    assert_eq!(cache.get(&2), None);

    // check that the remaining items are still in the cache
    assert_eq!(cache.get(&3), Some(&30));
    assert_eq!(cache.get(&4), Some(&40));
    assert_eq!(cache.get(&5), Some(&50));
}
