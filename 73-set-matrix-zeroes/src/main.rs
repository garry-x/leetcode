pub struct Solution {}

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (m, n) = (matrix.len(), matrix[0].len());

        let mut row_zero = false;                              
        let mut col_zero = false;

        for i in 0..std::cmp::max(m, n) {
            if i < m && matrix[i][0] == 0 {
                col_zero = true;
            }
            if i < n && matrix[0][i] == 0 {
                row_zero = true;
            }
        }
        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        for i in 1..m {
            for j in 1..n {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }
        for i in 0..std::cmp::max(m, n) {
            if i < m && col_zero {
                matrix[i][0] = 0;
            }
            if i < n && row_zero {
                matrix[0][i] = 0;
            }
        }
    } 
 }

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();

    Solution::set_zeroes(&mut vec![vec![1, 1, 1],
                                   vec![1, 0, 1],
                                   vec![1, 1, 1]]); 
    Solution::set_zeroes(&mut vec![vec![0, 1, 2, 0],
                                   vec![3, 4, 5, 2],
                                   vec![1, 3, 1, 5]]); 
    Solution::set_zeroes(&mut vec![vec![1]]);
    Solution::set_zeroes(&mut vec![vec![0]]);
    Solution::set_zeroes(&mut vec![vec![1, 0, 3]]);
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
