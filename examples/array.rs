//Letcoode problems

use std::collections::HashMap;

struct Array {}

impl Array {
    //Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
    // You may assume that each input would have exactly one solution, and you may not use the same element twice.
    // You can return the answer in any order.
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
}

fn main() {
    Array::test_two_sum();
}
