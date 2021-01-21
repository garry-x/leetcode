pub struct Solution {}

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        // 1. multiply Nth(n-1 to 0) digit with num2
        // 2. add result with shift
        // 3. loop above to end 
        let mut chrs_num1 = num1.chars().collect::<Vec<char>>();
        chrs_num1.reverse();

        let mut chrs_num2 = num2.chars().collect::<Vec<char>>();
        chrs_num2.reverse();

        let mut chrs_out: Vec<char> = Vec::new();
        let mut shift = 0;
        
        for ch in chrs_num2.iter() {
            chrs_out = str_add(chrs_out,
                               str_mul(&chrs_num1, ch.to_digit(10).unwrap()),
                               shift); 
            
            shift += 1;
        }
        
        // trim '0'
        while chrs_out.len() > 1 && *chrs_out.last().unwrap() == '0' {
            chrs_out.pop();
        }

        chrs_out.reverse();
        chrs_out.into_iter().collect::<String>()
    }
}

fn str_mul(chrs: &Vec<char>, d: u32) -> Vec<char> {
    let mut out: Vec<char> = Vec::new(); 
    assert!(chrs.len() >= 1); 

    if d == 0 {
        return vec!['0'];
    }
    
    let mut carry = 0;
    for ch in chrs.iter() {
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

fn str_add(chrs_num1: Vec<char>, 
           chrs_num2: Vec<char>, shift: usize) -> Vec<char> {
    let mut out: Vec<char> = Vec::new();
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
