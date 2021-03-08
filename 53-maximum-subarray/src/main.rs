pub struct Solution {}

/**
Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.



Example 1:

Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
Output: 6
Explanation: [4,-1,2,1] has the largest sum = 6.
Example 2:

Constraints:

1 <= nums.length <= 3 * 104
-105 <= nums[i] <= 105

*/

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_total = 0;
        let mut max_cur = 0;

        for i in (0..nums.len()).rev() {
            if i == nums.len() - 1 {
                max_total = nums[i];
                max_cur = nums[i];
                continue;
            }

            max_cur = std::cmp::max(max_cur + nums[i], nums[i]);
            max_total = std::cmp::max(max_cur, max_total);
        }

        max_total
    }
}

pub fn max_from(nums: &Vec<i32>, i: usize) -> (i32, i32) {
    if i >= nums.len() - 1 {
        return (nums[i], nums[i]);
    }

    let (max_prev, max_total) = max_from(nums, i + 1);
    let max_cur = std::cmp::max(max_prev + nums[i], nums[i]);
    (max_cur, std::cmp::max(max_cur, max_total))
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();
    assert_eq!(6, Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]));
    assert_eq!(-2, Solution::max_sub_array(vec![-2]));
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
