use std::collections::HashMap;
use std::collections::VecDeque;

struct ArrayUtils {}
impl ArrayUtils {
    fn remove_key_from_vec(keys_order: &mut Vec<i32>, key: i32) {
        for i in 0..keys_order.len() {
            if keys_order[i] == key {
                keys_order.remove(i);
                break;
            }
        }
    }
    fn remove_key_from_deque(keys_order: &mut VecDeque<i32>, key: i32) {
        for i in 0..keys_order.len() {
            if keys_order[i] == key {
                keys_order.remove(i);
                break;
            }
        }
    }
}

// Design a data structure that follows the constraints of a Least Recently Used (LRU) cache.
// LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
// int get(int key) Return the value of the key if the key exists, otherwise return -1.
// void put(int key, int value) Update the value of the key if the key exists.
// Otherwise, add the key-value pair to the cache. If the number of keys
// exceeds the capacity from this operation, evict the least recently used key.
// The functions get and put must each run in O(1) average time complexity.
struct LRUCache {
    map: HashMap<i32, i32>,
    // Vector that lists the keys used in the map from the oldest to the newest
    keys_order: Vec<i32>,
    max_elements: i32,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            map: HashMap::<i32, i32>::with_capacity(capacity as usize),
            keys_order: Vec::<i32>::with_capacity(capacity as usize),
            max_elements: capacity,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let res = self.map.get(&key);
        match res {
            Some(value) => {
                ArrayUtils::remove_key_from_vec(&mut self.keys_order, key);
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
                ArrayUtils::remove_key_from_vec(&mut self.keys_order, key);
            }
            None => {
                if self.keys_order.len() as i32 > self.max_elements {
                    let oldest_key = self.keys_order.first();
                    match oldest_key {
                        Some(k) => {
                            let cloned_key = k.clone();
                            ArrayUtils::remove_key_from_vec(&mut self.keys_order, cloned_key);
                            self.map.remove(&cloned_key);
                        }
                        None => {}
                    }
                }
            }
        }
    }

    pub fn test() {
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

// Design and implement a data structure for a Least Frequently Used (LFU)
// cache. Implement the LFUCache class:
// LFUCache(int capacity) Initializes the object with the capacity of the
// data structure.
// int get(int key) Gets the value of the key if the key exists in the cache.
// Otherwise, returns -1.
// void put(int key, int value) Update the value of the key if present,
// or inserts the key if not already present. When the cache reaches its
// capacity, it should invalidate and remove the least frequently used
// key before inserting a new item. For this problem, when there is a tie
// (i.e., two or more keys with the same frequency), the least recently
// used key would be invalidated.
// To determine the least frequently used key, a use counter is maintained
// for each key in the cache. The key with the smallest use counter is the
// least frequently used key.
// When a key is first inserted into the cache, its use counter is set to
// 1 (due to the put operation). The use counter for a key in the cache
// is incremented either a get or put operation is called on it.
// The functions get and put must each run in O(1) average time complexity.
struct LFUCache {
    // Map that stores the key value and frequency of use { key -> [value, times_used]}
    map: HashMap<i32, (i32, i32)>,
    // Vector of vectors of keys that stores the frequency of the keys { times_used -> Vec<key> } the inner vectors are ordered with the most recently used key last
    frequency_map: HashMap<i32, VecDeque<i32>>,
    max_elem_num: i32,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        LFUCache {
            map: HashMap::<i32, (i32, i32)>::with_capacity(capacity as usize),
            frequency_map: HashMap::<i32, VecDeque<i32>>::new(),
            max_elem_num: capacity,
        }
    }
    fn update_key_frequency(
        frequency_map: &mut HashMap<i32, VecDeque<i32>>,
        key: &i32,
        times_used: &mut i32,
    ) {
        let previous_times_used: i32 = times_used.clone();
        *times_used += 1;
        if let Some(mut old_layer) = frequency_map.get_mut(&previous_times_used) {
            ArrayUtils::remove_key_from_deque(&mut old_layer, *key);
            if old_layer.is_empty() {
                frequency_map.remove(&previous_times_used);
            }
        }
        match frequency_map.get_mut(times_used) {
            Some(new_layer) => {
                new_layer.push_back(*key);
            }
            None => {
                frequency_map.insert(*times_used, VecDeque::from([*key]));
            }
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.map.get_mut(&key) {
            Some((value, times_used)) => {
                Self::update_key_frequency(&mut self.frequency_map, &key, times_used);
                return *value;
            }
            None => {
                return -1;
            }
        }
    }

    fn remove_least_frequent_element(
        frequency_map: &mut HashMap<i32, VecDeque<i32>>,
    ) -> Option<i32> {
        let mut removed_elem = None;
        if let Some(smallest_key) = frequency_map.keys().min() {
            let smallest_key_clone = smallest_key.clone();
            let elements = frequency_map.get_mut(&smallest_key_clone).unwrap();
            assert!(!elements.is_empty());
            removed_elem = elements.pop_front();
            if elements.is_empty() {
                frequency_map.remove(&smallest_key_clone);
            }
        }
        return removed_elem;
    }

    fn put(&mut self, key: i32, value: i32) {
        match self.map.get_mut(&key) {
            Some((stored_value, times_used)) => {
                Self::update_key_frequency(&mut self.frequency_map, &key, times_used);
                *stored_value = value;
            }
            None => {
                if self.map.len() as i32 >= self.max_elem_num {
                    if let Some(removed_elem) =
                        Self::remove_least_frequent_element(&mut self.frequency_map)
                    {
                        self.map.remove(&removed_elem);
                    }
                }
                let mut new_entry_times_used = 0;
                Self::update_key_frequency(
                    &mut self.frequency_map,
                    &key,
                    &mut new_entry_times_used,
                );
                self.map.insert(key, (value, new_entry_times_used));
            }
        }
    }

    pub fn test() {
        // cnt(x) = the use counter for key x
        // cache=[] will show the last used order for tiebreakers (leftmost element is  most recent)
        let mut cache = LFUCache::new(2);
        cache.put(1, 1); // cache=[1,_], cnt(1)=1
        cache.put(2, 2); // cache=[2,1], cnt(2)=1, cnt(1)=1
        assert_eq!(cache.get(1), 1); // return 1
                                     // cache=[1,2], cnt(2)=1, cnt(1)=2
        cache.put(3, 3); // 2 is the LFU key because cnt(2)=1 is the smallest, invalidate 2.
                         // cache=[3,1], cnt(3)=1, cnt(1)=2
        assert_eq!(cache.get(2), -1); // return -1 (not found)
        assert_eq!(cache.get(3), 3); // return 3
                                     // cache=[3,1], cnt(3)=2, cnt(1)=2
        cache.put(4, 4); // Both 1 and 3 have the same cnt, but 1 is LRU, invalidate 1.
                         // cache=[4,3], cnt(4)=1, cnt(3)=2
        assert_eq!(cache.get(1), -1); // return -1 (not found)
        assert_eq!(cache.get(3), 3); // return 3
                                     // cache=[3,4], cnt(4)=1, cnt(3)=3
        assert_eq!(cache.get(4), 4); // return 4
                                     // cache=[4,3], cnt(4)=2, cnt(3)=3
    }
}

fn main() {
    LRUCache::test();
    LFUCache::test();
}
