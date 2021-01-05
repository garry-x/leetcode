pub struct Solution {}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() <= 1 {
            return;
        }

        let mut i = nums.len() as i32 - 2;
        while i >= 0 {
            if nums[i as usize] < nums[i as usize + 1] {
                // find the orig to swap
                break; 
            }
            i -= 1;
        }

        // nums[i + 1..]  are in descending order
        let mut j = i + 1;
        while j > 0 && (j as usize) < nums.len() {
            if nums[i as usize] >= nums[j as usize] {
                break; 
            }

            j += 1; 
        }
        
        // match pair found 
        if i >= 0 {
            j -= 1;
            nums.swap(i as usize, j as usize);
        }

        // reverse nums[i + 1..] 
        nums[(i + 1) as usize..].reverse();  
    }
}

fn main() {
    Solution::next_permutation(&mut vec![1, 2, 3]);
    Solution::next_permutation(&mut vec![3, 2, 1]);
    Solution::next_permutation(&mut vec![1, 1, 5]);
    Solution::next_permutation(&mut vec![1]);
    Solution::next_permutation(&mut vec![1, 1, 1]);
    Solution::next_permutation(&mut vec![1, 3, 2]);
    Solution::next_permutation(&mut vec![2, 3, 1]);
    Solution::next_permutation(&mut vec![1, 5, 1]);
}
