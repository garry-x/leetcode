pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0; 
        let mut j = i + 1;

        if nums.len() == 0 {
            return 0;
        }

        loop {
            if j >= nums.len() {
                break;
            }

            if nums[j] != nums[i] {
                i += 1;
            }

            nums[i] = nums[j];
            j += 1;
        }
        
        (i + 1) as i32
    }
}

fn main() {
    println!("output {}", Solution::remove_duplicates(&mut vec![1, 1, 2]));
    println!("output {}", Solution::remove_duplicates(&mut vec![]));
    println!("output {}", Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]));
}
