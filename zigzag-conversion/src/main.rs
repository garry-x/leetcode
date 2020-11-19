pub struct Solution {}

// PAYPALISHIRING --> PAHNAPLSIIGYIR
// P   A   H   N
// A P L S I I G
// Y   I   R

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut rows = Vec::new();
        let row_num = num_rows as usize;

        if num_rows <= 1 {
            return s;
        }

        for i in 0..row_num {
            rows.push(Vec::<char>::new());
        }
     
        let l = row_num + row_num - 2;

        for (n, c) in s.char_indices() {
            // calculate the next position
            // 1. each loop = row_num + row_num - 2 chars
            // 2. every loop has a wall and a stair
            // 3. wall position:
            //    row = (n % loop) % row_num 
            // 4. stair positon:
            //    row = row_num - 1 - ((n % loop) % row_num + 1)
            let pos  = n % l; 
             
            if pos < row_num {
                // on the wall
                rows[pos % row_num].push(c);    
            } else {
                rows[row_num -1 - (pos % row_num + 1)].push(c);
            }
        }

        let mut res = String::new();

        for i in 0..row_num {
            let vec = &rows[i];
            res += &(vec.into_iter().collect::<String>());
        }

        res
    }
}

fn main() {
    println!("result={}", Solution::convert(String::from("PAYPALISHIRING"), 3));
    println!("result={}", Solution::convert(String::from("PAYPALISHIRING"), 4));
    println!("result={}", Solution::convert(String::from("A"), 1));
}
