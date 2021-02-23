pub struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut out: Vec<Vec<i32>> = Vec::new();    
        
        perm_rec(&nums, &mut vec![false; nums.len()],
                 &mut Vec::new(), &mut out, nums.len());
    
        out
    }
}

pub fn perm_rec(nums: &Vec<i32>, mask: &mut Vec<bool>,
                buf: &mut Vec<i32>, out: &mut Vec<Vec<i32>>, 
                count: usize) {
    if count == 0 {
        out.push(buf.clone());
        return;
    }
    
    for i in 0..nums.len() {
        if mask[i] {
            continue;
        }

        mask[i] = true;
        buf.push(nums[i]);
        perm_rec(nums, mask, buf, out, count - 1);
        buf.pop();
        mask[i] = false;
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();

    println!("{:?}", Solution::permute(vec![1, 2]));
    println!("{:?}", Solution::permute(vec![1, 2, 3]));
    println!("{:?}", Solution::permute(vec![1, 2, 3, 4]));
    println!("{:?}", Solution::permute(vec![1, 2, 3, 4, 6]));
    println!("{:?}", Solution::permute(vec![1]));
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
