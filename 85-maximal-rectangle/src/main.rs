pub struct Solution {}

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return 0;
        }
        
        let (m, n) = (matrix.len(), matrix[0].len());
        
        let mut heights: Vec<Vec<usize>> = vec![vec![0; n + 1]; m];
        for j in 0..n {
            for i in (0..m).rev() {
                if matrix[i][j] == '1' {
                    heights[i][j] = 1;
                    if i + 1 < m {
                        heights[i][j] += heights[i + 1][j];
                    }
                }
            }
        }

        let mut max = 0;
        for i in 0..m {
            max = std::cmp::max(max_rect(&heights[i]), max);
        }
        max as i32
    }
}

pub fn max_rect(heights: &Vec<usize>) -> usize {
    // use stack to save a ascending sequence
    let mut stack: Vec<usize> = Vec::new();
    let mut max = 0; 
    
    for i in 0..heights.len() {
        while !stack.is_empty() && heights[i] <= heights[*stack.last().unwrap()]{
            let top = stack.pop().unwrap();
            if stack.is_empty() {
                max = std::cmp::max(heights[top] * i, max);
            } else {
                let prev = *stack.last().unwrap();
                max = std::cmp::max(heights[top] * (i - prev - 1), max);
            }
        }

        stack.push(i);
    }
    
    max
}

pub fn print_matrix<T: std::fmt::Display>(matrix: &Vec<Vec<T>>) {
    let m = matrix.len();
    let n = matrix[0].len();

    for i in 0..(m * n) {
        if (i % n) == 0 {
            print!("\n");
        }
        print!("{:>3}", matrix[i / n][i % n]);
    }
    print!("\n");
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();

    assert_eq!(6, Solution::maximal_rectangle(vec![vec!['1', '0', '1', '0', '0'],
                                                   vec!['1', '0', '1', '1', '1'],
                                                   vec!['1', '1', '1', '1', '1'],
                                                   vec!['1', '0', '0', '1', '0']]));

    assert_eq!(8, Solution::maximal_rectangle(vec![vec!['1', '0', '1', '1', '0', '1'],
                                                   vec!['1', '1', '1', '1', '1', '1'],
                                                   vec!['0', '1', '1', '0', '1', '1'],
                                                   vec!['1', '1', '1', '0', '1', '0'],
                                                   vec!['0', '1', '1', '1', '1', '1'],
                                                   vec!['1', '1', '0', '1', '1', '1']]));

    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
