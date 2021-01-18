pub struct Solution {}

// LEARN FROM: https://leetcode.com/problems/sudoku-solver/discuss/300781/not-so-fast-but-easy-to-understand-rust-solution

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        my_filler(board, 0);
    }
}

fn my_filler(board: &mut Vec<Vec<char>>, i: usize) -> bool {
    match (i / 9, i % 9) {
        (9, _) => true,
        (r, c) if board[r][c] != '.' => my_filler(board, i + 1),
        (r, c) => "123456789.".chars().any(|v| {
            board[r][c] = v;
            v != '.' && is_valid(board, r, c) && my_filler(board, i + 1)
        }),
    }
}

fn is_valid(board: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
    
    let value = board[r][c];
    let line = board[r].iter().filter(|v| **v == value);
    let column = board.iter().map(|v| v[c]).filter(|v| *v == value);

    let (r, c) = ((r / 3) * 3, (c / 3) * 3);
    let cube = board[r..r+3].iter().flat_map(|v| v[c..c+3].iter()).filter(|v| **v == value);

    line.count() == 1 && column.count() == 1 && cube.count() ==1
}

fn print_sudoku(board: &Vec<Vec<char>>) {
    board.iter().for_each(|v| println!("{:?}", v));
}


fn main() {
    Solution::solve_sudoku(&mut vec![vec!['5','3','.','.','7','.','.','.','.']
                                    ,vec!['6','.','.','1','9','5','.','.','.']
                                    ,vec!['.','9','8','.','.','.','.','6','.']
                                    ,vec!['8','.','.','.','6','.','.','.','3']
                                    ,vec!['4','.','.','8','.','3','.','.','1']
                                    ,vec!['7','.','.','.','2','.','.','.','6']
                                    ,vec!['.','6','.','.','.','.','2','8','.']
                                    ,vec!['.','.','.','4','1','9','.','.','5']
                                    ,vec!['.','.','.','.','8','.','.','7','9']]);

    Solution::solve_sudoku(&mut vec![vec!['.','.','9','7','4','8','.','.','.'],
                                     vec!['7','.','.','.','.','.','.','.','.'],
                                     vec!['.','2','.','1','.','9','.','.','.'],
                                     vec!['.','.','7','.','.','.','2','4','.'],
                                     vec!['.','6','4','.','1','.','5','9','.'],
                                     vec!['.','9','8','.','.','.','3','.','.'],
                                     vec!['.','.','.','8','.','3','.','2','.'],
                                     vec!['.','.','.','.','.','.','.','.','6'],
                                     vec!['.','.','.','2','7','5','9','.','.']]);
}
