pub struct Solution {}

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let (n, mut k) = (n as usize, k as usize - 1);
        
        let mut cand: Vec<usize> = (1..(n + 1)).collect();
        let mut total = cand.iter().fold(1, |acc, x| acc * x);
        let mut out: Vec<char> = Vec::new();
        let mut layer = 0;

        while out.len() < n {
            total /= n - layer;
            let i = k / total;
            k = k % total;
            
            out.push(std::char::from_digit(cand[i] as u32, 10).unwrap());
            cand.remove(i);
            layer += 1;
        }

        out.iter().collect()
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();
  
    println!("{:?}", Solution::get_permutation(1, 1));

    for i in 1..50 {
        println!("{:?}", Solution::get_permutation(5, i));
    }

    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
