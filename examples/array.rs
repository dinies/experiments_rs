//Letcoode problems on arrays

use std::collections::HashMap;

use rand::random;

struct Array {}

impl Array {
    //Given an array of integers nums and an integer target, return indices
    //of the two numbers such that they add up to target.
    // You may assume that each input would have exactly one solution, and you
    // may not use the same element twice. You can return the answer in any order.
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = Vec::<i32>::new();
        //build a map { value -> [vec_indexes] }
        let mut map = HashMap::<i32, Vec<i32>>::new();
        for (idx, &val) in nums.iter().enumerate() {
            let curr_idx: i32 = idx.try_into().unwrap();
            if map.contains_key(&val) {
                if let Some(x) = map.get_mut(&val) {
                    x.push(curr_idx);
                };
            } else {
                map.insert(val, vec![curr_idx]);
            }
        }
        for (value, vec_indexes) in &map {
            let other_value = target - value;
            if *value == other_value && vec_indexes.len() == 2 {
                result.push(vec_indexes[0]);
                result.push(vec_indexes[1]);
                break;
            }
            let new_element = map.get(&other_value);
            match new_element {
                Some(new_value) => {
                    let first_idx = *vec_indexes.first().unwrap();
                    let second_idx = *new_value.first().unwrap();
                    if first_idx != second_idx {
                        result.push(first_idx);
                        result.push(second_idx);
                        break;
                    }
                }
                None => continue,
            }
        }
        return result;
    }

    pub fn test_two_sum() {
        {
            let n1 = vec![2, 7, 11, 15];
            let t1 = 9;
            let y1 = vec![0, 1];
            let r1 = Self::two_sum(n1, t1);
            assert!(r1.eq(&y1) || r1.eq(&y1.into_iter().rev().collect::<Vec<_>>()));
        }
        {
            let n2 = vec![3, 2, 4];
            let t2 = 6;
            let y2 = vec![1, 2];
            let r2 = Self::two_sum(n2, t2);
            assert!(r2.eq(&y2) || r2.eq(&y2.into_iter().rev().collect::<Vec<_>>()));
        }
        {
            let n3 = vec![3, 3];
            let t3 = 6;
            let y3 = vec![0, 1];
            let r3 = Self::two_sum(n3, t3);
            assert!(r3.eq(&y3) || r3.eq(&y3.into_iter().rev().collect::<Vec<_>>()));
        }
    }

    // Given an integer array nums sorted in non-decreasing order, remove the
    // duplicates in-place such that each unique element appears only once.
    // The relative order of the elements should be kept the same.
    // Consider the number of unique elements in nums to be k. After removing
    // duplicates, return the number of unique elements k.
    // The first k elements of nums should contain the unique numbers in sorted
    // order. The remaining elements beyond index k - 1 can be ignored.
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut distance = nums.len();
        let mut i = 0;
        while i < distance {
            if i > 0 && nums[i] == nums[i - 1] {
                nums.remove(i);
                distance -= 1;
            } else {
                i += 1;
            }
        }
        return distance as i32;
    }
    pub fn test_remove_duplicates() {
        {
            let mut n1 = vec![1, 1, 2];
            let y1 = vec![1, 2, random()];
            let z1 = 2;
            let r1 = Self::remove_duplicates(&mut n1);

            assert_eq!(r1, z1);
            for i in 0..z1 as usize {
                assert_eq!(n1[i], y1[i]);
            }
        }
        {
            let mut n2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
            let y2 = vec![
                0,
                1,
                2,
                3,
                4,
                random(),
                random(),
                random(),
                random(),
                random(),
                random(),
            ];
            let z2 = 5;
            let r2 = Self::remove_duplicates(&mut n2);

            assert_eq!(r2, z2);
            for i in 0..z2 as usize {
                assert_eq!(n2[i], y2[i]);
            }
        }
    }

    // Given an integer numRows, return the first numRows of Pascal's triangle.
    // In Pascal's triangle, each number is the sum of the two numbers directly above
    fn generate_pascal_triangle(num_rows: i32) -> Vec<Vec<i32>> {
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
    pub fn test_generate_pascal_triangle() {
        {
            let n1 = 5;
            let y1 = vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
            ];
            assert_eq!(Self::generate_pascal_triangle(n1), y1);
        }
        {
            let n2 = 1;
            let y2 = vec![vec![1]];
            assert_eq!(Self::generate_pascal_triangle(n2), y2);
        }
    }
    // Given a non-negative integer x, return the square root of x rounded down
    // to the nearest integer. The returned integer should be non-negative as well.
    // You must not use any built-in exponent function or operator.
    fn square_root(x: i32) -> i32 {
        match x {
            1 => {
                return 1;
            }
            2.. => {
                let mut lower_bound = 0i64;
                let mut upper_bound = x as i64;
                while upper_bound - lower_bound > 1 {
                    let pivot: i64 = lower_bound + (upper_bound - lower_bound) / 2;
                    match x as i64 - pivot * pivot {
                        0 => {
                            return pivot as i32;
                        }
                        ..=-1 => {
                            upper_bound = pivot;
                        }
                        1.. => {
                            lower_bound = pivot;
                        }
                    }
                }
                return lower_bound as i32;
            }
            _ => return 0,
        }
    }
    pub fn test_square_root() {
        {
            let x1 = 4;
            let y1 = 2;
            assert_eq!(y1, Self::square_root(x1));
        }
        {
            let x2 = 8;
            let y2 = 2;
            assert_eq!(y2, Self::square_root(x2));
        }
        {
            let x3 = 2147395599;
            let y3 = 46339;
            assert_eq!(y3, Self::square_root(x3));
        }
    }
}

fn main() {
    Array::test_two_sum();
    Array::test_remove_duplicates();
    Array::test_generate_pascal_triangle();
    Array::test_square_root();
}
