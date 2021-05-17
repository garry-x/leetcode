pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let mut i = 0;
        let mut j = nums.len() - 1;

        while i <= j {
            let mid = i + (j - i) / 2;

            if nums[mid] == target {
                return true;
            }

            if i == j {
                break;
            }

            if nums[i] > nums[mid] {
                if target > nums[mid] && target <= nums[j] { 
                    i = mid;
                } else {
                    j = mid;
                }
            } else if nums[i] < nums[mid] {
                if target >= nums[i] && target < nums[mid] {
                    j = mid;
                } else {
                    i = mid;
                }
            } else {
                if nums[i] == target {
                    return true;
                } else {
                    i += 1;
                }
            }
        }

        false
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();

    assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
    assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
    assert_eq!(Solution::search(vec![2], 3), false);
    assert_eq!(Solution::search(vec![2, 2, 2, 2, 2, 2], 3), false);
    assert_eq!(Solution::search(vec![2, 2, 2, 2, 2, 2], 2), true);
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
