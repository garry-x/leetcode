pub struct Solution {}

/**
Given an integer array nums that may contain duplicates, return all possible subsets (the power set).

The solution set must not contain duplicate subsets. Return the solution in any order.

Example 1:

Input: nums = [1,2,2]
Output: [[],[1],[1,2],[1,2,2],[2],[2,2]]
Example 2:

Input: nums = [0]
Output: [[],[0]]

Constraints:

1 <= nums.length <= 10
-10 <= nums[i] <= 10
*/

use std::collections::HashMap;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut nums_dedup: Vec<i32> = Vec::new();
        for n in &nums {
            let e = map.entry(*n).or_insert(0);
            *e += 1;
            if *e == 1 {
                nums_dedup.push(*n);
            }
        }

        let mut buf: Vec<i32> = Vec::new();
        let mut out: Vec<Vec<i32>> = Vec::new();

        for n in 0..nums.len() + 1{
            subsets_n(&nums_dedup, &mut map, &mut buf, n, &mut out); 
        }
        
        out 
    }
}

pub fn subsets_n(nums: &[i32], map: &mut HashMap<i32, i32>,
                 buf: &mut Vec<i32>, n: usize, 
                 out: &mut Vec<Vec<i32>>) {
    if n == 0 {
        out.push(Vec::new());
        return;
    }

    if buf.len() == n {
        out.push(buf.clone());
        return;
    }

    for i in 0..nums.len() {
        let v = map.get_mut(&nums[i]).unwrap();
        *v -= 1;
        
        buf.push(nums[i]);
        if *v > 0 {
            subsets_n(&nums[i..], map, buf, n, out);
        } else {
            subsets_n(&nums[i+1..], map, buf, n, out);
        }
        buf.pop();

        *map.get_mut(&nums[i]).unwrap() += 1;
    }
}

use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();
    
    println!("{:?}", Solution::subsets_with_dup(vec![1, 2, 3, 2]));
    println!("{:?}", Solution::subsets_with_dup(vec![1, 2, 2]));
    println!("{:?}", Solution::subsets_with_dup(vec![0]));
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
