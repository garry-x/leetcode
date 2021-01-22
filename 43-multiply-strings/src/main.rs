pub struct Solution {}

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1: String = num1.chars().rev().collect();
        let num2: String = num2.chars().rev().collect();

        let mut out = String::new();
        let mut shift = 0;
        
        for ch in num2.chars() {
            out = str_add(&out,
                          &str_mul(&num1, ch.to_digit(10).unwrap()),
                          shift); 
            
            shift += 1;
        }
         
        out = out.chars().rev().collect::<String>().trim_start_matches('0').to_string();

        match out.len() {
            v if v == 0 => "0".to_string(),
            _ => out,
        }
    }
}

fn str_mul(str1: &String, d: u32) -> String {
    let mut out = String::new(); 

    if d == 0 {
        return "0".to_string();
    }
    
    let mut carry = 0;
    for ch in str1.chars() {
        let mut d1 = ch.to_digit(10).unwrap() * d + carry;
        
        carry = d1 / 10;
        d1 = d1 % 10;

        out.push(DIGITS[d1 as usize]);
    }

    if carry > 0 {
        out.push(DIGITS[carry as usize]);
    }

    out
}

fn str_add(str1: &String, 
           str2: &String, shift: usize) -> String {
    let chrs_num1: Vec<char> = str1.chars().collect();
    let chrs_num2: Vec<char> = str2.chars().collect();

    let mut out = String::new();
    let mut carry = 0;
    
    let mut i = 0; 
    let len = std::cmp::max(chrs_num1.len(), shift + chrs_num2.len());

    while i < len {
        let (v1, v2);

        if i < shift || i >= (chrs_num2.len() + shift) {
            v2 = 0;
        } else {
            v2 = chrs_num2[i - shift].to_digit(10).unwrap();
        }

        if i >= chrs_num1.len() {
            v1 = 0;
        } else {
            v1 = chrs_num1[i].to_digit(10).unwrap();
        }
          
        let sum = v1 + v2 + carry;
        carry = sum / 10;
        out.push(DIGITS[(sum % 10) as usize]);

        i += 1;
    }

    if carry > 0 {
        out.push(DIGITS[carry as usize]);
    }

    out
}

fn main() {
    println!("Output: {}", Solution::multiply("2".to_string(), "3".to_string()));
    println!("Output: {}", Solution::multiply("123".to_string(), "456".to_string()));
    println!("Output: {}", Solution::multiply("10000".to_string(), "1".to_string()));
    println!("Output: {}", Solution::multiply("10000".to_string(), "1000000".to_string()));
    println!("Output: {}", Solution::multiply("10000".to_string(), "0".to_string()));
    println!("Output: {}", Solution::multiply("0".to_string(), "52".to_string()));
}
