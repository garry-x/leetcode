pub struct Solution {}

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        fn format_line(words: &[String], words_len: usize, max_width: usize,
                       last: bool) -> String {
            assert!(words.len() > 0);
            
            // get count for extra white space
            let mut extra = max_width - words_len - (words.len() - 1);

            // num of pad white spaces for each word
            let mut pads: Vec<usize> = vec![1; words.len()];
            if !last {
                pads[words.len() - 1] = 0;
                // split extra 
                let mut nums = pads.len();
                while nums > 0 && extra > 0 {
                    let pad = match nums { 1 => extra, v => extra / (v - 1) };
                    pads[ match nums { 1 => 0, v => v - 2 } ] += pad;
                    extra -= pad;
                    nums -= 1;             
                }
            } else {
                pads[words.len() - 1] = extra;
            }

            let mut out: String = "".to_string();
            for (i, w) in words.iter().enumerate() {
                out.push_str(w);
                if pads[i] > 0 {
                    out.push_str(&vec![' '; pads[i]].iter().collect::<String>()[..]);
                }
            }
            
            out
        }

        let max_width = max_width as usize;
        
        let mut line_start = 0;
        let mut line_words = 0;
        let mut words_len = 0;

        let mut out: Vec<String> = Vec::new();

        for i in 0..words.len() {
            // try to fill as many words as possible to current line
            if words_len + line_words + words[i].len() <= max_width {
                words_len += words[i].len();
                line_words += 1;
            } else {
                // format line
                out.push(format_line(&words[line_start..i], words_len,
                                     max_width, false)); 

                // start a new line
                line_start = i;
                line_words = 1;
                words_len = words[i].len(); 
            }
        }
        out.push(format_line(&words[line_start..words.len()],
                             words_len, max_width, true)); 

        out
    }
}

use std::time:: SystemTime;

pub fn print_reseult(out: &Vec<String>) {
    for l in out {
        println!("{}", l);
    }
}

fn main() {
    let start = SystemTime::now();

    print_reseult(&Solution::full_justify(vec!["This", "is", "an", "example",
                                              "of", "text", "justification.", "by"].iter().map(|v| v.to_string()).collect(), 16));
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
