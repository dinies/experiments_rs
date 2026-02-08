use std::collections::HashMap;

struct LRUCache {
    map: HashMap<i32, i32>,
    // Vector that lists the keys used in the map from the oldest to the newest
    keys_order: Vec<i32>,
    max_elements: i32,
}

// Design a data structure that follows the constraints of a Least Recently Used (LRU) cache.
// LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
// int get(int key) Return the value of the key if the key exists, otherwise return -1.
// void put(int key, int value) Update the value of the key if the key exists.
// Otherwise, add the key-value pair to the cache. If the number of keys
// exceeds the capacity from this operation, evict the least recently used key.
// The functions get and put must each run in O(1) average time complexity.
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            map: HashMap::<i32, i32>::new(),
            keys_order: Vec::<i32>::new(),
            max_elements: capacity,
        }
    }

    fn remove_key(keys_order: &mut Vec<i32>, key: i32) {
        for i in 0..keys_order.len() {
            if keys_order[i] == key {
                keys_order.remove(i);
                break;
            }
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let res = self.map.get(&key);
        match res {
            Some(value) => {
                Self::remove_key(&mut self.keys_order, key);
                self.keys_order.push(key);
                return *value;
            }
            None => {
                return -1;
            }
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let insertion = self.map.insert(key, value);
        self.keys_order.push(key);
        match insertion {
            Some(_) => {
                Self::remove_key(&mut self.keys_order, key);
            }
            None => {
                if self.keys_order.len() as i32 > self.max_elements {
                    let oldest_key = self.keys_order.first();
                    match oldest_key {
                        Some(k) => {
                            let cloned_key = k.clone();
                            Self::remove_key(&mut self.keys_order, cloned_key);
                            self.map.remove(&cloned_key);
                        }
                        None => {}
                    }
                }
            }
        }
    }

    pub fn test_lru_cache() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1); // cache is {1=1}
        cache.put(2, 2); // cache is {1=1, 2=2}
        assert_eq!(cache.get(1), 1); // return 1
        cache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
        assert_eq!(cache.get(2), -1); // returns -1 (not found)
        cache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
        assert_eq!(cache.get(1), -1); // return -1 (not found)
        assert_eq!(cache.get(3), 3); // return 3
        assert_eq!(cache.get(4), 4); // return 4
    }
}

fn main() {
    LRUCache::test_lru_cache();
}
