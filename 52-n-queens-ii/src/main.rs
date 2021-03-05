pub struct Solution {}

/**
The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.

Given an integer n, return the number of distinct solutions to the n-queens puzzle.

Input: n = 4
Output: 2
Explanation: There are two distinct solutions to the 4-queens puzzle as shown.

Example 2:

Input: n = 1
Output: 1


Constraints:

1 <= n <= 9
*/

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        solve_rec(n as usize, &mut Vec::new())
    }
}

pub fn is_insight((x, y): (i32, i32), (x1, y1): (i32, i32)) -> bool {
    if x == x1 || y == y1 || (x - x1).abs() == (y - y1).abs() {
        true
    } else {
        false
    }
}

pub fn solve_rec(n: usize, cols: &mut Vec<usize>) -> i32{
    if cols.len() == n {
        return 1;
    }
    
    let mut count = 0; 
    let y = cols.len();

    'outer: for x in 0..n {
        'inner: for (y1, &x1) in cols.iter().enumerate() {
            if is_insight((x as i32, y as i32),
                          (x1 as i32, y1 as i32)) {
                continue 'outer;
            }
        }

        cols.push(x);
        count += solve_rec(n, cols);
        cols.pop();
    }

    count
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();
    
    println!("{}", Solution::total_n_queens(4)); 
    println!("{}", Solution::total_n_queens(9)); 

    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
