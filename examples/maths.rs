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
mod core {
    use num_traits::{FromPrimitive, Unsigned};
    pub fn split_into_digits<T: Unsigned + FromPrimitive + Copy + std::cmp::PartialOrd>(
        number: T,
    ) -> Vec<T> {
        let mut power = 0;
        let mut digits: Vec<T> = Vec::new();
        loop {
            let divisor = T::from_u64(10u64.pow(power)).unwrap();
            power += 1;
            let magnitude = T::from_u64(10u64.pow(power)).unwrap();
            digits.push(number % magnitude / divisor);
            if number / magnitude < T::one() {
                break;
            }
        }
        digits
    }
}

mod palindrome {
    use super::core::split_into_digits;
    pub fn check(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let digits: Vec<u64> = split_into_digits(u64::try_from(x).unwrap());
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
/** Self-dividing numbers
A self-dividing number is a number that is divisible by every digit it contains.

For example, 128 is a self-dividing number because 128 % 1 == 0,
128 % 2 == 0, and 128 % 8 == 0.
A self-dividing number is not allowed to contain the digit zero.

Given two integers left and right, return a list of all the
self-dividing numbers in the range [left, right] (both inclusive).

Example 1:
Input: left = 1, right = 22
Output: [1,2,3,4,5,6,7,8,9,11,12,15,22]

Example 2:
Input: left = 47, right = 85
Output: [48,55,66,77]

Constraints:
1 <= left <= right <= 104
*/

mod self_dividing_numbers {
    use super::core::split_into_digits;
    fn is_self_dividing(n: i32) -> bool {
        let digits: Vec<u32> =
            split_into_digits(u32::try_from(if n < 0 { -n } else { n }).unwrap());
        let mut it = digits.into_iter();
        while let Some(digit) = it.next() {
            if digit == 0 || n % digit as i32 != 0 {
                return false;
            }
        }
        return true;
    }
    pub fn find_in_range(left: i32, right: i32) -> Vec<i32> {
        (left..=right).filter(|x| is_self_dividing(*x)).collect()
    }
    fn test_is_self_dividing() {
        assert_eq!(is_self_dividing(128), true)
    }
    fn test_find_in_range() {
        assert_eq!(
            find_in_range(1, 22),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
        );
        assert_eq!(find_in_range(47, 85), vec![48, 55, 66, 77]);
    }
    pub fn test() {
        test_is_self_dividing();
        test_find_in_range();
    }
}

/* Permutation Sequence
The set [1, 2, 3, ..., n] contains a total of n! unique permutations.
By listing and labeling all of the permutations in order, we get the
following sequence for n = 3:

"123"
"132"
"213"
"231"
"312"
"321"
Given n and k, return the kth permutation sequence.

Example 1:
Input: n = 3, k = 3
Output: "213"

Example 2:
Input: n = 4, k = 9
Output: "2314"

Example 3:
Input: n = 3, k = 1
Output: "123"

Constraints:
1 <= n <= 9
1 <= k <= n!
*/
mod permutation_sequence {
    fn factorial(n: i32) -> Option<i32> {
        if n < 0 {
            return None;
        }
        if n == 0 {
            return Some(1);
        }
        let mut curr_n = n;
        let mut result = 1;
        while curr_n > 0 {
            result *= curr_n;
            curr_n -= 1;
        }
        Some(result)
    }
    fn test_factorial() {
        let in_out = vec![
            (-1, None),
            (0, Some(1)),
            (1, Some(1)),
            (2, Some(2)),
            (3, Some(6)),
            (4, Some(24)),
            (5, Some(120)),
            (6, Some(720)),
            (7, Some(5040)),
        ];
        in_out
            .into_iter()
            .for_each(|(input, output)| assert_eq!(factorial(input), output));
    }
    pub fn get(n: i32, k: i32) -> String {
        let mut current_perm: i32 = 0;
        let mut stack: Vec<(Vec<i32>, Vec<i32>)> = vec![(Vec::new(), (1..=n).collect())];
        while let Some(nums) = stack.pop() {
            let inner_permutations_num = factorial(nums.1.len() as i32 - 1).unwrap();
            for (outer_idx, &val) in nums.1.iter().enumerate() {
                if current_perm + inner_permutations_num >= k {
                    let mut curr_base = nums.0.clone();
                    curr_base.push(val);
                    if nums.1.len() == 1 && current_perm + inner_permutations_num == k {
                        return curr_base
                            .into_iter()
                            .map(|number| char::from_digit(number as u32, 10).unwrap())
                            .collect();
                    }
                    stack.push((
                        curr_base,
                        nums.1
                            .clone()
                            .into_iter()
                            .enumerate()
                            .map(|(i, elem)| (i, elem))
                            .filter(|(inner_idx, _)| *inner_idx != outer_idx)
                            .map(|(_, elem)| elem)
                            .collect(),
                    ));
                    break;
                } else {
                    current_perm += inner_permutations_num;
                }
            }
        }
        String::from("")
    }
    pub fn test_get() {
        assert_eq!(get(3, 3), String::from("213"));
        assert_eq!(get(4, 9), String::from("2314"));
        assert_eq!(get(3, 1), String::from("123"));
    }
    pub fn test() {
        test_factorial();
        test_get();
    }
}

/* Pascal's triangle
 Given an integer numRows, return the first numRows of Pascal's triangle.
In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:

Example 1:
Input: numRows = 5
Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]

Example 2:
Input: numRows = 1
Output: [[1]]

Constraints:
1 <= numRows <= 30
*/
mod pascal_triangle {
    fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle = Vec::<Vec<i32>>::new();
        match num_rows {
            0 => {}
            1 => {
                triangle.push(vec![1]);
            }
            2.. => {
                triangle.push(vec![1]);
                for _ in 2..=num_rows {
                    let mut last_row: Vec<i32> = triangle
                        .last()
                        .expect("There is always an element in the vector at this point")
                        .clone();
                    last_row.insert(0, 0);
                    last_row.push(0);
                    let new_dim = last_row.len() - 1;
                    let mut new_row = Vec::<i32>::with_capacity(new_dim);
                    for i in 0..new_dim {
                        new_row.push(last_row[i] + last_row[i + 1]);
                    }
                    triangle.push(new_row);
                }
            }
            _ => {}
        }
        return triangle;
    }
    pub fn test() {
        {
            let n1 = 5;
            let y1 = vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
            ];
            assert_eq!(generate(n1), y1);
        }
        {
            let n2 = 1;
            let y2 = vec![vec![1]];
            assert_eq!(generate(n2), y2);
        }
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
    println!("self_dividing");
    self_dividing_numbers::test();
    println!("permutation_sequence");
    permutation_sequence::test();
    println!("pascal_triangle");
    pascal_triangle::test();
}
