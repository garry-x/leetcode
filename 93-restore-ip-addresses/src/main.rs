pub struct Solution {}

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        if s.len() < 4 || s.len() > 12 {
            return vec![];
        }
        
        let chars: Vec<char> = s.chars().collect();
        let mut res: Vec<String> = Vec::new();
        restore_rec(&chars, &mut Vec::new(), &mut res);  
        res        
    }
}

pub fn restore_rec(chars: &[char], buf: &mut Vec<String>,
                   res: &mut Vec<String>) {
    if buf.len() == 4  && chars.len() == 0 {
        res.push(buf[0].clone() + "." +
                 &buf[1] + "." +
                 &buf[2] + "." +
                 &buf[3]); 
        return;
    }

    if buf.len() == 4 || chars.len() == 0 {
        return;
    }

    // 1 digit
    buf.push(String::from(chars[..1].iter().collect::<String>()));
    restore_rec(&chars[1..], buf, res);
    buf.pop();
    
    // 2 digits
    if chars.len() > 1 && chars[0] != '0' {
        buf.push(chars[..2].iter().collect::<String>());
        restore_rec(&chars[2..], buf, res);
        buf.pop(); 
    }

    // 3 digits
    if chars.len() > 2 && chars[0] != '0' {
        let s = chars[..3].iter().collect::<String>();
        if s.parse::<i32>().unwrap() < 256 {
            buf.push(s);
            restore_rec(&chars[3..], buf, res);
            buf.pop();
        }
    }
}

use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();
        
    println!("{:?}", Solution::restore_ip_addresses("25525511135".to_string()));
    println!("{:?}", Solution::restore_ip_addresses("0000".to_string()));
    println!("{:?}", Solution::restore_ip_addresses("1111".to_string()));
    println!("{:?}", Solution::restore_ip_addresses("010010".to_string()));
    println!("{:?}", Solution::restore_ip_addresses("101023".to_string()));

    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
