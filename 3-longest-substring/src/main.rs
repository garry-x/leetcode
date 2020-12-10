use std::collections::HashMap;
use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len = 0;
        let mut map = HashMap::new();
        let mut start = 0;

        // slice window 
        for (i, c1) in s.char_indices() {
            if let Some(&index) = map.get(&c1) {
                if index >= start {
                    max_len = cmp::max(i - start, max_len);
                    start = index + 1; 
                }
            } 

            map.insert(c1, i);
        }

        max_len = cmp::max(s.chars().count() - start, max_len);

        max_len as i32
    }
}

fn main() {
    assert!(0 == Solution::length_of_longest_substring(String::from("")));
    assert!(3 == Solution::length_of_longest_substring(String::from("abcabcbb")));
    assert!(1 == Solution::length_of_longest_substring(String::from(" ")));
    assert!(4 == Solution::length_of_longest_substring(String::from("abcbad")));
    assert!(6 == Solution::length_of_longest_substring(String::from("abcdabcdef")));
    assert!(2 == Solution::length_of_longest_substring(String::from("cdd")));
    assert!(1 == Solution::length_of_longest_substring(String::from("bbbb")));
    assert!(2 == Solution::length_of_longest_substring(String::from("abba")));
    assert!(6 == Solution::length_of_longest_substring(String::from("abcdef")));
}
