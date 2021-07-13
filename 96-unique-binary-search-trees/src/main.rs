pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        num_trees_rec(1, n, &mut HashMap::new())
    }
}

pub fn num_trees_rec(start: i32, end: i32,
                     map: &mut HashMap<(i32, i32), i32>) -> i32 {
    if start > end {
        return 1;
    }

    if let Some(&v) = map.get(&(start, end)) {
        return v;
    }

    let mut sum = 0;

    for i in start..end + 1 {
        let left = num_trees_rec(start, i - 1, map);
        let right = num_trees_rec(i + 1, end, map);
        sum += left * right; 
    }

    map.insert((start, end), sum);

    sum
}

use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();
    
    assert_eq!(1, Solution::num_trees(1));
    assert_eq!(2, Solution::num_trees(2));
    assert_eq!(5, Solution::num_trees(3));
    assert_eq!(14, Solution::num_trees(4));

    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
