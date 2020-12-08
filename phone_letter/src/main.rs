pub struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let str: Vec<char> = digits.chars().collect();
        let mut b: Vec<char> = Vec::new();
        let mut r: Vec<String> = Vec::new();
        if str.len() > 0 {
            Self::letter_comb_inter(&str, 0, &mut b, &mut r);
        }
        r  
    }

    fn letter_comb_inter(str: &Vec<char>, i: usize,
                         buf: &mut Vec<char>, r: &mut Vec<String>){
        if i >= str.len() {
            r.push(buf.iter().collect::<String>());
            return;
        }
    
        let v = Self::digits_letter(str[i]);

        for c in v.iter() {
            buf.push(*c); 
            Self::letter_comb_inter(str, i + 1, buf, r);
            buf.pop();  
        }
    }

    fn digits_letter(d: char) -> Vec<char> {
        match d {
            '1' => return vec![],
            '2' => return vec!['a', 'b', 'c'],
            '3' => return vec!['d', 'e', 'f'],
            '4' => return vec!['g', 'h', 'i'],
            '5' => return vec!['j', 'k', 'l'],
            '6' => return vec!['m', 'n', 'o'],
            '7' => return vec!['p', 'q', 'r', 's'],
            '8' => return vec!['t', 'u', 'v'],
            '9' => return vec!['w', 'x', 'y', 'z'],
            _ => return vec![],
        }
    }
}

fn main() {
    println!("{:?}", Solution::letter_combinations("23".to_string()));
    println!("{:?}", Solution::letter_combinations("".to_string()));
    println!("{:?}", Solution::letter_combinations("2".to_string()));
}
