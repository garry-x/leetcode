pub struct Solution {}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut cols: Vec<usize> = Vec::new();
        let mut out: Vec<Vec<String>>= Vec::new();
        solve_rec(n as usize, &mut cols, &mut out);
        out  
    }
}

pub fn is_insight((x, y): (i32, i32), (x1, y1): (i32, i32)) -> bool {
    if x == x1 || y == y1 || (x - x1).abs() == (y - y1).abs() {
        true
    } else {
        false
    }
}

pub fn solve_rec(n: usize, cols: &mut Vec<usize>,
                 out: &mut Vec<Vec<String>>) {
    if cols.len() == n {
        out.push(draw_board(n, cols));
        return;
    }
    
    let y = cols.len();

    'outer: for x in 0..n {
        'inner: for (y1, &x1) in cols.iter().enumerate() {
            if is_insight((x as i32, y as i32),
                          (x1 as i32, y1 as i32)) {
                continue 'outer;
            }
        }

        cols.push(x);
        solve_rec(n, cols, out);
        cols.pop();
    }
}

pub fn draw_board(n: usize, cols: &Vec<usize>) -> Vec<String> {
    let mut board: Vec<Vec<char>> = vec![vec!['.'; n]; n];

    for (y, &x) in cols.iter().enumerate() {
        board[x][y] = 'Q';
    }

    let mut out: Vec<String> = Vec::new();
    for v in board {
        out.push(v.into_iter().collect::<String>());
    }

    out
}

fn print_board(boards: &Vec<Vec<String>>) {
    for b in boards {
        for s in b {
            println!("{}", s);
        }
        println!("")
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();
   
    print_board(&Solution::solve_n_queens(1));
    print_board(&Solution::solve_n_queens(2));
    print_board(&Solution::solve_n_queens(3));
    print_board(&Solution::solve_n_queens(4));
    //print_board(&Solution::solve_n_queens(9));

    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
