pub struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut i, mut j, mut k) = (m - 1, n - 1, m + n - 1);

        while j >=0 {
            if i < 0 || nums1[i as usize] < nums2[j as usize] {
                nums1[k as usize] = nums2[j as usize];
                j -= 1;
            } else {
                nums1[k as usize] = nums1[i as usize];
                i -= 1;
            }
            k -= 1;
        }
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();

    Solution::merge(&mut vec![1, 2, 3, 0, 0, 0], 3, &mut vec![2, 5, 6], 3); 
    Solution::merge(&mut vec![1], 1, &mut vec![], 0); 
    Solution::merge(&mut vec![0], 0, &mut vec![1], 1); 

    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
