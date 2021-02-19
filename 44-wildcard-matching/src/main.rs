pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut map: HashMap<(usize, usize), bool> = HashMap::new();
        
        let chrs_s: Vec<char> = s.chars().collect();
        let chrs_p: Vec<char> = p.chars().collect();

        is_match_inner(&chrs_s, 0, &chrs_p, 0, &mut map)
    }
}

pub fn is_match_inner(chrs_s: &Vec<char>, i_s: usize,
                      chrs_p: &Vec<char>, i_p: usize,
                      map: &mut HashMap<(usize, usize), bool>) -> bool {
    if let Some(&v) = map.get(&(i_s, i_p)) {
        return v;
    }
    
    if i_s >= chrs_s.len() && i_p >= chrs_p.len() {
        return true;
    }

    if i_p >= chrs_p.len() {
        return false;
    }

    match chrs_p[i_p] {
        '*' => {
            let mut res;

            res = is_match_inner(chrs_s, i_s,
                                 chrs_p, i_p + 1, map);
            map.insert((i_s, i_p + 1), res);
            
            if !res && i_s < chrs_s.len() {
                res = is_match_inner(chrs_s, i_s + 1,
                                     chrs_p, i_p, map);
                map.insert((i_s + 1, i_p), res);
            }

            res
        },
        _ => {
            if i_s >= chrs_s.len() || (chrs_p[i_p] != '?' && 
                                       chrs_s[i_s] != chrs_p[i_p]){
                false
            } else {
                is_match_inner(chrs_s, i_s + 1,
                               chrs_p, i_p + 1, map)
            }
        }
    }
}

use std::time:: SystemTime;

fn test_is_match() {
    let start = SystemTime::now();
    println!("Output: {}",  Solution::is_match("aa".to_string(), "a".to_string()));
    println!("Output: {}",  Solution::is_match("aa".to_string(), "*".to_string()));
    println!("Output: {}",  Solution::is_match("cb".to_string(), "*a".to_string()));
    println!("Output: {}",  Solution::is_match("cb".to_string(), "?a".to_string()));
    println!("Output: {}",  Solution::is_match("adceb".to_string(), "*a*b".to_string()));
    println!("Output: {}",  Solution::is_match("acdcb".to_string(), "a*c?b".to_string()));
    println!("Output: {}",  Solution::is_match("".to_string(), "".to_string()));
    println!("Output: {}",  Solution::is_match("babbbbaabababaabbababaababaabbaabababbaaababbababaaaaaabbabaaaabababbabbababbbaaaababbbabbbbbbbbbbaabbb".to_string(),
                                               "b**bb**a**bba*b**a*bbb**aba***babbb*aa****aabb*bbb***a".to_string()));
    println!("Output: {}",  Solution::is_match("aaabbbaabaaaaababaabaaabbabbbbbbbbaabababbabbbaaaaba".to_string(),
                                               "a**********************a".to_string()));
    println!("Output: {}",  Solution::is_match("abbabaaabbabbaababbabbbbbabbbabbbabaaaaababababbbabababaabbababaabbbbbbaaaabababbbaabbbbaabbbbababababbaabbaababaabbbababababbbbaaabbbbbabaaaabbababbbbaababaabbababbbbbababbbabaaaaaaaabbbbbaabaaababaaaabb".to_string(),
                                               "**aa*****ba*a*bb**aa*ab****a*aaaaaa***a*aaaa**bbabb*b*b**aaaaaaaaa*a********ba*bbb***a*ba*bb*bb**a*b*bb".to_string()));
    println!("Time elapsed:{} ms", SystemTime::now().duration_since(start).unwrap().as_millis());
}

fn main() {
    test_is_match()
}
