use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut n1 = 0;
        let mut n2 = height.len() - 1;

        while n1 < n2 {
            let h1 = height[n1] as usize;
            let h2 = height[n2] as usize;

            max_area = cmp::max(cmp::min(h2, h1) * (n2 - n1), max_area);

            if h1 < h2 {
                n1 += 1;
            } else {
                n2 -= 1;
            }
        }
        
        max_area as i32
    }
}

fn main() {
    assert!(49 == Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    assert!(1 == Solution::max_area(vec![1, 1]));
    assert!(16 == Solution::max_area(vec![4, 3, 2, 1, 4]));
    assert!(2 == Solution::max_area(vec![1, 2, 1]));
}
