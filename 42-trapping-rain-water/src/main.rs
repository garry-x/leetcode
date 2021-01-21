use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        // 1. the topmost pos if all the right pos are lower then 'left'
        // 2. the first pos that higher than 'left' 
        fn first_top(h: &Vec<i32>, left: usize) -> i32 {
            let mut i = left + 1;
            let mut right = -1;
            let mut max_height = -1;

            while i + 1 < h.len() {
                while i + 1 < h.len() && h[i] < h[i + 1] 
                                      && h[i] < h[left] {
                    i += 1;
                }

                if h[i] > max_height {
                    right = i as i32;
                    max_height = h[i];
                }

                if h[i] >= h[left] {
                    break;
                }

                i += 1;    
            }

            right
        }
        
        if height.len() == 0 {
            return 0;
        }
        
        let mut area = 0;
        let mut i = 0;
        loop {
            if i + 1 >= height.len() {
                break;
            }

            if height[i + 1] >= height[i] {
                // climb the mountain
                i += 1;
                continue;
            }

            // downhill ? check if we can get a ravine
            let right = first_top(&height, i);
            if right == -1{
                // no room to hold water
                break;
            }
            
            let mut larea = (right - i as i32 - 1) * cmp::min(height[i],
                                                              height[right as usize]);
            
            i += 1;
            while i < right as usize {
                larea -= height[i];
                i += 1;
            }
            assert!(larea >= 0);

            area += larea;

        }

        area
    }
}

fn main() {
    assert_eq!(6, Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]));
    assert_eq!(9, Solution::trap(vec![4,2,0,3,2,5]));
    assert_eq!(0, Solution::trap(vec![]));
    assert_eq!(0, Solution::trap(vec![1]));
    assert_eq!(0, Solution::trap(vec![0]));
    assert_eq!(1, Solution::trap(vec![5,4,1,2]));
    assert_eq!(9, Solution::trap(vec![2,4,5,6,8,5,5,0,0,0,3,3]));
}
