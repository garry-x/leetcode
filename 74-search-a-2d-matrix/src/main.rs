pub struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        
        let mut start = 0;
        let mut end = m * n - 1;
        
        while start < end {
            if target > matrix[end / n][end % n] || 
               target < matrix[start / n][start % n]{
                return false;
            }
            
            let mid = start + ((end - start) / 2); 
            let tmp = matrix[mid / n][mid % n];
            
            if target == tmp {
                return true;
            } else if target > tmp {
                start = if mid == start { start + 1} else { mid };
            } else {
                end = mid;
            }
        }

        start == end && matrix[start / n][start % n] == target 
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();
   
    assert_eq!(true, Solution::search_matrix(vec![vec![1, 3, 5, 7],
                                                  vec![10, 11, 16, 20],
                                                  vec![23, 30, 34, 60]], 3));

    assert_eq!(false, Solution::search_matrix(vec![vec![1, 3, 5, 7],
                                                   vec![10, 11, 16, 20],
                                                   vec![23, 30, 34, 60]], 13));

    assert_eq!(true, Solution::search_matrix(vec![vec![1, 3, 5, 7],
                                                  vec![10, 11, 16, 20],
                                                  vec![23, 30, 34, 60]], 16));

    assert_eq!(false, Solution::search_matrix(vec![vec![1]], 2));

    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
