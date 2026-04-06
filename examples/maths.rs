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
        return false;
    }
    pub fn test() {
        assert_eq!(is_ugly(6), true);
        assert_eq!(is_ugly(1), true);
        assert_eq!(is_ugly(14), false);
    }
}

fn main() {
    println!("arithmetic_progression");
    arithmetic_progression::test();
    println!("pivot");
    pivot::test();
    println!("palindrome");
    palindrome::test();
    println!("ugly number");
    primes::test();
}
