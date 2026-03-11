/*
Implement the RandomizedSet class:

RandomizedSet() Initializes the RandomizedSet object.
bool insert(int val) Inserts an item val into the set if not present.
Returns true if the item was not present, false otherwise.
bool remove(int val) Removes an item val from the set if present.
Returns true if the item was present, false otherwise.
int getRandom() Returns a random element from the current set of elements
(it's guaranteed that at least one element exists when this method is called).
Each element must have the same probability of being returned.
You must implement the functions of the class such that each
function works in average O(1) time complexity.

Constraints:

-2^31 <= val <= 2^31 - 1
At most 2 * 10^5 calls will be made to insert, remove, and getRandom.
There will be at least one element in the data structure when getRandom is called.
*/

use rand::{thread_rng, Rng};
use std::collections::HashSet;
struct RandomizedSet {
    data: HashSet<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            data: HashSet::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.data.contains(&val) {
            return false;
        }
        self.data.insert(val);
        return true;
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.data.contains(&val) {
            return false;
        }
        self.data.remove(&val);
        return true;
    }

    fn get_random(&self) -> i32 {
        let mut rng = thread_rng();
        let curr_numbers: Vec<&i32> = self.data.iter().collect();
        let random_idx: i32 = rng.gen_range(0i32..(curr_numbers.len() as i32));
        return *curr_numbers[random_idx as usize];
    }

    fn test() {
        let mut obj = RandomizedSet::new();
        assert_eq!(obj.insert(1), true); // Inserts 1 to the set, returns true. Set now contains [1].

        assert_eq!(obj.remove(2), false); // Returns false as 2 does not exist in the set.
        assert_eq!(obj.insert(2), true); // Inserts 2 to the set, returns true. Set now contains [1,2].
        let res = obj.get_random();
        assert!(res == 1 || res == 2); // getRandom() should return either 1 or 2 randomly.
        assert_eq!(obj.remove(1), true); // Removes 1 from the set, returns true. Set now contains [2].
        assert_eq!(obj.insert(2), false); // 2 was already in the set, so return false.
        assert_eq!(obj.get_random(), 2); // Since 2 is the only number in the set, getRandom() will always return 2.
    }
}

fn main() {
    println!("RandomizedSet");
    RandomizedSet::test();
}
