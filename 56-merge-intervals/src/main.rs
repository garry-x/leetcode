pub struct Solution {}

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by(|v1, v2| v1[0].cmp(&v2[0]));
        
        let mut out: Vec<Vec<i32>> = Vec::new();
        let (mut s, mut e) = (intervals[0][0], intervals[0][1]);

        for i in &intervals {
            if i[0] > e {
                out.push(vec![s, e]);
                s = i[0];
                e = i[1];
                continue
            }

            if i[1] >= e {
                e = i[1];
            }
        }
        out.push(vec![s, e]);

        out
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();

    println!("Output:{:?}", Solution::merge(vec![vec![8, 10], vec![15, 18], vec![1, 3], vec![2, 6]]));
    println!("Output:{:?}", Solution::merge(vec![vec![8, 10], vec![15, 18], vec![1, 1], vec![1, 1]]));
    println!("Output:{:?}", Solution::merge(vec![vec![8, 10], vec![15, 18], vec![1, 2], vec![2, 7]]));
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
