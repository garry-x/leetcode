pub struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 2 {
            return 1;
        }
        
        // ...[step_0][step_1][step_2]...top 
        // step_0 = step_1 + step_2
        // initialization: near top
        let mut step_2 = 1;
        let mut step_1 = 2;
        let mut step_0;
        
        let mut i = n - 3;
        while i >= 0 {
            step_0 = step_1 + step_2;

            // go downstair
            step_2 = step_1;
            step_1 = step_0;
            i -= 1;
        }

        step_1
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();
    
    println!("{}", Solution::climb_stairs(1)); 
    println!("{}", Solution::climb_stairs(2)); 
    println!("{}", Solution::climb_stairs(3)); 
    println!("{}", Solution::climb_stairs(45)); 

    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
