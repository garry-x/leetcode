pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let n = strs.len();

        if n == 0 {
            return "".to_string();
        }
        
        if n == 1 {
            return strs[0].clone();
        }

        Self::lcp(Self::longest_common_prefix(strs[..n/2].to_vec()),
                  Self::longest_common_prefix(strs[n/2..].to_vec()))
    }

    fn lcp(s1: String, s2: String) -> String {
        let v1: Vec<char> = s1.chars().collect();
        let v2: Vec<char> = s2.chars().collect();
        let mut r: Vec<char> = Vec::new();
        let mut i = 0;

        while i < std::cmp::min(v1.len(), v2.len()) {
            if v1[i] == v2[i] {
                r.push(v1[i]);
            } else {
                break;
            }
            i += 1;
        }

        r.iter().collect()
    }
}

fn main() {
    assert_eq!("fl".to_string(), Solution::longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]));
    assert_eq!("".to_string(), Solution::longest_common_prefix(vec!["flower".to_string(), "dog".to_string()]));
    assert_eq!("".to_string(), Solution::longest_common_prefix(vec!["".to_string()]));
    assert_eq!("".to_string(), Solution::longest_common_prefix(vec!["".to_string(), "dog".to_string()]));
}
