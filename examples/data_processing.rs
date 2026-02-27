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
use std::collections::VecDeque;

struct StreamChecker {
    chars: VecDeque<char>,
    reversed_words: Vec<Vec<char>>,
    num_to_remember: usize,
}

impl StreamChecker {
    pub fn new(words: Vec<String>) -> Self {
        let upper_bound_words = 2e3;
        let mut _reversed_words: Vec<Vec<char>> = Vec::with_capacity(upper_bound_words as usize);
        let mut _num_to_remember = 0;
        for word in words {
            if word.len() > _num_to_remember {
                _num_to_remember = word.len();
            }
            let reversed_word: Vec<char> = word.chars().map(|c| c).rev().collect::<Vec<_>>();
            _reversed_words.push(reversed_word);
        }

        StreamChecker {
            chars: VecDeque::with_capacity(_num_to_remember),
            reversed_words: _reversed_words,
            num_to_remember: _num_to_remember,
        }
    }

    pub fn query(&mut self, letter: char) -> bool {
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
    fn simple_test() {
        let mut obj = StreamChecker::new(vec![
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

    fn stress_test() {
        let mut words: Vec<String> = Vec::with_capacity(1664);
        let base: String = String::from("charleetss");
        for _ in 0..64 {
            for l in 'a'..='z' {
                words.push(l.to_string() + &base);
            }
        }
        let queries: Vec<char> = vec!['s'; 40000];
        let mut obj = StreamChecker::new(words);
        for (i, q) in queries.iter().enumerate() {
            dbg!(i);
            assert_eq![obj.query(*q), false];
        }
    }

    fn test() {
        Self::simple_test();
        Self::stress_test();
    }
}

fn main() {
    KthLargest::test();
    StreamChecker::test();
}
