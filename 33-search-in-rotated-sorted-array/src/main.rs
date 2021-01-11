pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut base = 0;
        let mut len = nums.len();
        let mut half;

        loop {
            half = len / 2;
            let mid = base + half;

            if nums[mid] == target {
                return mid as i32;
            }

            if half < 1 {
                break;
            }

            if !((nums[base] < nums[mid] && (target >= nums[base] && target <= nums[mid])) || 
                (nums[base] > nums[mid] && (target >= nums[base] || target <= nums[mid]))) {
                // goto another half
                base = mid;
            }

            len -= half;
        }

        -1
    }
}

fn main() {
    assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
    assert_eq!(-1, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
    assert_eq!(-1, Solution::search(vec![1], 0));
    assert_eq!(2, Solution::search(vec![1, 3, 5], 5));
    assert_eq!(0, Solution::search(vec![1, 3, 4, 5], 1));
    assert_eq!(3, Solution::search(vec![1, 3, 4, 5], 5));
    assert_eq!(2, Solution::search(vec![3, 4, 5, 1], 5));
}
