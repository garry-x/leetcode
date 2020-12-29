pub struct Solution {}

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == 0 {
            return 0;
        }

        //check sig
        let is_negative = dividend.signum() + divisor.signum() == 0;

        let mut value  = (dividend as i64).abs();
        let mut result = 0i64;

        let mut base = (divisor as i64).abs();
        let mut count = 1;

        let mut cache:Vec<(i64, i64)> = Vec::new();
        cache.push((base, count)); 
        
        loop {
            if value >= base {
                value -= base;
                result += count;

                // double base & count
                base += base;
                count += count;

                // put base and result to cache
                cache.push((base, count));
            } else {
                if cache.is_empty() {
                    break;
                }

                // shrink base
                let (b1, c1) = cache.pop().unwrap();
                base = b1;
                count = c1;
            }
        }
        
        // fix sig
        if is_negative {
            result = 0 - result;
        }
        
        // check overflow
        if result >= 2147483648 || result < -2147483648 {
            return 2147483647;
        }
          
        result as i32
    }
}

fn main() {
    assert_eq!(3, Solution::divide(10, 3));
    assert_eq!(-2, Solution::divide(7, -3));
    assert_eq!(0, Solution::divide(0, 1));
    assert_eq!(1, Solution::divide(1, 1));
    assert_eq!(2147483647, Solution::divide(-2147483648, -1));
}
