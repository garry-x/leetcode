pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        num_decode_itr(&s.chars().collect::<Vec<char>>(), &mut HashMap::new(), 0)
    }
}

pub fn num_decode_itr(chrs: &[char], map: &mut HashMap<usize, i32>,
                      base: usize) -> i32{
    if chrs.len() == 0 {
        return 1;
    }

    if chrs[0] == '0' {
        return 0;
    }

    if let Some(&v) = map.get(&base) {
        return v;
    }

    // 'eat' 1 number
    let mut res = num_decode_itr(&chrs[1..], map, base + 1);     

    if chrs.len() >= 2 {
        // 'eat' 2 number
        let v = chrs[0].to_digit(10).unwrap() * 10 + 
                chrs[1].to_digit(10).unwrap();
        if v <= 26 {
            res += num_decode_itr(&chrs[2..], map, base + 2);
        }
    }

    map.insert(base, res);

    res
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
