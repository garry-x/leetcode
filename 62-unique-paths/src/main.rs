pub struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize - 1, n as usize - 1);
        
        let s = std::cmp::min(m, n);

        if s == 0 {
            return 1;
        }

        // choose 's' empty slot from (m + n) && remove dup
        // C(s) (m+n) / s!
        let mut sum = ((m + n - s + 1)..(m + n + 1)).fold(1, |acc, x| acc * x);
        sum /= (1..(s + 1)).fold(1, |acc, x| acc * x);

        sum as i32
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();
   
    assert_eq!(28, Solution::unique_paths(3, 7));
    assert_eq!(3, Solution::unique_paths(3, 2));
    assert_eq!(28, Solution::unique_paths(7, 3));
    assert_eq!(6, Solution::unique_paths(3, 3));
    assert_eq!(1, Solution::unique_paths(3, 1));

    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
