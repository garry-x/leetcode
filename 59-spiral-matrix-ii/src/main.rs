pub struct Solution {}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut matrix = vec![vec![0; n]; n]; 

        let mut layer = 0;
        let (mut x, mut y) = (0, 0);

        for i in 1..(n * n + 1) {
            matrix[x][y] = i as i32;

            if x == layer && y < (n - layer - 1) {
                y += 1;
                continue;    
            }
            if y == n - layer - 1 && x < (n - layer - 1) {
                x += 1;
                continue;
            }
            if x == n - layer - 1 && y > layer {
                y -= 1;
                continue;
            }
            if y == layer && x > layer {
                x -= 1;
            }
            if x == layer && y == layer {
                x += 1;
                y += 1;
                layer += 1;
            }
        }

        matrix
    }
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
    
    print_square(&Solution::generate_matrix(1));
    print_square(&Solution::generate_matrix(2));
    print_square(&Solution::generate_matrix(3));
    print_square(&Solution::generate_matrix(4));
    print_square(&Solution::generate_matrix(5));

    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
