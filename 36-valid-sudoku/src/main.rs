use std::collections::HashMap;

pub struct Solution {}

struct Digit {
    pos: Vec<(usize, usize)>,
}

impl Digit {
    fn new() -> Digit {
        Digit { pos: Vec::new() }
    }

    fn is_valid(&self, r: usize, c: usize) -> bool {
        for (r1, c1) in self.pos.iter() {
            if *r1 == r || *c1 == c {
                return false;
            }
            if r1/3 == r/3 && c1/3 == c/3 {
                return false;
            }
        }

        true
    }

    fn add(&mut self, r: usize, c: usize) {
        self.pos.push((r, c));
    }
}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut map = HashMap::new();
        let lines = board.len();

        let mut r = 0;
        let mut c = 0;

        loop {
            if r >= lines {
                break;
            }
            
            if board[r][c] != '.' {
                let d = map.entry(board[r][c]).or_insert(Digit::new());

                if !d.is_valid(r, c) {
                    return false;
                }
            
                d.add(r, c);
            }
            
            c += 1;

            if c >= lines {
                r += 1;
                c = 0;
            }
        }

        true
    }
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
