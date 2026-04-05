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

/*
   RandomizedCollection is a data structure that contains a collection of
   numbers, possibly duplicates (i.e., a multiset). It should support
   inserting and removing specific elements and also reporting a random element.

   Implement the RandomizedCollection class:

   RandomizedCollection() Initializes the empty RandomizedCollection object.
   bool insert(int val) Inserts an item val into the multiset, even if the
   item is already present. Returns true if the item is not present, false
   otherwise.
   bool remove(int val) Removes an item val from the multiset if present.
   Returns true if the item is present, false otherwise. Note that if val has
   multiple occurrences in the multiset, we only remove one of them.
   int getRandom() Returns a random element from the current multiset of
   elements. The probability of each element being returned is linearly related
   to the number of the same values the multiset contains.
   You must implement the functions of the class such that
   each function works on average O(1) time complexity.

   Note: The test cases are generated such that getRandom will only be called
   if there is at least one item in the RandomizedCollection.

   Constraints:
   -2^31 <= val <= 2^31 - 1
   At most 2 * 10^5 calls in total will be made to insert, remove, and
   getRandom.
   There will be at least one element in the data structure when getRandom is
   called.
*/

use std::collections::HashMap;
struct RandomizedCollection {
    // value -> quantity
    data: HashMap<i32, u16>,
    current_capacity: i32,
}

impl RandomizedCollection {
    fn new() -> Self {
        RandomizedCollection {
            data: HashMap::new(),
            current_capacity: 0,
        }
    }
    fn insert(&mut self, val: i32) -> bool {
        self.current_capacity += 1;
        let mut was_present = false;
        match self.data.get_mut(&val) {
            Some(quantity) => {
                was_present = true;
                *quantity += 1
            }
            None => {
                self.data.insert(val, 1);
            }
        }
        return !was_present;
    }
    fn remove(&mut self, val: i32) -> bool {
        let mut needs_removing = false;
        match self.data.get_mut(&val) {
            Some(quantity) => {
                self.current_capacity -= 1;
                *quantity -= 1;
                if *quantity == 0 {
                    needs_removing = true;
                }
            }
            None => {
                return false;
            }
        }
        if needs_removing {
            self.data.remove(&val);
        }
        return true;
    }
    fn get_random(&self) -> i32 {
        let mut rng = thread_rng();
        let mut decay_counter = rng.gen_range(1i32..=(self.current_capacity));
        for (num, quantity) in self.data.iter() {
            if decay_counter <= *quantity as i32 {
                return *num;
            }
            decay_counter -= *quantity as i32;
        }
        panic!("This part should be unreachable");
    }
    fn test() {
        let mut obj = RandomizedCollection::new();
        assert_eq!(obj.insert(1), true); // return true since the collection does not contain 1.
                                         // Inserts 1 into the collection.
        assert_eq!(obj.insert(1), false); // return false since the collection contains 1.
                                          // Inserts another 1 into the collection. Collection now contains [1,1].
        assert_eq!(obj.insert(2), true); // return true since the collection does not contain 2.
                                         // Inserts 2 into the collection. Collection now contains [1,1,2].
        let res = obj.get_random();
        assert!(res == 1 || res == 2); // getRandom should: return 1 with probability 2/3, or return 2 with probability 1/3.
        assert_eq!(obj.remove(1), true); // return true since the collection contains 1.
                                         // Removes 1 from the collection. Collection now contains [1,2].
        let res = obj.get_random();
        assert!(res == 1 || res == 2); // getRandom should return 1 or 2, both equally likely.
    }
}

/*
   Design a data structure to store the strings' count with the ability to
   return the strings with minimum and maximum counts.

   Implement the AllOne class:
   AllOne() Initializes the object of the data structure.
   inc(String key) Increments the count of the string key by 1. If key does
   not exist in the data structure, insert it with count 1.
   dec(String key) Decrements the count of the string key by 1. If the count
   of key is 0 after the decrement, remove it from the data structure. It is
   guaranteed that key exists in the data structure before the decrement.
   getMaxKey() Returns one of the keys with the maximal count. If no element
   exists, return an empty string "".
   getMinKey() Returns one of the keys with the minimum count. If no element
   exists, return an empty string "".
   Note that each function must run in O(1) average time complexity.

   Constraints:
   1 <= key.length <= 10
   key consists of lowercase English letters.
   It is guaranteed that for each call to dec, key is existing in the data structure.
   At most 5 * 10^4 calls will be made to inc, dec, getMaxKey, and getMinKey.
*/
use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;
use std::rc::Weak;

struct TrieNode {
    parent: RefCell<Weak<TrieNode>>,
    children: RefCell<HashMap<char, Rc<TrieNode>>>,
    counter: RefCell<Option<u16>>,
}

impl TrieNode {
    fn new(node_parent: Weak<TrieNode>) -> Self {
        TrieNode {
            parent: RefCell::new(node_parent),
            children: RefCell::new(HashMap::new()),
            counter: RefCell::new(None),
        }
    }
    fn new_root() -> Self {
        TrieNode {
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(HashMap::new()),
            counter: RefCell::new(None),
        }
    }
    fn change_parent(&self, parent: Rc<TrieNode>) {
        *self.parent.borrow_mut() = Rc::downgrade(&parent);
    }

    fn make_orphan(&self) {
        *self.parent.borrow_mut() = Weak::new();
    }

    fn has_child(&self, x: char) -> bool {
        return self.children.borrow().contains_key(&x);
    }

    fn get_child(&self, x: char) -> Option<Rc<TrieNode>> {
        if let Some(child) = self.children.borrow().get(&x) {
            return Some(Rc::clone(child));
        }
        return None;
    }

    fn get_parent(&self) -> Option<Rc<TrieNode>> {
        return self.parent.borrow().upgrade();
    }

    fn remove_child(&self, label: char) -> bool {
        if self.has_child(label) {
            self.children.borrow_mut().remove(&label);
            return true;
        }
        return false;
    }

    fn add_child(&self, label: char, child: Rc<TrieNode>) -> bool {
        if self.has_child(label) {
            return false;
        } else {
            self.children.borrow_mut().insert(label, child);
            return true;
        }
    }

    fn get_num_children(&self) -> usize {
        self.children.borrow().iter().len()
    }

    fn inc_counter(&self) {
        let mut counter = self.counter.borrow_mut();
        if counter.is_some() {
            *counter = Some(counter.unwrap() + 1u16);
        } else {
            *counter = Some(1u16);
        }
    }

    fn dec_counter(&self) {
        let mut counter = self.counter.borrow_mut();
        if counter.is_some() {
            let num = counter.unwrap();
            if num == 1u16 {
                *counter = None;
            } else {
                *counter = Some(num - 1u16);
            }
        }
    }

    fn get_counter(&self) -> Option<u16> {
        return *self.counter.borrow();
    }

    fn test_basic() {
        let dad = Rc::new(TrieNode::new(Weak::new()));
        let child = Rc::new(TrieNode::new(Rc::downgrade(&dad)));
        dad.children.borrow_mut().insert('c', Rc::clone(&child));
        dad.children.borrow_mut().remove(&'c');
        *child.parent.borrow_mut() = Weak::new();
        {
            let mum = Rc::new(TrieNode::new(Weak::new()));
            *child.parent.borrow_mut() = Rc::downgrade(&mum);
            mum.children.borrow_mut().insert('c', Rc::clone(&child));
        }
    }
    fn test_api() {
        let dad = Rc::new(TrieNode::new_root());
        let child = Rc::new(TrieNode::new(Rc::downgrade(&dad)));
        assert!(dad.add_child('c', Rc::clone(&child)));
        assert!(dad.get_child('c').is_some());
        assert!(dad.remove_child('c'));
        assert!(dad.get_child('c').is_none());
        child.make_orphan();
        {
            let mum = Rc::new(TrieNode::new_root());
            assert_eq!(mum.get_num_children(), 0);
            child.change_parent(Rc::clone(&mum));
            assert!(mum.get_child('c').is_none());
            assert!(mum.add_child('c', Rc::clone(&child)));

            assert_eq!(mum.get_num_children(), 1);
            assert!(mum.get_child('c').is_some());

            child.inc_counter();
            child.inc_counter();
            assert_eq!(child.get_counter(), Some(2u16));
            child.dec_counter();
            child.dec_counter();
            assert_eq!(child.get_counter(), None);
        }
    }
    fn test() {
        TrieNode::test_basic();
        TrieNode::test_api();
    }
}

struct AllOne {
    data: Rc<TrieNode>,
    frequency: LinkedList<(u16, HashSet<String>)>,
}

impl AllOne {
    fn new() -> Self {
        AllOne {
            data: Rc::new(TrieNode::new(Weak::new())),
            frequency: LinkedList::new(),
        }
    }

    fn add_word(trie: Rc<TrieNode>, word: String) -> u16 {
        let mut cursor: Rc<TrieNode> = trie;
        for letter in word.chars().into_iter() {
            if !cursor.has_child(letter) {
                let new_child = Rc::new(TrieNode::new(Rc::downgrade(&cursor)));
                cursor.add_child(letter, new_child);
            }
            cursor = cursor.get_child(letter).unwrap();
        }
        cursor.inc_counter();
        return cursor.get_counter().unwrap();
    }

    fn remove_word(trie: Rc<TrieNode>, word: String) -> Option<u16> {
        let mut cursor: Rc<TrieNode> = trie;
        for letter in word.chars().into_iter() {
            if let Some(child) = cursor.get_child(letter) {
                cursor = child;
            } else {
                return None;
            }
        }
        cursor.dec_counter();
        let frequency = cursor.get_counter();
        let mut removing: Option<char> = None;
        for letter in word.chars().into_iter().rev() {
            if cursor.get_counter().is_none() && cursor.get_num_children() == 0 {
                removing = Some(letter);
                cursor = cursor.get_parent().unwrap();
            } else {
                if removing.is_some() {
                    cursor.remove_child(removing.unwrap());
                }
                return frequency;
            }
        }
        return frequency;
    }

    fn update_frequency(
        frequency_list: &mut LinkedList<(u16, HashSet<String>)>,
        old_freq: u16,
        new_freq: u16,
        key: String,
    ) {
        assert!(old_freq != new_freq);
        let mut inserted_update = false;

        let mut new_frequency_list: LinkedList<(u16, HashSet<String>)> = LinkedList::new();
        for (quantity, set) in frequency_list.into_iter() {
            if *quantity == old_freq && old_freq > 0 {
                set.remove(&key);
                if set.is_empty() {
                    continue;
                }
            } else if *quantity == new_freq && new_freq > 0 {
                set.insert(key.clone());
                inserted_update = true;
            } else if *quantity > new_freq && new_freq > 0 {
                let mut new_set: HashSet<String> = HashSet::new();
                new_set.insert(key.clone());
                new_frequency_list.push_back((new_freq, new_set));
                inserted_update = true;
            } else {
            }
            new_frequency_list.push_back((*quantity, set.clone()));
        }
        if !inserted_update && new_freq > 0 {
            let mut new_set: HashSet<String> = HashSet::new();
            new_set.insert(key.clone());
            new_frequency_list.push_back((new_freq, new_set));
        }
        *frequency_list = new_frequency_list;
    }

    fn inc(&mut self, key: String) {
        let new_freq = Self::add_word(Rc::clone(&self.data), key.clone());
        Self::update_frequency(&mut self.frequency, new_freq - 1, new_freq, key);
    }

    fn dec(&mut self, key: String) {
        let new_freq = Self::remove_word(Rc::clone(&self.data), key.clone());
        match new_freq {
            Some(num) => {
                Self::update_frequency(&mut self.frequency, num + 1, num, key);
            }
            None => {
                Self::update_frequency(&mut self.frequency, 1, 0, key);
            }
        }
    }

    fn get_max_key(&self) -> String {
        match self.frequency.back() {
            Some(elem) => {
                return elem.1.iter().last().unwrap().to_string();
            }
            None => {
                return String::from("");
            }
        }
    }

    fn get_min_key(&self) -> String {
        match self.frequency.front() {
            Some(elem) => {
                return elem.1.iter().last().unwrap().to_string();
            }
            None => {
                return String::from("");
            }
        }
    }

    fn test() {
        let mut obj = AllOne::new();
        obj.inc(String::from("hello"));
        obj.inc(String::from("hello"));
        assert_eq!(obj.get_max_key(), String::from("hello"));
        assert_eq!(obj.get_min_key(), String::from("hello"));
        obj.inc(String::from("leet"));
        assert_eq!(obj.get_max_key(), String::from("hello"));
        assert_eq!(obj.get_min_key(), String::from("leet"));
        obj.dec(String::from("leet"));
        assert_eq!(obj.get_max_key(), String::from("hello"));
        assert_eq!(obj.get_min_key(), String::from("hello"));
    }
}

fn main() {
    println!("RandomizedSet");
    RandomizedSet::test();
    println!("RandomizedCollection");
    RandomizedCollection::test();

    println!("TrieNode");
    TrieNode::test();
    println!("AllOne");
    AllOne::test();
}
