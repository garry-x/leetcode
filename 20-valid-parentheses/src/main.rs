pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let mut buf: Vec<char> = Vec::new();

        for c in chars.iter() {
            if Self::is_left(*c) {
                buf.push(*c);
                continue;
            }

            if Self::is_right(*c) {
                let v = buf.pop();
                
                if v.is_none() {
                    return false;
                }

                if !Self::is_match(v.unwrap(), *c) {
                    return false;
                }
            }
        }

        buf.len() == 0
    }

    fn is_left(c: char) -> bool {
        match c {
            '(' | '[' | '{' => true,
            _ => false,
        }
    }

    fn is_right(c: char) -> bool {
        match c {
            ')' | ']' | '}' => true,
            _ => false,
        }
    }

    fn is_match(l: char, r: char) -> bool {
        match (l, r) {
            ('(', ')') => true,
            ('[', ']') => true,
            ('{', '}') => true,
            _ => false,
        }
    }
}

fn main() {
    assert_eq!(Solution::is_valid("()".to_string()),true);
    assert_eq!(Solution::is_valid("()[]{}".to_string()),true);
    assert_eq!(Solution::is_valid("(]".to_string()),false);
    assert_eq!(Solution::is_valid("([)]".to_string()),false);
    assert_eq!(Solution::is_valid("{[]}".to_string()),true);
}
