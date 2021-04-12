pub struct Solution {}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut red = 0;
        let mut blue = nums.len() - 1;
        
        let mut i = 0;
        while i <= blue &&  red < blue {
            match nums[i] {
                0 => {
                    nums[i] = nums[red];
                    nums[red] = 0;
                    red += 1;
                },
                2 => {
                    nums[i] = nums[blue];
                    nums[blue] = 2;
                    blue -= 1;
                    continue;
                },
                _ => (),
            }

            i += 1;
        }
    }                 
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();

    Solution::sort_colors(&mut vec![2, 0, 2, 1, 1, 2]);
    Solution::sort_colors(&mut vec![0, 0, 2, 1, 1, 0]);
    Solution::sort_colors(&mut vec![2]);
    Solution::sort_colors(&mut vec![2, 0, 1]);
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
