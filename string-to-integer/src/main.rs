pub struct Solution {}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut res = 0i64;
        let mut sflag = 1;

        for (n, c) in s.trim_start().char_indices() {
             if Self::check_ch(c, n) {
                 match c {
                     '+' => (),
                     '-' => sflag = -1,
                     _ => res = res * 10 + c.to_digit(10).unwrap() as i64, 
                 }

                 if (res * sflag) > 2147483647 {
                     res = 2147483647;
                     return res as i32;
                 }

                 if (res * sflag) < -2147483648 {
                     res = -2147483648;
                     return res as i32;
                 }
             } else {
                 break;
             }
        }

        (res * sflag) as i32
    }

    fn check_ch(ch: char, n: usize) -> bool {
        // check sign bit
        if n == 0 && (ch == '+' || ch == '-') {
            return true;
        }

        if ch.is_digit(10) {
            return true;
        }

        false
    }
}

fn main() {
    println!("result={}", Solution::my_atoi(String::from("42")));
    println!("result={}", Solution::my_atoi(String::from("-42")));
    println!("result={}", Solution::my_atoi(String::from("+42")));
    println!("result={}", Solution::my_atoi(String::from("4193 with words")));
    println!("result={}", Solution::my_atoi(String::from("words and 987")));
    println!("result={}", Solution::my_atoi(String::from("-91283472332")));
    println!("result={}", Solution::my_atoi(String::from("   -42")));
    println!("result={}", Solution::my_atoi(String::from("+-12")));
    println!("result={}", Solution::my_atoi(String::from("  -0012a42")));
    println!("result={}", Solution::my_atoi(String::from("20000000000000000000")));
    println!("result={}", Solution::my_atoi(String::from("")));
}
