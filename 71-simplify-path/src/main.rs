pub struct Solution {}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut out: Vec<String> = Vec::new();

        for s in path.split('/') {
            match s {
                "." => (),
                ".." => if out.len() > 0 { out.pop(); },
                _ => {
                    if s.len() > 0 {
                        out.push("/".to_string() + &s.to_string());
                    }
                }
            }
        }

        if out.len() == 0 {
            out.push("/".to_string());
        }

        out.iter().fold("".to_string(), |sum, s| sum + s)
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();

    println!("Output: {}", Solution::simplify_path("/home/".to_string())); 
    println!("Output: {}", Solution::simplify_path("/../".to_string())); 
    println!("Output: {}", Solution::simplify_path("/home//fool/".to_string())); 
    println!("Output: {}", Solution::simplify_path("/a/./b/../../c/".to_string())); 
    println!("Output: {}", Solution::simplify_path("//".to_string())); 
    println!("Output: {}", Solution::simplify_path("/../a".to_string())); 
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
