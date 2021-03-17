pub struct Solution {}

/**
Given two binary strings a and b, return their sum as a binary string.

Example 1:

Input: a = "11", b = "1"
Output: "100"
Example 2:

Input: a = "1010", b = "1011"
Output: "10101"
 

Constraints:

1 <= a.length, b.length <= 104
a and b consist only of '0' or '1' characters.
Each string does not contain leading zeros except for the zero itself.
*/

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let ch_digits: [char; 2] = ['0', '1'];

        let mut v1: Vec<char> = a.chars().collect();
        let mut v2: Vec<char> = b.chars().collect();
        let mut out: Vec<char> = Vec::new();

        v1.reverse();
        v2.reverse();

        let mut i = 0;
        let mut add = 0;
        while i < v1.len() || i < v2.len() {
            let (mut d1, mut d2) = (0, 0);
            if i < v1.len() {
                d1 = v1[i].to_digit(10).unwrap();
            }
            if i < v2.len() {
                d2 = v2[i].to_digit(10).unwrap();
            }

            let sum = d1 + d2 + add;
            add = sum / 2;
            out.push(ch_digits[sum as usize % 2]);

            i += 1;
        }

        if add > 0 {
            out.push(ch_digits[add as usize]);
        }
        out.reverse();

        out.iter().collect::<String>()
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();

    println!("{}", Solution::add_binary("11".to_string(), "1".to_string()));
    println!("{}", Solution::add_binary("1010".to_string(), "1011".to_string()));
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
