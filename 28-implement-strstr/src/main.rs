pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }

        return Self::str_inner(&haystack.chars().collect(),
                               &needle.chars().collect());
    }

    fn str_inner(orig: &Vec<char>, needle: &Vec<char>) -> i32 {
        for (n, c) in orig.iter().enumerate() {
            let ret = Self::str_rec(&orig[n..], &needle);
            if ret != -1 {
                return n as i32 + ret;
            }
        }

        -1
    }

    fn str_rec(orig: &[char], needle: &Vec<char>) -> i32 {
        if needle.len() > orig.len() {
            return -1;
        }

        for (n, c) in needle.iter().enumerate() {
            if needle[n] != orig[n] {
                return -1;
            }
        }

        0
    }
}

fn main() {
    assert_eq!(2, Solution::str_str("hello".to_string(), "ll".to_string()));
    assert_eq!(-1, Solution::str_str("aaaaa".to_string(), "bba".to_string()));
    assert_eq!(0, Solution::str_str("".to_string(), "".to_string()));
}
