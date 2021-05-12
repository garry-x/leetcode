pub struct Solution {}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut out: Vec<Vec<i32>> = Vec::new();
        combine_itr(&mut out, &mut Vec::new(), n as usize, 0, k as usize);
        out
    }
}

pub fn combine_itr(out: &mut Vec<Vec<i32>>,
                   buf: &mut Vec<i32>, n: usize, s: usize, k: usize){
    if buf.len() == k {
        out.push(buf.clone());
        return;
    }

    if buf.len() + n - s < k {
        return;
    }

    for i in s..n {
        buf.push(i as i32 + 1);
        combine_itr(out, buf, n, i + 1, k); 
        buf.pop();
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();

    println!("Output: {:?}", Solution::combine(1, 1));
    println!("Output: {:?}", Solution::combine(4, 2));
    println!("Output: {:?}", Solution::combine(4, 3));
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
