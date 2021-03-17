pub struct Solution {}

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        digits.reverse();
        
        let mut add = 1;
        for d in digits.iter_mut() {
            let v = *d + add;
            add =  v / 10;
            *d = v % 10; 
        }

        if add != 0 {
            digits.push(add);
        }

        digits.reverse();

        digits
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();

    println!("{:?}", Solution::plus_one(vec![1, 2, 3]));
    println!("{:?}", Solution::plus_one(vec![1]));
    println!("{:?}", Solution::plus_one(vec![9, 9, 9]));
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
