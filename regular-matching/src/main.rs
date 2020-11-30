use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut m: HashMap<(usize, usize), bool> = HashMap::new();
        return Self::is_match_inner(&s.chars().collect(),
                                    0, &p.chars().collect(), 0, &mut m);
    }

    fn is_match_inner(s_vec: &Vec<char>, i: usize, 
                      p_vec: &Vec<char>, j: usize,
                      m: &mut HashMap<(usize, usize), bool>) -> bool {
        if j >= p_vec.len() {
            return i >= s_vec.len();
        }
        
        let mut m_flag;

        if i >= s_vec.len(){
            m_flag = false;
        } else {
            m_flag = p_vec[j] == '.' || s_vec[i] == p_vec[j];
        }
       
        if j + 1 < p_vec.len() && p_vec[j + 1] == '*'{
            if m_flag {
                let p1 = Self::try_match(s_vec, i + 1, p_vec, j, m);
                let p2 = Self::try_match(s_vec, i, p_vec, j + 2, m);
                return p1 || p2;
            } else {
                return Self::try_match(s_vec, i, p_vec, j + 2, m);
            }
        } else {
            if m_flag {
                return Self::try_match(s_vec, i + 1, p_vec, j + 1, m);
            } else {
                return false;
            }
        }
    }

    fn try_match(s_vec: &Vec<char>, i: usize, 
                 p_vec: &Vec<char>, j: usize,
                 m: &mut HashMap<(usize, usize), bool>) -> bool {
        let mut res: bool;

        match m.get(&(i, j)) {
            Some(value) => res = *value,
            _ => {
                res = Self::is_match_inner(s_vec, i, p_vec, j, m);
                m.insert((i, j), res);
            },
        }

        res
    }
}

fn main() {
    assert!(false == Solution::is_match(String::from("aa"), String::from("a")));
    assert!(true == Solution::is_match(String::from("aa"), String::from("a*")));
    assert!(true == Solution::is_match(String::from("ab"), String::from(".*")));
    assert!(true == Solution::is_match(String::from("aab"), String::from("c*a*b")));
    assert!(false == Solution::is_match(String::from("mississippi"), String::from("mis*is*p*.")));
    assert!(true == Solution::is_match(String::from("bbbba"), String::from(".*a*a")));
    assert!(true == Solution::is_match(String::from("aaaab"), String::from("a*ab")));
}
