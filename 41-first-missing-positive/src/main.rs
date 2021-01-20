pub struct Solution {}

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();

        // i + 1 is used: nums[i] = i + 1 
        // i + 1 is target: nums[i] != i + 1 
        for i in 0..len {
            let mut n1 = nums[i] as usize;
            
            // each loop will fill a pos between 0 && len -1
            // overall complexity will be O(n1) + O(n2) -> O(n):
            // n1: the number of digits to be placed to correct location
            // n2: ignore digits(already placed || negative || two large || equal) 
            while n1 > 0 && n1 < len && n1 != i + 1 && nums[n1 - 1] != (n1 as i32) {
                nums[i] = nums[n1 -1];
                nums[n1 - 1] = n1 as i32; 

                n1 = nums[i] as usize
            }
        }

        for i in 0..len {
            if nums[i] as usize != i + 1 {
                return i as i32 + 1;
            }
        }
        
        len as i32 + 1   
    }
}

fn main() {
    assert_eq!(3, Solution::first_missing_positive(vec![1, 2, 0]));
    assert_eq!(2, Solution::first_missing_positive(vec![3, 4, -1, 1]));
    assert_eq!(1, Solution::first_missing_positive(vec![7, 8, 9, 11, 12]));
    assert_eq!(2, Solution::first_missing_positive(vec![7, 8, 1, 11, 12]));
    assert_eq!(4, Solution::first_missing_positive(vec![1, 2, 3]));
    assert_eq!(3, Solution::first_missing_positive(vec![1, 2, 2,4]));
    assert_eq!(2, Solution::first_missing_positive(vec![1]));
    assert_eq!(1, Solution::first_missing_positive(vec![2]));
    assert_eq!(3, Solution::first_missing_positive(vec![-1, 4, 2, 1, 9, 10]));
    assert_eq!(5, Solution::first_missing_positive(vec![2, 2, 4, 0, 1, 3, 3, 3, 4, 3]));
}
