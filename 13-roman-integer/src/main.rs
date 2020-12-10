pub struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let v: Vec<char> = s.chars().collect();
       
        let mut r = 0;
        let mut i = 0;
        
        while i < v.len() {
            let mut v1 = Self::to_int(v[i]);

            if i + 1 < v.len() {
                let v2 = Self::to_int(v[i + 1]);
                if v1 < v2 {
                    v1 = 0 - v1;
                }
            }

            r += v1;
            i += 1;
        }

        r
    }

    fn to_int(c: char) -> i32 {
        match c {
            'I' => return 1,
            'V' => return 5,
            'X' => return 10,
            'L' => return 50,
            'C' => return 100,
            'D' => return 500,
            'M' => return 1000,
            _ => return 0,
        }
    }
}

fn main() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
    assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
}
