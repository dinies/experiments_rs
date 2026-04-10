/**
A sequence of numbers is called an arithmetic progression
if the difference between any two consecutive elements is the same.
Given an array of numbers arr, return true if the array can be
rearranged to form an arithmetic progression. Otherwise, return false.

Example 1:
Input: arr = [3,5,1]
Output: true
Explanation: We can reorder the elements as [1,3,5] or [5,3,1]
with differences 2 and -2 respectively, between each consecutive elements.

Example 2:
Input: arr = [1,2,4]
Output: false
Explanation: There is no way to reorder the elements to obtain an
arithmetic progression.

Constraints:
2 <= arr.length <= 1000
-10^6 <= arr[i] <= 10^6
*/

mod arithmetic_progression {
    pub fn is_progression(arr: Vec<i32>) -> bool {
        if arr.len() < 2 {
            return false;
        }
        let mut sorted = arr.clone();
        sorted.sort();
        let difference = sorted[1] - sorted[0];
        let mut iter = sorted.iter().peekable();
        loop {
            match (iter.next(), iter.peek()) {
                (Some(left), Some(right)) => {
                    if *right - *left != difference {
                        return false;
                    }
                }

                (Some(_), None) => {
                    return true;
                }

                _ => {
                    panic!("Supposed to be unreachable");
                }
            }
        }
    }

    pub fn test() {
        assert_eq!(is_progression(vec![3, 5, 1]), true);
        assert_eq!(is_progression(vec![1, 2, 4]), false);
    }
}

/**
Given a positive integer n, find the pivot integer x such that:

The sum of all elements between 1 and x inclusively equals
the sum of all elements between x and n inclusively.
Return the pivot integer x. If no such integer exists, return -1.
It is guaranteed that there will be at most one pivot index for the given input.

Example 1:
Input: n = 8
Output: 6
Explanation: 6 is the pivot integer since: 1 + 2 + 3 + 4 + 5 + 6 = 6 + 7 + 8 = 21.

Example 2:
Input: n = 1
Output: 1
Explanation: 1 is the pivot integer since: 1 = 1.

Example 3:
Input: n = 4
Output: -1
Explanation: It can be proved that no such integer exist.

Constraints:
1 <= n <= 1000
*/

mod pivot {
    pub fn find(n: i32) -> i32 {
        for candidate in (1..=n).into_iter().rev() {
            if (1..=candidate).sum::<i32>() == (candidate..=n).sum::<i32>() {
                return candidate;
            }
        }
        return -1;
    }
    pub fn test() {
        assert_eq!(find(8), 6);
        assert_eq!(find(1), 1);
        assert_eq!(find(4), -1);
    }
}

/**
Given an integer x, return true if x is a palindrome, and false otherwise.

Example 1:
Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left.

Example 2:
Input: x = -121
Output: false
Explanation: From left to right, it reads -121.
From right to left, it becomes 121-. Therefore it is not a palindrome.

Example 3:
Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

Constraints:
-2^31 <= x <= 2^31 - 1

Follow up: Could you solve it without converting the integer to a string?
 */

mod palindrome {
    pub fn check(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut power = 0;
        let mut digits: Vec<i64> = Vec::new();
        loop {
            let divisor = 10i64.pow(power);
            power += 1;
            let magnitude = 10i64.pow(power);
            digits.push(x as i64 % magnitude / divisor);
            if x as i64 / magnitude < 1 {
                break;
            }
        }
        let mut from_left = digits.clone().into_iter();
        let mut from_right = digits.into_iter().rev();
        loop {
            match (from_left.next(), from_right.next()) {
                (None, None) => return true,
                (Some(left), Some(right)) => {
                    if left != right {
                        return false;
                    }
                }
                _ => {
                    panic!("Should be unreachable")
                }
            }
        }
    }
    pub fn test() {
        assert_eq!(check(121), true);
        assert_eq!(check(-121), false);
        assert_eq!(check(10), false);
        assert_eq!(check(1410110141), true);
    }
}

/**
The ugly number

An ugly number is a positive integer which does not have
a prime factor other than 2, 3, and 5.
Given an integer n, return true if n is an ugly number.

Example 1:
Input: n = 6
Output: true
Explanation: 6 = 2 × 3

Example 2:
Input: n = 1
Output: true
Explanation: 1 has no prime factors.

Example 3:
Input: n = 14
Output: false
Explanation: 14 is not ugly since it includes the prime factor 7.

Constraints:
-2^31 <= n <= 2^31 - 1
*/
mod primes {
    pub fn is_ugly(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let mut num_left = n.clone();
        let mut divisors_to_try = vec![5, 3, 2];
        while let Some(div) = divisors_to_try.pop() {
            while num_left % div == 0 {
                num_left /= div;
            }
        }
        return num_left == 1;
    }
    pub fn test() {
        assert_eq!(is_ugly(6), true);
        assert_eq!(is_ugly(1), true);
        assert_eq!(is_ugly(14), false);
    }
}

/**
Smallest Integer Divisible by K
Given a positive integer k, you need to find the length of the
smallest positive integer n such that n is divisible by k,
and n only contains the digit 1.

Return the length of n. If there is no such n, return -1.

Note: n may not fit in a 64-bit signed integer.

Example 1:
Input: k = 1
Output: 1
Explanation: The smallest answer is n = 1, which has length 1.

Example 2:
Input: k = 2
Output: -1
Explanation: There is no such positive integer n divisible by 2.

Example 3:
Input: k = 3
Output: 3
Explanation: The smallest answer is n = 111, which has length 3.

Constraints:
1 <= k <= 10^5
*/

pub mod smallest_divisible {
    use num_traits::{Num, Zero};
    fn cumsum<T: Num + Clone>(vec: Vec<T>) -> Vec<T> {
        let mut result = Vec::<T>::with_capacity(vec.len());
        let mut last_sum: T = Zero::zero();
        for i in 0..vec.len() {
            last_sum = last_sum + vec[i].clone();
            result.push(last_sum.clone());
        }
        result
    }
    fn test_cumsum() {
        let in_out_floats = vec![
            (vec![1.0, 13.0, 4.0], vec![1.0, 14.0, 18.0]),
            (vec![-1.0, 3.5, -2.5], vec![-1.0, 2.5, 0.0]),
            (vec![0.0], vec![0.0]),
            (vec![], vec![]),
        ];
        for (input, output) in in_out_floats {
            assert_eq!(cumsum(input), output)
        }
        let in_out_integers = vec![
            (vec![1, 5, 4, -1], vec![1, 6, 10, 9]),
            (vec![0], vec![0]),
            (vec![], vec![]),
        ];
        for (input, output) in in_out_integers {
            assert_eq!(cumsum(input), output)
        }
    }

    fn modular_pow(b: u64, e: u64, module: u64) -> u64 {
        if module == 1 {
            return 0u64;
        }
        let mut res = 1;
        let mut base = b;
        let mut exp = e;
        base %= module;
        while exp > 0 {
            if exp % 2 == 1 {
                res = (res * base) % module;
            }
            base = (base * base) % module;
            exp /= 2;
        }
        res
    }

    fn test_modular_pow() {
        assert_eq!(modular_pow(10, 23, 29), 11)
    }

    pub fn find(k: i32) -> i32 {
        if k % 2 == 0 || k % 5 == 0 {
            return -1;
        }
        let leftovers: Vec<u64> = (0..k)
            .map(|i| modular_pow(10u64, i as u64, k as u64))
            .collect();

        let csum = cumsum(leftovers);
        let remainders = csum.into_iter().map(|x| x % k as u64).collect::<Vec<_>>();
        let mut found: Option<i32> = None;
        remainders.into_iter().enumerate().for_each(|(idx, x)| {
            if x == 0u64 && found.is_none() {
                found = Some((idx + 1) as i32);
            }
        });
        match found {
            Some(idx) => idx,
            None => -1,
        }
    }
    fn test_find() {
        assert_eq!(find(1), 1);
        assert_eq!(find(2), -1);
        assert_eq!(find(3), 3);
        assert_eq!(find(29), 28);
        assert_eq!(find(9989,), 4278);
    }
    pub fn test() {
        test_cumsum();
        test_modular_pow();
        test_find();
    }
}

#[allow(dead_code)]
fn main() {
    println!("arithmetic_progression");
    arithmetic_progression::test();
    println!("pivot");
    pivot::test();
    println!("palindrome");
    palindrome::test();
    println!("ugly number");
    primes::test();
    println!("smallest_divisible");
    smallest_divisible::test();
}
