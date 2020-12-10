pub struct Solution {}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut sorted: Vec<i32> = nums.clone();
        sorted.sort_unstable(); 

        let mut buf: Vec<i32> = Vec::new(); 
        let mut res: Vec<Vec<i32>> = Vec::new();

        if nums.len() >= 4 {
            Self::sum_iter(&sorted, 0, target, 4, &mut buf, &mut res);
        }

        res
    }

    fn sum_iter(nums: &Vec<i32>, start: usize, target: i32, level: usize,
                buf: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) { 
        if level == 0 || start >= nums.len() {
            if target == 0 {
                res.push(buf.clone());
            }
            return;    
        }
        
        let mut prev: Option<i32> = None;
        let mut i = start;
        
        while i < nums.len() {
            // skip equal value
            if prev.is_some() && prev.unwrap() == nums[i] {
                i += 1;
                continue;
            }
            
            // jump when nums are not enough for next levels
            if nums.len() - i < level {
                i += 1;
                continue;
            }

            buf.push(nums[i]);
            Self::sum_iter(nums, i + 1, target - nums[i], level - 1, buf, res);
            buf.pop();

            prev = Some(nums[i]);
            i += 1; 
        }
    }
}

fn main() {
    println!("{:?}", Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0));
    println!("{:?}", Solution::four_sum(vec![], 0));
}
