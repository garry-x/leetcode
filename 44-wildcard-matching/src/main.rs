pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let chrs_s: Vec<char> = s.chars().collect();
        let chrs_p: Vec<char> = p.chars().collect();
        
        let (mut i_s, mut i_p) = (0, 0);

        // position for '*' pattern
        let (mut s_s, mut s_p) : (i32, i32) = (-1, -1);

        while i_s < chrs_s.len() {
            if i_p < chrs_p.len() && (chrs_p[i_p] == '?' || 
                                      chrs_s[i_s] == chrs_p[i_p]) {
                i_s += 1;
                i_p += 1;
            } else if i_p < chrs_p.len() && chrs_p[i_p] == '*' {
                s_s = i_s as i32;
                s_p = i_p as i32;
                
                i_p += 1;
            } else if s_s >= 0 && s_p >= 0 {
                // '*' eat a charater
                s_s += 1;

                // try again
                i_s = s_s as usize;
                i_p = (s_p + 1) as usize;
            } else {
                return false;
            }
        }

        while i_p < chrs_p.len() && chrs_p[i_p] == '*' {
            i_p += 1;
        }

        i_p >= chrs_p.len()
    }
}

use std::time:: SystemTime;

fn test_is_match() {
    let start = SystemTime::now();
    assert_eq!(false, Solution::is_match("aa".to_string(), "a".to_string()));
    assert_eq!(true, Solution::is_match("aa".to_string(), "*".to_string()));
    assert_eq!(false, Solution::is_match("cb".to_string(), "*a".to_string()));
    assert_eq!(false, Solution::is_match("cb".to_string(), "?a".to_string()));
    assert_eq!(true, Solution::is_match("adceb".to_string(), "*a*b".to_string()));
    assert_eq!(false, Solution::is_match("acdcb".to_string(), "a*c?b".to_string()));
    assert_eq!(true, Solution::is_match("".to_string(), "".to_string()));
    assert_eq!(false, Solution::is_match("babbbbaabababaabbababaababaabbaabababbaaababbababaaaaaabbabaaaabababbabbababbbaaaababbbabbbbbbbbbbaabbb".to_string(),
                                         "b**bb**a**bba*b**a*bbb**aba***babbb*aa****aabb*bbb***a".to_string()));
    assert_eq!(true, Solution::is_match("aaabbbaabaaaaababaabaaabbabbbbbbbbaabababbabbbaaaaba".to_string(),
                                        "a**********************a".to_string()));
    assert_eq!(false, Solution::is_match("abbabaaabbabbaababbabbbbbabbbabbbabaaaaababababbbabababaabbababaabbbbbbaaaabababbbaabbbbaabbbbababababbaabbaababaabbbababababbbbaaabbbbbabaaaabbababbbbaababaabbababbbbbababbbabaaaaaaaabbbbbaabaaababaaaabb".to_string(),
                                         "**aa*****ba*a*bb**aa*ab****a*aaaaaa***a*aaaa**bbabb*b*b**aaaaaaaaa*a********ba*bbb***a*ba*bb*bb**a*b*bb".to_string()));
    println!("Time elapsed:{} ms", SystemTime::now().duration_since(start).unwrap().as_millis());
}

fn main() {
    test_is_match()
}
