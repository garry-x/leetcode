pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let vec: Vec<char> = s.chars().collect(); 
        let (mut start, mut end) = (0, 0);
        let mut max_len = 0;
        
        for i in 0..vec.len() {
            let (i1, i2) = Self::inner_longest(&vec, i);
             
            if (i2 - i1 + 1) > max_len {
                max_len = i2 - i1 + 1;
                start = i1;
                end = i2;   
            }
        }

        vec[start..(end + 1)].into_iter().collect()
    }

    // step downside from the mountain top
    fn inner_longest(vec: &Vec<char>, r: usize) -> (usize, usize) {
        let (mut i1, mut i2) = (r as i32, r); 
       
        // put consecutive chars same with root on the top
        while i2 + 1 < vec.len() && vec[i2] == vec[i2 + 1] {
            i2 = i2 + 1;
        }

        while i1 >= 0 && i2 < vec.len() {
            if vec[i1 as usize] != vec[i2] {
                return (i1 as usize + 1, i2 - 1)     
            } else {
                i1 = i1 - 1;
                i2 = i2 + 1;
            }
        }

        ((i1 + 1) as usize, i2 - 1)
    }
}

fn main() {
    println!("result={}", Solution::longest_palindrome(String::from("babad")));
    println!("result={}", Solution::longest_palindrome(String::from("cbbd")));
    println!("result={}", Solution::longest_palindrome(String::from("a")));
    println!("result={}", Solution::longest_palindrome(String::from("ac")));
    println!("result={}", Solution::longest_palindrome(String::from("aabbccbbaa")));
    println!("result={}", Solution::longest_palindrome(String::from("bbb")));
    println!("result={}", Solution::longest_palindrome(String::from("bbbb")));
    println!("result={}", Solution::longest_palindrome(String::from("cabbbad")));
}
