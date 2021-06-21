pub struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        if chars[0] == '0' {
            return 0;
        }

        let mut dp: Vec<i32> = vec![0; chars.len()];
        dp[0] = 1;
        
        for i in 1..dp.len() {
            // decode single 
            dp[i] = match chars[i] {
                '0' => 0,
                _ => dp[i - 1],
            };

            if chars[i - 1] == '0' {
                continue;
            }

            // decode double
            let tmp = chars[i - 1].to_digit(10).unwrap() * 10 +
                      chars[i].to_digit(10).unwrap();
            if tmp <= 26 {
                dp[i] += if i > 1 { dp[i - 2] } else { 1 };
            }
        }

        dp[dp.len() - 1]
    }
}

use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();

    assert_eq!(2, Solution::num_decodings("12".to_string()));
    assert_eq!(3, Solution::num_decodings("226".to_string())); 
    assert_eq!(0, Solution::num_decodings("0".to_string())); 
    assert_eq!(0, Solution::num_decodings("06".to_string())); 

    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
