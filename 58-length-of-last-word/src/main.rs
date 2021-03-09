pub struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        match s.split_ascii_whitespace().last() {
            Some(v) => v.len() as i32,
            None => 0
        }
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();
   
    assert_eq!(5, Solution::length_of_last_word("Hello World".to_string()));
    assert_eq!(1, Solution::length_of_last_word(" a".to_string()));
    assert_eq!(0, Solution::length_of_last_word("  ".to_string()));

    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
