pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut sorted = nums.clone();
        sorted.sort_unstable();

        let mut gap: Option<i32> = None;
        let mut sum: Option<i32> = None;     

        for (i, n) in sorted.iter().enumerate() {
            if i + 2 >= sorted.len() {
                continue;
            }

            let new_sum = Self::two_sum_closest(&sorted[i + 1..], target - n) + n;
            let new_gap = (target - new_sum).abs();

            if gap.is_none() || new_gap < gap.unwrap() { 
                gap = Some(new_gap);
                sum = Some(new_sum);
            }
        }

        sum.unwrap()
    }

    fn two_sum_closest(nums: &[i32], target: i32) -> i32 {
        let mut gap: Option<i32> = None;
        let mut sum: Option<i32> = None;
        
        let mut head = 0;
        let mut tail = nums.len() - 1; 
    
        while head < tail {
           let new_sum = nums[head] + nums[tail];
           let new_gap = target - new_sum;

           if gap.is_none() || new_gap.abs() < gap.unwrap() {
               gap = Some(new_gap.abs());
               sum = Some(new_sum);
           }

           if new_gap == 0 {
               return new_sum;
           } else if new_gap > 0 {
               head += 1; 
           } else {
               tail -= 1;
          }
        }

        sum.unwrap()
    }
}

fn main() {
    assert_eq!(2, Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
}
