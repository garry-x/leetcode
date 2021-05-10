pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let chrs_s: Vec<char> = s.chars().collect();

        let mut map: HashMap<char, i32> = HashMap::new();
        for ch in t.chars() {
            let counter = map.entry(ch).or_insert(0);
            *counter += 1;
        }

        let mut min = None;
        let (mut min_s, mut min_e) = (0, 0);

        let mut count = 0;
        let mut l_cur = 0;
        let mut cur = 0;

        while cur < chrs_s.len() {
            let mut found = false;

            if let Some(v) = map.get_mut(&chrs_s[cur]) {
                *v -= 1;
                if *v >= 0 {
                    count += 1;
                }
                found = true;
            }

            while count >= t.len() && found {
                if min.is_none() || cur - l_cur + 1 < min.unwrap() {
                    min = Some(cur - l_cur + 1);
                    min_s = l_cur;
                    min_e = cur;
                }

                if let Some(v) = map.get_mut(&chrs_s[l_cur]) {
                    *v += 1;
                    if *v > 0 {
                        count -= 1;
                    }
                }

                l_cur += 1;
            }
            cur += 1;
        }

        if min.is_none() {
            "".to_string()
        } else {
            chrs_s[min_s..min_e + 1].iter().collect::<String>()
        }
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();
    
    println!("Output:{}", Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()));
    println!("Output:{}", Solution::min_window("a".to_string(), "a".to_string()));
        
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
