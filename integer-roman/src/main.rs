pub struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut r = String::new();
        let mut n = num;

        let mut l = 1000;

        while l > 0 {
            let d = n / l;
            
            if d > 0 {
                match l {
                    1000 => r += &Self::to_roman(d, 'M', ' ', ' '),
                    100 => r += &Self::to_roman(d, 'C', 'D', 'M'),
                    10 => r += &Self::to_roman(d, 'X', 'L', 'C'),
                    1 => r += &Self::to_roman(d, 'I', 'V', 'X'),
                    _ =>(),
                };
            }

            n %= l;
            l /= 10;
        }

        r
    }
    
    fn to_roman(d: i32, s1: char, s5: char, s10: char) -> String {
        let mut r = String::new();
        
        if d <= 3 {
            r += &s1.to_string().repeat(d as usize);
        } else if d == 4 {
            r.push(s1);
            r.push(s5);
        } else if d > 4 && d < 9 {
            r.push(s5);
            r += &s1.to_string().repeat(d as usize - 5);
        } else if d == 9 {
            r.push(s1);
            r.push(s10);
        }
        
        r
    }
}

fn main() {
    assert_eq!(Solution::int_to_roman(3), "III".to_string());
    assert_eq!(Solution::int_to_roman(4), "IV".to_string());
    assert_eq!(Solution::int_to_roman(9), "IX".to_string());
    assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
    assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
}
