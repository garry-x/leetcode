pub struct Solution {}

impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        
        for x in 0..m {
            for y in 0..n {
                if x == 0 && y == 0 {
                    continue;
                }

                if x == 0 {
                    grid[x][y] += grid[x][y - 1];
                } else if y == 0 {
                    grid[x][y] += grid[x - 1][y];
                } else {
                    grid[x][y] += std::cmp::min(grid[x - 1][y], grid[x][y - 1]);
                }
            }
        }
        
        grid[m - 1][n - 1]
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();
   
    assert_eq!(7, Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]));
    assert_eq!(12, Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]));
    assert_eq!(1, Solution::min_path_sum(vec![vec![1]]));
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
