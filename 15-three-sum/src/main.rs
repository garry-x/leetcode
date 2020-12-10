pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sorted = nums.to_vec();
        sorted.sort_unstable();
        
        let mut prev: Option<i32> = None;
        let mut res: Vec<Vec<i32>> = Vec::new();
       
        for (i, n) in sorted.iter().enumerate() {
            if let Some(v) = prev {
                if v == *n {
                    continue;
                }
            }

            if i + 1 < sorted.len() {
                Self::two_sum(&sorted[i+1..], -n, &mut res);
            }

            prev = Some(*n);
        }
        
        res       
    }

    fn two_sum(nums: &[i32], sum: i32, res: &mut Vec<Vec<i32>>) {
        let mut start = 0i32;
        let mut end = nums.len() as i32 - 1;

        while end > start {
            if nums[start as usize] + nums[end as usize] == sum {
                res.push(vec![-sum, nums[start as usize], nums[end as usize]]);
                start = Self::first_not_equal(nums, start, 1, nums.len() as i32);
            } else if nums[start as usize] + nums[end as usize] > sum {
                end = Self::first_not_equal(nums, end, -1, -1); 
            } else {
                start = Self::first_not_equal(nums, start, 1, nums.len() as i32);
            }
        }
    }

    fn first_not_equal(nums: &[i32], i: i32, step: i32, end: i32) -> i32 {
        let mut cur = i + step;
        while cur != end && cur >= 0 {
            if nums[cur as usize] != nums[i as usize] {
                break;
            }
            cur += step;
        }
        cur
    }
}

fn main() {
    println!("{:?}", Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]));
    println!("{:?}", Solution::three_sum(vec![]));
    println!("{:?}", Solution::three_sum(vec![0]));
}
