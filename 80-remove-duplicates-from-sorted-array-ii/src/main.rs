pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut next = 0;

        for i in 0..nums.len() {
            let v = map.entry(nums[i]).or_insert(0);

            if *v < 2 {
                *v += 1;

                if i != next {
                    // swap current value with slots
                    let tmp = nums[i];
                    nums[i] = nums[next];
                    nums[next] = tmp;
                }
                // next slot
                next += 1;
            }
        }

        next as i32
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();

    assert_eq!(Solution::remove_duplicates(&mut vec![1, 1, 1, 2, 2, 3]), 5);
    assert_eq!(Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 1, 2, 3, 3]), 7);
    assert_eq!(Solution::remove_duplicates(&mut vec![1, 2, 3, 4, 5, 6]), 6);

    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
