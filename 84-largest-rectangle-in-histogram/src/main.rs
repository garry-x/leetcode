pub struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        // shrink stack in the end
        heights.push(0);

        let mut v: Vec<usize> = Vec::new();
        let mut max = 0;

        for i in 0..heights.len() {
            // ascending stack
            if v.len() == 0 || heights[i] >= heights[*v.last().unwrap()] {
                v.push(i);
                continue;
            }

            let mut min_h = heights[*v.last().unwrap()];
            
            while v.len() > 0 && heights[*v.last().unwrap()] > heights[i] {
                let top = v.pop().unwrap();
                if heights[top]  < min_h {
                    min_h = heights[top];
                }

                let mut left = 0;
                if v.len() > 0 { 
                    left = v.last().unwrap() + 1;
                }

                max = std::cmp::max(max, (i - left) as i32 * min_h);
            }
        
            v.push(i);
        }

        max
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();

    assert_eq!(10, Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]));
    assert_eq!(2, Solution::largest_rectangle_area(vec![2]));
    assert_eq!(3, Solution::largest_rectangle_area(vec![2, 1, 2]));
    assert_eq!(20000, Solution::largest_rectangle_area(vec![2; 10000]));
    assert_eq!(2, Solution::largest_rectangle_area(vec![0, 2, 0]));
    assert_eq!(6, Solution::largest_rectangle_area(vec![4, 2, 0, 3, 2, 5]));
    
    let n = 10000;
    assert_eq!((n + 1) * (n + 1) / 4, Solution::largest_rectangle_area((1..n + 1).collect::<Vec<i32>>()));
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
