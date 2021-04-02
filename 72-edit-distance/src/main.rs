pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        min_rec(&word1.chars().collect(), &word2.chars().collect(),
                0, 0, 0, &mut HashMap::new()) as i32
    }
}

pub fn min_rec(v1: &Vec<char>, v2: &Vec<char>, i: usize, j: usize,
               count: usize, map: &mut HashMap<(usize, usize),
               usize>) -> usize {
    // insert
    if i == v1.len() { return count + v2.len() - j; }
    // delete
    if j == v2.len() { return count + v1.len() - i; } 

    if let Some(&v) = map.get(&(i, j)) {
        return v + count;
    }

    let min_count;
    if v1[i] == v2[j] {
        // go forward
        min_count = min_rec(v1, v2, i + 1, j + 1, count, map);
    } else {
        // replace
        let c1 = min_rec(v1, v2, i + 1, j + 1, count + 1, map);
        // delete
        let c2 = min_rec(v1, v2, i + 1, j, count + 1, map);
        // insert
        let c3 = min_rec(v1, v2, i, j + 1, count + 1, map);
        min_count = std::cmp::min(std::cmp::min(c1, c2), c3);
    }
    
    map.insert((i, j), min_count - count);
    min_count
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
