pub struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut pos = -1;

        for (i, v) in nums.iter().enumerate() {
            if target < *v {
                break;
            } else if target == *v {
                return i as i32;
            } else {
                pos = i as i32;
            }
        }

        pos += 1;

        pos
    }
}

fn main() {
    assert_eq!(2, Solution::search_insert(vec![1, 3, 5, 6], 5));
    assert_eq!(3, Solution::search_insert(vec![1, 3, 5, 6], 6));
    assert_eq!(0, Solution::search_insert(vec![1, 3, 5, 6], 1));
    assert_eq!(2, Solution::search_insert(vec![1, 3, 5, 6], 4));
    assert_eq!(4, Solution::search_insert(vec![1, 3, 5, 6], 7));
}
