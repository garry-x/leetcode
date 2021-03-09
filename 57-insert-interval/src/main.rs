pub struct Solution {}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut out: Vec<Vec<i32>> = Vec::new();
       
        let (mut s, mut e, mut inserted) = (new_interval[0], new_interval[1], false);

        for i in intervals {
            if i[1] < s {
                out.push(i);
                continue
            }

            if i[0] > e  {
                if !inserted {
                    inserted = true;
                    out.push(vec![s, e]);
                }
                out.push(i);
                continue;
            }

            s = std::cmp::min(s, i[0]);
            e = std::cmp::max(e, i[1]);
        }

        if !inserted {
            out.push(vec![s, e]);
        }

        out        
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();
   
    println!("Output: {:?}", Solution::insert(vec![vec![1,3],vec![6,9]], vec![2,5])); 
    println!("Output: {:?}", Solution::insert(vec![vec![1,2],vec![3,5],vec![6,7],vec![8,10],vec![12,16]], vec![4,8])); 
    println!("Output: {:?}", Solution::insert(vec![], vec![2,5])); 
    println!("Output: {:?}", Solution::insert(vec![vec![1,5]], vec![2,3])); 
    println!("Output: {:?}", Solution::insert(vec![vec![1,5]], vec![2,7])); 
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
