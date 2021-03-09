pub struct Solution {}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut out: Vec<i32> = Vec::new();
        
        let len_x = matrix.len();
        let len_y = matrix[0].len();

        let mut layer = 0;
        let (mut x, mut y) = (0, 0);

        while out.len() < len_x * len_y {
            out.push(matrix[x][y]);

            if x == layer && y < (len_y - layer - 1) {
                y += 1;
                continue;    
            }

            if y == len_y - layer - 1 && x < (len_x - layer - 1) {
                x += 1;
                continue;
            }

            if x == len_x - layer - 1 && y > layer {
                y -= 1;
                continue;
            }

            if y == layer && x > layer {
                x -= 1;
            }

            if x == layer && y == layer {
                x += 1;
                y += 1;
                layer += 1;
            }
        }

        out 
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();
    
    //1  2  3  4
    //5  6  7  8
    //9 10 11 12
    println!("{:?}", Solution::spiral_order(vec![vec![1, 2, 3, 4],
                                                 vec![5, 6, 7, 8],
                                                 vec![9, 10, 11, 12]]));    

    println!("{:?}", Solution::spiral_order(vec![vec![1]]));
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
