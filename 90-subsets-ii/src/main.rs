pub struct Solution {}

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable(); 

        let mut out: Vec<Vec<i32>> = vec![vec![]]; 
        let mut last = 0;
    
        for i in 0..nums.len() {
            let mut start = 0;
            if last != 0 && nums[i] == *out.last().unwrap().last().unwrap() {
                start = last;
            }
            last = out.len();

            for j in start..out.len() {
                out.push(out[j].clone());
                out.last_mut().unwrap().push(nums[i]);
            }
        }

        out
    }
}

use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();
    
    println!("{:?}", Solution::subsets_with_dup(vec![1, 2, 3]));
    println!("{:?}", Solution::subsets_with_dup(vec![1, 2, 3, 2]));
    println!("{:?}", Solution::subsets_with_dup(vec![1, 2, 2, 2]));
    println!("{:?}", Solution::subsets_with_dup(vec![0]));
    println!("{:?}", Solution::subsets_with_dup(vec![-1, 1, 2]));
    println!("{:?}", Solution::subsets_with_dup(vec![1, 1]));
    println!("{:?}", Solution::subsets_with_dup(vec![1, 1, 1]));
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
