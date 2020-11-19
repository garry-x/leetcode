pub struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut v: i64 = x as i64;
        let mut res = 0i64;

        while v != 0{
            res = res * 10 + (v % 10);
            v /= 10;
        }
    
        if res < -2147483648 || res > 2147483647 {
            return 0;
        }

        res as i32
    }
}

fn main() {
    println!("{}", Solution::reverse(123));
    println!("{}", Solution::reverse(-123));
    println!("{}", Solution::reverse(120));
    println!("{}", Solution::reverse(0));
    println!("{}", Solution::reverse(302));
    println!("{}", Solution::reverse(1534236469));
}
