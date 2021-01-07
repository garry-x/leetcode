pub struct Solution {}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = Vec::new();
        stack.push(-1);

        let mut max = 0;

        for (i, c) in s.char_indices() {
            match c {
                '(' => {
                    stack.push(i as i32);
                },
                ')' => {
                    stack.pop();

                    if stack.len() == 0 {
                        stack.push(i as i32);
                    } else {
                        max = std::cmp::max(i as i32 - stack.last().unwrap(), max);
                    }
                },
                _ => ()
            }
        }

        max
    }
}

fn main() {
    assert_eq!(2, Solution::longest_valid_parentheses("(()".to_string()));
    assert_eq!(4, Solution::longest_valid_parentheses(")()())".to_string()));
    assert_eq!(0, Solution::longest_valid_parentheses("".to_string()));
    assert_eq!(2, Solution::longest_valid_parentheses("()(()".to_string()));
    assert_eq!(8, Solution::longest_valid_parentheses("(()()())".to_string()));
    assert_eq!(8, Solution::longest_valid_parentheses("(()()(())".to_string()));
}
