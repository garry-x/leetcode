use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (_i, value) in nums.iter().enumerate() {
            match map.get(value) {
                Some(&index) => return vec![index, _i as i32],
                None => map.insert(target - value, _i as i32),
            };
        }
        
        vec![]
    }
}

fn main() {
    println!("result {:?}", Solution::two_sum(vec![2, 7, 11], 9));
    println!("result {:?}", Solution::two_sum(vec![3, 2, 4], 6));
    println!("result {:?}", Solution::two_sum(vec![3, 3], 6));
}
