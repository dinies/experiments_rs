//Letcoode problems on arrays

mod array_operations {
    use std::collections::HashMap;

    //Given an array of integers nums and an integer target, return indices
    //of the two numbers such that they add up to target.
    // You may assume that each input would have exactly one solution, and you
    // may not use the same element twice. You can return the answer in any order.
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
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
            let r1 = two_sum(n1, t1);
            assert!(r1.eq(&y1) || r1.eq(&y1.into_iter().rev().collect::<Vec<_>>()));
        }
        {
            let n2 = vec![3, 2, 4];
            let t2 = 6;
            let y2 = vec![1, 2];
            let r2 = two_sum(n2, t2);
            assert!(r2.eq(&y2) || r2.eq(&y2.into_iter().rev().collect::<Vec<_>>()));
        }
        {
            let n3 = vec![3, 3];
            let t3 = 6;
            let y3 = vec![0, 1];
            let r3 = two_sum(n3, t3);
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
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
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
    use rand::random;
    pub fn test_remove_duplicates() {
        {
            let mut n1 = vec![1, 1, 2];
            let y1 = vec![1, 2, random()];
            let z1 = 2;
            let r1 = remove_duplicates(&mut n1);

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
            let r2 = remove_duplicates(&mut n2);

            assert_eq!(r2, z2);
            for i in 0..z2 as usize {
                assert_eq!(n2[i], y2[i]);
            }
        }
    }

    // Given a non-negative integer x, return the square root of x rounded down
    // to the nearest integer. The returned integer should be non-negative as well.
    // You must not use any built-in exponent function or operator.
    pub fn square_root(x: i32) -> i32 {
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
            assert_eq!(y1, square_root(x1));
        }
        {
            let x2 = 8;
            let y2 = 2;
            assert_eq!(y2, square_root(x2));
        }
        {
            let x3 = 2147395599;
            let y3 = 46339;
            assert_eq!(y3, square_root(x3));
        }
    }
}

fn main() {
    array_operations::test_two_sum();
    array_operations::test_remove_duplicates();
    array_operations::test_square_root();
}
