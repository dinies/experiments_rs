/*
    KthLargest
You are part of a university admissions office and need to keep track of the
kth highest test score from applicants in real-time. This helps to determine
cut-off marks for interviews and admissions dynamically as new applicants
submit their scores.
You are tasked to implement a class which, for a given integer k, maintains
a stream of test scores and continuously returns the kth highest test score
after a new score has been submitted. More specifically, we are looking for
the kth highest score in the sorted list of all scores.
Implement the KthLargest class:
KthLargest(int k, int[] nums) Initializes the object with the integer k and
the stream of test scores nums.
int add(int val) Adds a new test score val to the stream and returns the
element representing the kth largest element in the pool of test scores so far.

Example 1:
Input:
["KthLargest", "add", "add", "add", "add", "add"]
[[3, [4, 5, 8, 2]], [3], [5], [10], [9], [4]]
Output: [null, 4, 5, 5, 8, 8]

Explanation:
KthLargest kthLargest = new KthLargest(3, [4, 5, 8, 2]);
kthLargest.add(3); // return 4
kthLargest.add(5); // return 5
kthLargest.add(10); // return 5
kthLargest.add(9); // return 8
kthLargest.add(4); // return 8

Example 2:
Input:
["KthLargest", "add", "add", "add", "add"]
[[4, [7, 7, 7, 7, 8, 3]], [2], [10], [9], [9]]
Output: [null, 7, 7, 7, 8]

Explanation:
KthLargest kthLargest = new KthLargest(4, [7, 7, 7, 7, 8, 3]);
kthLargest.add(2); // return 7
kthLargest.add(10); // return 7
kthLargest.add(9); // return 7
kthLargest.add(9); // return 8

Constraints:
0 <= nums.length <= 104
1 <= k <= nums.length + 1
-104 <= nums[i] <= 104
-104 <= val <= 104
At most 104 calls will be made to add.
*/

struct KthLargest {
    ordered_marks: Vec<i32>,
    nth_target: usize,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut copy_nums = nums.clone();
        copy_nums.sort();
        copy_nums.reverse();
        KthLargest {
            ordered_marks: copy_nums,
            nth_target: k as usize,
        }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        let pos = self.ordered_marks.iter().position(|elem| *elem < val);
        match pos {
            Some(index) => {
                self.ordered_marks.insert(index, val);
            }
            None => {
                self.ordered_marks.push(val);
            }
        }
        return *self.ordered_marks.get(self.nth_target - 1).unwrap();
    }
    fn test() {
        let mut obj1 = KthLargest::new(3, vec![4, 5, 8, 2]);
        let calls1: Vec<(i32, i32)> = vec![(3, 4), (5, 5), (10, 5), (9, 8), (4, 8)];

        for (input, output) in calls1 {
            assert_eq!(obj1.add(input), output);
        }

        let mut obj2 = KthLargest::new(4, vec![7, 7, 7, 7, 8, 3]);
        let calls2: Vec<(i32, i32)> = vec![(2, 7), (10, 7), (9, 7), (9, 8)];

        for (input, output) in calls2 {
            assert_eq!(obj2.add(input), output);
        }
    }
}

/*
    StreamChecker
Design an algorithm that accepts a stream of characters and checks if a suffix
of these characters is a string of a given array of strings words.
For example, if words = ["abc", "xyz"] and the stream added the four characters
(one by one) 'a', 'x', 'y', and 'z', your algorithm should detect that the
suffix "xyz" of the characters "axyz" matches "xyz" from words.
Implement the StreamChecker class:
StreamChecker(String[] words) Initializes the object with the strings array words.
boolean query(char letter) Accepts a new character from the stream and returns
true if any non-empty suffix from the stream forms a word that is in words.

Example 1:
Input
["StreamChecker", "query", "query", "query", "query", "query", "query", "query", "query", "query", "query", "query", "query"]
[[["cd", "f", "kl"]], ["a"], ["b"], ["c"], ["d"], ["e"], ["f"], ["g"], ["h"], ["i"], ["j"], ["k"], ["l"]]
Output
[null, false, false, false, true, false, true, false, false, false, false, false, true]
Explanation:
StreamChecker streamChecker = new StreamChecker(["cd", "f", "kl"]);
streamChecker.query("a"); // return False
streamChecker.query("b"); // return False
streamChecker.query("c"); // return False
streamChecker.query("d"); // return True, because 'cd' is in the wordlist
streamChecker.query("e"); // return False
streamChecker.query("f"); // return True, because 'f' is in the wordlist
streamChecker.query("g"); // return False
streamChecker.query("h"); // return False
streamChecker.query("i"); // return False
streamChecker.query("j"); // return False
streamChecker.query("k"); // return False
streamChecker.query("l"); // return True, because 'kl' is in the wordlist

Constraints:
1 <= words.length <= 2000
1 <= words[i].length <= 200
words[i] consists of lowercase English letters.
letter is a lowercase English letter.
At most 4 * 104 calls will be made to query.
*/

trait StreamChecker {
    fn new(words: Vec<String>) -> Self;
    fn query(&mut self, letter: char) -> bool;
}

use std::collections::HashSet;
use std::collections::VecDeque;

struct SlowStreamChecker {
    chars: VecDeque<char>,
    reversed_words: Vec<Vec<char>>,
    num_to_remember: usize,
}

impl StreamChecker for SlowStreamChecker {
    fn new(words: Vec<String>) -> Self {
        let upper_bound_words = 2e3;
        let mut _reversed_words: Vec<Vec<char>> = Vec::with_capacity(upper_bound_words as usize);
        let mut _num_to_remember = 0;
        let words_without_duplicates: HashSet<String> = words.into_iter().collect();
        for word in words_without_duplicates {
            if word.len() > _num_to_remember {
                _num_to_remember = word.len();
            }
            let reversed_word: Vec<char> = word.chars().map(|c| c).rev().collect::<Vec<_>>();
            _reversed_words.push(reversed_word);
        }

        SlowStreamChecker {
            chars: VecDeque::with_capacity(_num_to_remember),
            reversed_words: _reversed_words,
            num_to_remember: _num_to_remember,
        }
    }

    fn query(&mut self, letter: char) -> bool {
        self.chars.push_front(letter);
        if self.chars.len() > self.num_to_remember {
            self.chars.pop_back();
        }
        let mut viable_words_indexes: Vec<usize> = Vec::with_capacity(self.reversed_words.len());

        for i in 0..self.reversed_words.len() {
            viable_words_indexes.push(i);
        }

        let mut curr_letter_position = 0;
        while !viable_words_indexes.is_empty() {
            let mut indexes_to_remove: Vec<usize> = Vec::with_capacity(viable_words_indexes.len());
            if let Some(char_to_match) = self.chars.get(curr_letter_position) {
                for viable_word_idx in viable_words_indexes.iter() {
                    let viable_word: &Vec<char> = &self.reversed_words[*viable_word_idx];
                    if let Some(char_in_word) = viable_word.get(curr_letter_position) {
                        if char_in_word == char_to_match {
                            if viable_word.len() == curr_letter_position + 1 {
                                return true;
                            } else {
                                continue;
                            }
                        }
                    }
                    indexes_to_remove.push(*viable_word_idx);
                }
                viable_words_indexes = viable_words_indexes
                    .into_iter()
                    .filter(|x| !indexes_to_remove.iter().any(|y| y == x))
                    .collect();

                curr_letter_position += 1;
            } else {
                return false;
            }
        }
        return false;
    }
}

fn simple_test<T: StreamChecker>() {
    let mut obj = T::new(vec![
        String::from("cd"),
        String::from("f"),
        String::from("kl"),
    ]);
    assert_eq![obj.query('a'), false];
    assert_eq![obj.query('b'), false];
    assert_eq![obj.query('c'), false];
    assert_eq![obj.query('d'), true]; // because 'cd' is in the wordlist
    assert_eq![obj.query('e'), false];
    assert_eq![obj.query('f'), true]; // because 'f' is in the wordlist
    assert_eq![obj.query('g'), false];
    assert_eq![obj.query('h'), false];
    assert_eq![obj.query('i'), false];
    assert_eq![obj.query('j'), false];
    assert_eq![obj.query('k'), false];
    assert_eq![obj.query('l'), true]; // because 'kl' is in the wordlist
}

fn stress_test_duplicates<T: StreamChecker>() {
    let mut words: Vec<String> = Vec::with_capacity(1664);
    let base: String = String::from("charleetss");
    for _ in 0..64 {
        for l in 'a'..='z' {
            words.push(l.to_string() + &base);
        }
    }
    let queries: Vec<char> = vec!['s'; 40000];
    let mut obj = T::new(words);
    for q in queries.iter() {
        assert_eq![obj.query(*q), false];
    }
}

fn medium_test<T: StreamChecker>() {
    let mut obj = T::new(vec![
        String::from("ab"),
        String::from("ba"),
        String::from("aaab"),
        String::from("abab"),
        String::from("baa"),
    ]);
    assert_eq![obj.query('a'), false];
    assert_eq![obj.query('a'), false];
    assert_eq![obj.query('a'), false];
    assert_eq![obj.query('a'), false];
    assert_eq![obj.query('a'), false];
    assert_eq![obj.query('b'), true];
    assert_eq![obj.query('a'), true];
    assert_eq![obj.query('b'), true];
    assert_eq![obj.query('a'), true];
    assert_eq![obj.query('b'), true];
    assert_eq![obj.query('b'), false];
    assert_eq![obj.query('b'), false];
    assert_eq![obj.query('a'), true];
    assert_eq![obj.query('b'), true];
    assert_eq![obj.query('a'), true];
    assert_eq![obj.query('b'), true];
    assert_eq![obj.query('b'), false];
    assert_eq![obj.query('b'), false];
    assert_eq![obj.query('b'), false];
    assert_eq![obj.query('a'), true];
    assert_eq![obj.query('b'), true];
    assert_eq![obj.query('a'), true];
    assert_eq![obj.query('b'), true];
    assert_eq![obj.query('a'), true];
    assert_eq![obj.query('a'), true];
    assert_eq![obj.query('a'), false];
    assert_eq![obj.query('b'), true];
    assert_eq![obj.query('a'), true];
    assert_eq![obj.query('a'), true];
    assert_eq![obj.query('a'), false];
}

use randomizer::Randomizer;
fn create_stress_input() -> (Vec<String>, Vec<char>) {
    let num_words = 200;
    let num_char_in_word = 200;
    let num_input_chars = 80000;

    let mut words: Vec<String> = Vec::with_capacity(num_words);
    let mut queries: Vec<char> = Vec::with_capacity(num_input_chars);

    for _ in 0..num_words {
        words.push(
            Randomizer::ALPHABETICAL_LOWER(num_char_in_word)
                .string()
                .unwrap(),
        );
    }
    for _ in 0..num_input_chars {
        queries.push(
            Randomizer::ALPHABETICAL_LOWER(1)
                .string()
                .unwrap()
                .chars()
                .next()
                .unwrap(),
        );
    }
    (words, queries)
}

fn stress_test_long_dictionary<T: StreamChecker>() {
    let input: (Vec<String>, Vec<char>) = create_stress_input();
    use std::time::Instant;
    let now = Instant::now();
    let mut obj = T::new(input.0);
    println!("Elapsed after new() : {:.2?}", now.elapsed());
    for q in input.1.into_iter() {
        assert_eq![obj.query(q), false];
    }
    println!("Elapsed after call() : {:.2?}", now.elapsed());
}

impl SlowStreamChecker {
    fn test() {
        simple_test::<SlowStreamChecker>();
        medium_test::<SlowStreamChecker>();
        stress_test_duplicates::<SlowStreamChecker>();
        stress_test_long_dictionary::<SlowStreamChecker>();
    }
}

use std::collections::HashMap;
struct TrieNode {
    children: HashMap<char, Box<TrieNode>>,
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }
}

struct FastStreamChecker {
    trie: TrieNode,
    chars: VecDeque<char>,
    num_to_remember: usize,
}
impl StreamChecker for FastStreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut _num_to_remember = 0;
        let mut _trie = TrieNode::new();
        for word in words.into_iter() {
            if word.len() > _num_to_remember {
                _num_to_remember = word.len();
            }
            Self::add_word(
                &mut _trie,
                word.chars().map(|c| c).rev().collect::<Vec<_>>(),
            );
        }
        FastStreamChecker {
            trie: _trie,
            chars: VecDeque::with_capacity(_num_to_remember),
            num_to_remember: _num_to_remember,
        }
    }

    fn query(&mut self, letter: char) -> bool {
        self.chars.push_front(letter);
        if self.chars.len() > self.num_to_remember {
            self.chars.pop_back();
        }

        let mut cursor: &TrieNode = &self.trie;
        for c in self.chars.iter() {
            if !cursor.children.contains_key(&c) {
                return false;
            }
            cursor = cursor.children.get(&c).unwrap();
            if cursor.is_end_of_word {
                return true;
            }
        }

        return false;
    }
}
impl FastStreamChecker {
    fn add_word(trie: &mut TrieNode, word: Vec<char>) {
        let mut cursor: &mut TrieNode = trie;
        for letter in word.into_iter() {
            if !cursor.children.contains_key(&letter) {
                cursor.children.insert(letter, Box::new(TrieNode::new()));
            }
            cursor = cursor.children.get_mut(&letter).unwrap();
        }
        cursor.is_end_of_word = true;
    }
    fn test() {
        simple_test::<FastStreamChecker>();
        medium_test::<FastStreamChecker>();
        stress_test_duplicates::<FastStreamChecker>();
        stress_test_long_dictionary::<FastStreamChecker>();
    }
}

/*
 SummaryRanges
Given a data stream input of non-negative integers a1, a2, ..., an, summarize
the numbers seen so far as a list of disjoint intervals.

Implement the SummaryRanges class:
SummaryRanges() Initializes the object with an empty stream.
void addNum(int value) Adds the integer value to the stream.
int[][] getIntervals() Returns a summary of the integers in the stream
currently as a list of disjoint intervals [start_i, end_i].
The answer should be sorted by start_i.

Constraints:
0 <= value <= 104
At most 3 * 104 calls will be made to addNum and getIntervals.
At most 102 calls will be made to getIntervals.

Follow up: What if there are lots of merges and the number of disjoint
intervals is small compared to the size of the data stream?
*/
struct SummaryRanges {
    ranges: Vec<Vec<i32>>,
}
impl SummaryRanges {
    fn new() -> Self {
        SummaryRanges {
            ranges: Vec::with_capacity(3 * 104),
        }
    }

    fn cmp_min<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
        if a <= b {
            return a;
        } else {
            return b;
        };
    }

    fn cmp_max<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
        if a >= b {
            return a;
        } else {
            return b;
        };
    }

    fn add_num(&mut self, value: i32) {
        let mut added = false;
        let mut new_ranges = Vec::<Vec<i32>>::with_capacity(self.ranges.len() + 1);
        let mut it = self.ranges.iter().peekable();
        let is_included = |elem: i32, bounds: &Vec<i32>| -> bool {
            return elem >= bounds[0] && elem <= bounds[1];
        };
        let is_just_outside = |elem: i32, bounds: &Vec<i32>| -> bool {
            return elem == bounds[0] - 1 || elem == bounds[1] + 1;
        };
        loop {
            match (it.next(), it.peek(), added) {
                (None, _, false) => {
                    new_ranges.push(vec![value, value]);
                    break;
                }
                (None, _, true) => {
                    break;
                }
                (Some(curr_elem), _, true) => new_ranges.push(curr_elem.to_vec()),
                (Some(curr_elem), next, false) => {
                    if is_included(value, &curr_elem) {
                        new_ranges.push(curr_elem.to_vec());
                        added = true;
                    } else if next != None
                        && is_just_outside(value, &curr_elem)
                        && is_just_outside(value, &next.unwrap())
                    {
                        new_ranges.push(vec![curr_elem[0], next.unwrap()[1]]);
                        added = true;
                        it.next();
                    } else if is_just_outside(value, &curr_elem) {
                        new_ranges.push(vec![
                            Self::cmp_min(curr_elem[0], value),
                            Self::cmp_max(curr_elem[1], value),
                        ]);
                        added = true;
                    } else if value < curr_elem[0] - 1 {
                        new_ranges.push(vec![value, value]);
                        new_ranges.push(curr_elem.to_vec());
                        added = true;
                    } else {
                        new_ranges.push(curr_elem.to_vec());
                    }
                }
            }
        }
        self.ranges = new_ranges;
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.ranges.clone()
    }

    fn test() {
        let mut obj = SummaryRanges::new();
        obj.add_num(1); // arr = [1]
        assert_eq!(obj.get_intervals(), vec![vec![1, 1]]); // return [[1, 1]]
        obj.add_num(3); // arr = [1, 3]
        assert_eq!(obj.get_intervals(), vec![vec![1, 1], vec![3, 3]]); // return [[1, 1], [3, 3]]
        obj.add_num(7); // arr = [1, 3, 7]
        assert_eq!(
            obj.get_intervals(),
            vec![vec![1, 1], vec![3, 3], vec![7, 7]]
        ); // return [[1, 1], [3, 3], [7, 7]]
        obj.add_num(2); // arr = [1, 2, 3, 7]
        assert_eq!(obj.get_intervals(), vec![vec![1, 3], vec![7, 7]]); // return [[1, 3], [7, 7]]
        obj.add_num(6); // arr = [1, 2, 3, 6, 7]
        assert_eq!(obj.get_intervals(), vec![vec![1, 3], vec![6, 7]]); // return [[1, 3], [6, 7]]
    }
}

fn main() {
    println!("KthLargest tests");
    KthLargest::test();
    println!("SlowStreamChecker tests");
    SlowStreamChecker::test();
    println!("FastStreamChecker tests");
    FastStreamChecker::test();
    println!("SummaryRanges tests");
    SummaryRanges::test();
}
