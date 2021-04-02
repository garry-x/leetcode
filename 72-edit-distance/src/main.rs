pub struct Solution {}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let v1: Vec<char> = word1.chars().collect();
        let v2: Vec<char> = word2.chars().collect();

        let mut dp: Vec<Vec<usize>> = vec![vec![0; v2.len() + 1]; v1.len() + 1];
        for i in (0..v1.len() + 1).rev() {
            for j in (0..v2.len() + 1).rev() {
                if j >= v2.len() { dp[i][j] = v1.len() - i; continue; }
                if i >= v1.len() { dp[i][j] = v2.len() - j; continue; }
                if v1[i] == v2[j] { dp[i][j] = dp[i + 1][j + 1]; continue; }
                dp[i][j] = std::cmp::min(std::cmp::min(dp[i + 1][j], dp[i + 1][j + 1]),
                                         dp[i][j + 1]) + 1;
            }
        }

        dp[0][0] as i32
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();

    assert_eq!(3, Solution::min_distance("horse".to_string(), "ros".to_string()));
    assert_eq!(5, Solution::min_distance("intention".to_string(), "execution".to_string()));
    assert_eq!(9, Solution::min_distance("intention".to_string(), "".to_string()));
    assert_eq!(9, Solution::min_distance("".to_string(), "execution".to_string()));
    assert_eq!(4, Solution::min_distance("b".to_string(), "abbbc".to_string()));
    assert_eq!(10, Solution::min_distance("trinitrophenylmethylnitramine".to_string(), "dinitrophenylhydrazine".to_string()));
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
