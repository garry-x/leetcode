pub struct Solution {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) { 
        let len = matrix.len();

        for i in 0..(len / 2) {
            for j in i..(len - i - 1) {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[len - 1 - j][i];
                matrix[len - 1 - j][i] = matrix[len - 1 - i][len - 1 - j];
                matrix[len - 1 - i][len - 1 - j] = matrix[j][len - 1 - i];
                matrix[j][len - 1 - i] = temp;
            }
        }
    }
}

pub fn build_square(width: usize) -> Vec<Vec<i32>> {
    let mut out = Vec::new();

    for _ in 0..width {
        out.push(Vec::new());
    }

    for i in 0..(width * width) {
        out[i / width].push(i as i32);
    }

    out
}

pub fn print_square(matrix: &Vec<Vec<i32>>) {
    for i in 0..(matrix.len() * matrix.len()) {
        if (i % matrix.len()) == 0 {
            print!("\n");
        }
        print!("{:>3}", matrix[i / matrix.len()][i % matrix.len()]);
    }
    print!("\n");
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();
    
    let mut s = build_square(3);
    print_square(&s);
    Solution::rotate(&mut s); 
    print_square(&s);

    let mut s = build_square(4);
    print_square(&s);
    Solution::rotate(&mut s); 
    print_square(&s);

    let mut s = build_square(5);
    print_square(&s);
    Solution::rotate(&mut s); 
    print_square(&s);

    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
