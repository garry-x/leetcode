pub struct Solution {}

impl Solution {
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
