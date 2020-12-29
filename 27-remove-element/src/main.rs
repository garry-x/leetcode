pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        let mut j = 0;

        loop {
            if std::cmp::max(j, i) >= nums.len() {
                break;
            }
            
            if nums[i] == val {
                // find a slot
                if nums[j] != val {
                    // switch slot and a value
                    nums[i] = nums[j];
                    nums[j] = val;
                    i += 1;
                }
            } else {
                // step forward when not a slot 
                i += 1;
            }

            j += 1;
        }

        std::cmp::min(nums.len(), i) as i32
    }
}

fn main() {
    println!("output {}", Solution::remove_element(&mut vec![3, 2, 2, 3], 3));
    println!("output {}", Solution::remove_element(&mut vec![0, 1, 2, 3, 0, 4, 2], 2));
    println!("output {}", Solution::remove_element(&mut vec![], 3));
}
