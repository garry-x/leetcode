pub struct Solution {}

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut out: Vec<i32> = Vec::new();
        for i in 0..2i32.pow(n as u32) {
            // https://en.wikipedia.org/wiki/Gray_code
            out.push((i >> 1) ^ i);
        }
        out
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();
    
    for i in 1..5 {
        let v = Solution::gray_code(i);
        for x in &v {
            println!("{} {:#06b}", x, x);
        }
        println!("");
    }
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
