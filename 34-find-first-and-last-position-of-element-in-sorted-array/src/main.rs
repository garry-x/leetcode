pub struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut start = -1;
        let mut end = -1;

        if nums.len() != 0  {
            Self::search_inner(&nums, 0, nums.len() - 1, target,
                               &mut start, &mut end);
        }

        return vec![start, end];    
    }

    fn search_inner(nums: &Vec<i32>,
                    i: usize,
                    j: usize,
                    target: i32,
                    start: &mut i32,
                    end: &mut i32) {
        
        if i > j {
            return;
        }

        if nums[i] == nums[j] && nums[i] == target {
            Solution::update_range(start, end, i as i32, j as i32);
            return;
        }

        let half = (j - i + 1) / 2;
        let mid = i + half;

        if nums[mid] == target {
            Solution::update_range(start, end, mid as i32, mid as i32);
        }

        if half < 1 {
            return;
        }

        if target >= nums[i] && target <= nums[mid] {
            Self::search_inner(nums, i, mid - 1, target, start, end);
        }

        if target >= nums[mid] && target <= nums[j] {
            Self::search_inner(nums, mid + 1, j, target, start, end);
        }
    }

    fn update_range(start: &mut i32, end: &mut i32, s: i32, e: i32) {
        if *start == -1 || s < *start {
            *start = s;
        }

        if *end == -1 || e > *end {
            *end = e;
        }
    }
}

fn main() {
    println!("Output: {:?}", Solution::search_range(vec![5,7,7,8,8,10], 8));
    println!("Output: {:?}", Solution::search_range(vec![5,7,7,8,8,10], 6));
    println!("Output: {:?}", Solution::search_range(vec![1], 0));
    println!("Output: {:?}", Solution::search_range(vec![], 0));
    println!("Output: {:?}", Solution::search_range(vec![8,8,8,8,8,8,8,8], 8));
}
