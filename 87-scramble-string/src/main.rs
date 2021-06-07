pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        is_scramble_itr(&s1.chars().collect::<Vec<char>>()[..], 0, 
                        &s2.chars().collect::<Vec<char>>()[..], 0,
                        &mut HashMap::new())
    }
}

pub fn is_scramble_itr(chrs1: &[char], i1: usize,
                       chrs2: &[char], i2: usize,
                       map: &mut HashMap<(usize, usize, usize), bool>) -> bool {

    if chrs1.len() != chrs2.len() {
        return false;
    }

    let len = chrs1.len();
    if len == 1 {
        return chrs1[0] == chrs2[0];
    }

    if let Some(&v) = map.get(&(i1, i2, len)) {
        return v;
    }

    for i in 1..len {
        if is_scramble_itr(&chrs1[0..i], i1, &chrs2[0..i], i2, map) && 
           is_scramble_itr(&chrs1[i..len], i1 + i, &chrs2[i..len], i2 + i, map) {
            return true;
        }
        if is_scramble_itr(&chrs1[0..i], i1, &chrs2[(len - i)..len], i2 + len - i, map) &&
           is_scramble_itr(&chrs1[i..len], i1 + i, &chrs2[0..len - i], i2, map) {
            return true;
        }
    }

    map.insert((i1, i2, len), false);

    false
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();

    assert_eq!(true, Solution::is_scramble("great".to_string(), "rgeat".to_string())); 
    assert_eq!(false, Solution::is_scramble("abcde".to_string(), "caebd".to_string())); 
    assert_eq!(false, Solution::is_scramble("a".to_string(), "c".to_string())); 
    assert_eq!(true, Solution::is_scramble("a".to_string(), "a".to_string())); 
    assert_eq!(false, Solution::is_scramble("ababcbaccbabbcbca".to_string(), "bbbbbaaaacccccbba".to_string())); 
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
