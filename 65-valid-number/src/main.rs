pub struct Solution {}

impl Solution {
    pub fn is_number(s: String) -> bool {
        let chrs: Vec<char> = s.chars().collect();
        
        let mut i_sig = -1;
        let mut i_dec = -1;
        let mut i_exp = -1;

        for (i, ch) in chrs.iter().enumerate() {
            match ch {
                '+' | '-' => {
                    if i + 1 >= chrs.len() || 
                            (i != 0 && i as i32 != (i_exp + 1)) {
                        return false;
                    }
                    i_sig = i as i32;
                },
                'e' | 'E' => {
                    if i_exp != -1 || i as i32 == (i_sig + 1) || 
                            i + 1 >= chrs.len() {
                        return false;
                    }
                    i_exp = i as i32;
                },
                '.' => {
                    if i_dec != -1 || 
                            (i_exp != -1 && i as i32 > i_exp) {
                        return false;
                    }
                    if i as i32 == i_sig + 1 && 
                            (i + 1 >= chrs.len() || 
                             !chrs[i + 1].is_digit(10)) {
                        return false;
                    }
                    i_dec = i as i32;
                },
                v if v.is_digit(10) => {},
                _ => { return false; }, 
            }
        }
        
        true
    }
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();
    
    assert_eq!(true, Solution::is_number("0".to_string())); 
    assert_eq!(false, Solution::is_number("e".to_string())); 
    assert_eq!(false, Solution::is_number(".".to_string())); 
    assert_eq!(true, Solution::is_number(".1".to_string())); 
    assert_eq!(true, Solution::is_number("+.1".to_string())); 
    assert_eq!(false, Solution::is_number("e3".to_string())); 
    assert_eq!(false, Solution::is_number("abc".to_string())); 
    assert_eq!(false, Solution::is_number("1a".to_string())); 
    assert_eq!(false, Solution::is_number("99e2.5".to_string())); 
    assert_eq!(false, Solution::is_number("--6".to_string())); 
    assert_eq!(false, Solution::is_number("-+3".to_string())); 
    assert_eq!(false, Solution::is_number("95a54e53".to_string())); 
    assert_eq!(true, Solution::is_number("4.".to_string())); 
    assert_eq!(true, Solution::is_number("-.9".to_string())); 
    assert_eq!(true, Solution::is_number("3e+7".to_string())); 
    assert_eq!(true, Solution::is_number("+6e-1".to_string())); 
    assert_eq!(true, Solution::is_number("53.5e93".to_string())); 
    assert_eq!(false, Solution::is_number("53.5e+".to_string())); 
    assert_eq!(false, Solution::is_number("+".to_string())); 
    assert_eq!(true, Solution::is_number("+.3".to_string())); 
    assert_eq!(false, Solution::is_number("+e3".to_string())); 

    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
