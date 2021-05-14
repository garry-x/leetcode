pub struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let chrs: Vec<char> = word.chars().collect();

        // construct a marks matrix to record walked path
        let m = board.len();
        let n = board[0].len();
        let mut marks: Vec<Vec<i32>> = vec![vec![0; n]; m];

        for pos in 0..m * n {
            if board[pos / n][pos % n] == chrs[0] {
                if walk(&board, &chrs, &mut marks, (pos / n, pos % n), 0) {
                    return true;
                }
            }
        }

        false
    }
}

pub fn walk(board: &Vec<Vec<char>>, chrs: &Vec<char>,
            marks: &mut Vec<Vec<i32>>,
            (x, y): (usize, usize), i: usize) -> bool {
    let m = board.len();
    let n = board[0].len();

    if marks[x][y] == 1 || board[x][y] != chrs[i] {
        return false;
    }

    marks[x][y] = 1;

    if i == chrs.len() - 1 {
        return true;
    }

    if y + 1 < n && walk(board, chrs, marks, (x, y + 1), i + 1) {
        return true;
    }
    if y >= 1 && walk(board, chrs, marks, (x, y - 1), i + 1) {
        return true;
    }
    if x + 1 < m && walk(board, chrs, marks, (x + 1, y), i + 1) {
        return true;
    }
    if x >=1 && walk(board, chrs, marks, (x - 1, y), i + 1) {
        return true;
    }

    marks[x][y] = 0;

    false
}

use std::fmt::Display;

pub fn print_matrix<T: Display>(matrix: &Vec<Vec<T>>) {
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

    assert_eq!(true, Solution::exist(vec![vec!['A', 'B', 'C', 'E'],
                                          vec!['S', 'F', 'C', 'S'],
                                          vec!['A', 'D', 'E', 'E']], "ABCCED".to_string()));

    assert_eq!(true, Solution::exist(vec![vec!['A', 'B', 'C', 'E'],
                                          vec!['S', 'F', 'C', 'S'],
                                          vec!['A', 'D', 'E', 'E']], "SEE".to_string()));

    assert_eq!(false, Solution::exist(vec![vec!['A', 'B', 'C', 'E'],
                                           vec!['S', 'F', 'C', 'S'],
                                           vec!['A', 'D', 'E', 'E']], "ABCB".to_string()));


    assert_eq!(true, Solution::exist(vec![vec!['A']], "A".to_string()));
    assert_eq!(false, Solution::exist(vec![vec!['A']], "B".to_string()));
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
