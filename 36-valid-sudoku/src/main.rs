pub struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for (i, value) in board.iter().flat_map(|v| v.iter()).enumerate() {
            if *value != '.' && !is_valid(&board, i / 9, i % 9) {
                return false;
            }
        }
        true
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

fn main() {
    assert_eq!(true, Solution::is_valid_sudoku(vec![vec!['5','3','.','.','7','.','.','.','.']
                                                   ,vec!['6','.','.','1','9','5','.','.','.']
                                                   ,vec!['.','9','8','.','.','.','.','6','.']
                                                   ,vec!['8','.','.','.','6','.','.','.','3']
                                                   ,vec!['4','.','.','8','.','3','.','.','1']
                                                   ,vec!['7','.','.','.','2','.','.','.','6']
                                                   ,vec!['.','6','.','.','.','.','2','8','.']
                                                   ,vec!['.','.','.','4','1','9','.','.','5']
                                                   ,vec!['.','.','.','.','8','.','.','7','9']]));
}
