pub struct Solution {}

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        // Bn-1Bn-2..B0
        // Gn-1Gn-2..G0
        // Gn-1 = Bn-1
        // Gi = Bi ^ Bi+1 (0 <= i <= n -2)
        fn convert_to_gray(v: i32, n: u32) -> i32 {
            if v == 0 || n == 1 {
                return v;
            }
           
            let mut out = v & 1 << (n - 1);
            for i in (0..n-1).rev() {
                out = out | (1 << i & v) ^ ((1 << (i + 1) & v) >> 1);
            }
            out
        }

        let mut out: Vec<i32> = Vec::new();
        for i in 0..2i32.pow(n as u32) {
            out.push(convert_to_gray(i, n as u32));
        }
        out
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();
    
    for i in 1..5 {
        let v = Solution::gray_code(i);
        for x in &v {
            println!("{} {:#06b}", x, x);
        }
        println!("");
    }
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
