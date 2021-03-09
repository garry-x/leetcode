pub struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut s = 0;
        let mut e = nums[s] as usize;

        while e < nums.len() - 1 && e > s {
            let mut max = e;
            for i in (s + 1)..(e + 1) {
               if i + nums[i] as usize > max {
                   max = i + nums[i] as usize;
               }
            }

            s = e;
            e = max;
        }

        e >= nums.len() - 1
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();
    assert_eq!(true, Solution::can_jump(vec![0])); 
    assert_eq!(true, Solution::can_jump(vec![2, 3, 1, 1, 4])); 
    assert_eq!(false, Solution::can_jump(vec![3, 2, 1, 0, 4])); 
    assert_eq!(true, Solution::can_jump(vec![1, 1, 1, 1, 1])); 
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
