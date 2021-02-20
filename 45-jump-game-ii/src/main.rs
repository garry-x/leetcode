pub struct Solution {}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut jumps = 0;
        let mut cur = 0;
        let mut prev = 0; 

        while cur < nums.len() - 1 {
            jumps += 1;
            
            let mut j = prev;
            prev = cur;

            while j <= prev {
                cur = std::cmp::max(cur, nums[j] as usize + j);
                j += 1;
            }
        }
        
        jumps
    }
}

use std::time:: SystemTime;

fn main() {
    let mut vec1: Vec<i32> = Vec::new();
    for i in (0..25000).rev() {
        vec1.push(i);
    }

    let start = SystemTime::now();
    assert_eq!(2, Solution::jump(vec![2, 3, 1, 1, 4]));
    assert_eq!(2, Solution::jump(vec![1, 3, 1, 1, 4]));
    assert_eq!(0, Solution::jump(vec![0]));
    assert_eq!(2, Solution::jump(vec![1, 3, 1, 0, 4]));
    assert_eq!(3, Solution::jump(vec![3, 2, 3, 0, 1, 5, 6]));
    assert_eq!(1, Solution::jump(vec1));
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
