pub struct Solution {}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        let half = Solution::my_pow(x, n / 2);
        
        if n % 2 == 0 {
            return half * half;
        } 

        if n > 0 {
            return half * half * x;
        }

        return half * half / x;
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();
    
    println!("{}^{} = {}", 3.0, 20, Solution::my_pow(3.0, 20));
    println!("{}^{} = {}", 2.0, 10, Solution::my_pow(2.0, 10));
    println!("{}^{} = {}", 2.1, 3, Solution::my_pow(2.1, 3));
    println!("{}^{} = {}", 2, -2, Solution::my_pow(2.0, -2));
    println!("{}^{} = {}", 0.00001, 2147483647, Solution::my_pow(0.00001, 2147483647));
    println!("{}^{} = {}", 2.00000, -2147483648, Solution::my_pow(2.00000, -2147483648));

    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
