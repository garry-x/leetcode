pub struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut out: Vec<Vec<i32>> = Vec::new();
        let mut buf: Vec<i32> = Vec::new();
        for i in 0..(nums.len() + 1) {
            combine_itr(&nums, &mut out, &mut buf, 0, i);
        }
        out 
    }
}

pub fn combine_itr(nums: &Vec<i32>, out: &mut Vec<Vec<i32>>,
                   buf: &mut Vec<i32>, s: usize, k: usize){
    if k == 0 {
        out.push(Vec::new());
        return;
    }

    if buf.len() == k {
        out.push(buf.clone());
        return;
    }

    if buf.len() + nums.len() - s < k {
        return;
    }

    for i in s..nums.len() {
        buf.push(nums[i]);
        combine_itr(nums, out, buf, i + 1, k); 
        buf.pop();
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();

    println!("Output: {:?}", Solution::subsets(vec![1, 2, 3])); 
    println!("Output: {:?}", Solution::subsets(vec![0])); 
    println!("Output: {:?}", Solution::subsets(vec![1, 2, 3, 4])); 
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
