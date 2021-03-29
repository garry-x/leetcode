pub struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        // Newton Iteration
        // f(x) = x^2 - n
        // f'(x) = 2x
        // x2 = 1/2(n/x1 - x1) + x1
        let n = x as f64; 
        let mut x = x as f64;

        while (x * x  - n).abs() >= 1f64 {
            x = (((n / x) - x) / 2f64) + x;
        }

        x as i32
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
