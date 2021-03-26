pub struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        (x as f64).sqrt() as i32
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();

    assert_eq!(0, Solution::my_sqrt(0));
    assert_eq!(1, Solution::my_sqrt(1));
    assert_eq!(1, Solution::my_sqrt(2));
    assert_eq!(2, Solution::my_sqrt(4));
    assert_eq!(2, Solution::my_sqrt(7));
    assert_eq!(46339, Solution::my_sqrt(2147395599));

    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
