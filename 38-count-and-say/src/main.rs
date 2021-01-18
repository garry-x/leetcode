pub struct Solution {}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }

        let str_n = Self::count_and_say(n - 1) + &"#".to_string();

        let mut count = 0;
        let mut out = "".to_string();
        let mut prev = '.';

        for ch in str_n.chars() {
           if ch != prev && prev != '.' {
               out.push_str(&count.to_string());
               out.push(prev);
               count = 1;
           } else {
               count += 1;
           }

           prev = ch;
        }

        out 
    }
}

fn main() {
    for i in 1..15 {
        println!("Input={}, Output={}", i, Solution::count_and_say(i));
    }
}
