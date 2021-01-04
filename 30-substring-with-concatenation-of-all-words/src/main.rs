use std::collections::HashMap;
use std::collections::hash_map::Entry;

// LEARN FROM: https://github.com/aylei/leetcode-rust/blob/master/src/solution/s0030_substring_with_concatenation_of_all_words.rs

struct WordRef {
    expect: i32,
    count: i32,
}

impl WordRef {
    fn new(expect: i32, count: i32) -> Self {
        WordRef { expect, count }
    }

    fn inc_expect(&mut self) { 
        self.expect += 1;
    }

    fn inc_ref(&mut self) {
        self.count += 1;
    }

    fn clear_ref(&mut self) {
        self.count = 0;
    }

    fn is_exhausted(&self) -> bool {
        self.count > self.expect
    }
}

pub struct Solution {}

impl Solution {
    pub fn find_substring(s: String,
                          words: Vec<String>) -> Vec<i32> {
        if words.len() < 1 {
            return vec![];
        }

        let word_len = words[0].len();

        if word_len < 1 {
            return vec![];
        }

        let total_len = word_len * words.len();

        // put words into hashmap
        let mut map: HashMap<&str, WordRef> = HashMap::new();
        for w in words.iter() {
            map.entry(w).or_insert(WordRef::new(0, 0)).inc_expect();
        }

        let mut out: Vec<i32> = Vec::new();

        let mut i = 0;
        'outer: loop {
            if i + total_len - 1 >= s.len() {
                break;
            }
            
            // check s[i..i + total_len] to find a match
            let mut j = 0;
            let mut is_match = true;

            'inner: loop {
                if j >= total_len {
                    break 'inner;
                }

                match map.entry(&s[(i + j)..(i + j + word_len)]) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().inc_ref();

                        if entry.get().is_exhausted() {
                            is_match = false;
                            break 'inner;
                        }
                    },
                    Entry::Vacant(entry) => {
                        is_match = false;
                        break 'inner;
                    }
                }

                j += word_len;
            }

            if is_match {
                out.push(i as i32);
            }

            // clear map
            map.iter_mut().for_each(|(_, v)| v.clear_ref());    

            i += 1;
        }

        out
    }

    fn get_strings(strs: Vec<&str>) -> Vec<String>{
        let mut temp: Vec<String> = Vec::new();
        
        for s in strs {
            temp.push(s.to_string());
        }

        temp
    }
}

fn main() {
    println!("{:?}", Solution::find_substring("barfoothefoobarman".to_string(),
                                              Solution::get_strings(vec!["foo", "bar"])));
    println!("{:?}", Solution::find_substring("wordgoodgoodgoodbestword".to_string(),
                                              Solution::get_strings(vec!["word", "good", "best", "word"])));
    println!("{:?}", Solution::find_substring("barfoofoobarthefoobarman".to_string(),
                                              Solution::get_strings(vec!["bar", "foo", "the"])));
    println!("{:?}", Solution::find_substring("wordgoodgoodgoodbestword".to_string(),
                                              Solution::get_strings(vec!["word", "good", "best", "good"])));
    println!("{:?}", Solution::find_substring("ababababab".to_string(),
                                              Solution::get_strings(vec!["ababa", "babab"])));
    println!("{:?}", Solution::find_substring("bcabbcaabbccacacbabccacaababcbb".to_string(),
                                              Solution::get_strings(vec!["c","b","a","c","a","a","a","b","c"])));
}
