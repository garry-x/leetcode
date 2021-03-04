pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, usize> = HashMap::new(); 
        let mut out: Vec<Vec<String>> = Vec::new();

        for s in strs {
            let mut v: Vec<char> = s.chars().collect();
            v.sort_unstable();
            let s1: String = v.into_iter().collect();

            if let Some(&j) = map.get(&s1) {
               out[j - 1].push(s); 
            } else {
               out.push(vec![s]);
               map.insert(s1, out.len());
            }
        }

        out
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();

    println!("{:?}", Solution::group_anagrams(vec!["eat".to_string(),"tea".to_string(),"tan".to_string(),"ate".to_string(),"nat".to_string(),"bat".to_string()]));
    println!("{:?}", Solution::group_anagrams(vec!["a".to_string()]));
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
