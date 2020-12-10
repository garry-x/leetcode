pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // special case negative integer
        if x < 0 {
			return false;
        }
        
        // special case: zero 
        if x == 0 {
            return true;
        }
        
        // find the length of the integer
        let mut max = 1i64;

        while (x as i64 / max) > 0 {
            max *= 10;
        }
        max /= 10;

        // approach from min && max
        let mut min = 1i64; 
        
        while min < max {
            let mut left = (x as i64 / max) % 10;
            let mut right = (x as i64 / min) % 10;

            if left != right {
                return false;
            }

            max /= 10;
            min *= 10;
        }

        true
    }
}

fn main() {
    assert!(true == Solution::is_palindrome(121));
    assert!(false == Solution::is_palindrome(-121));
    assert!(false == Solution::is_palindrome(10));
    assert!(false == Solution::is_palindrome(-101));
}
