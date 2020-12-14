pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut buf: Vec<char> = Vec::new();
        let mut out: Vec<char> = Vec::new();
        let mut res: Vec<String> = Vec::new();
        Self::gen_inner(0, n as usize, &mut buf, &mut out, &mut res);
        res
    }

    fn gen_inner(cur: usize, max: usize, buf: &mut Vec<char>,
                 out: &mut Vec<char>, res: &mut Vec<String>) {
        let mut lflag = true;
        let mut rflag = true;

        if cur >= max {
            lflag = false;
        }

        if buf.len() == 0 {
            rflag = false;
        }

        if !lflag && !rflag {
            // end of a choice
            res.push(out.iter().collect::<String>());
            return;
        }

        if lflag {
            //step left
            buf.push('(');
            out.push('(');
            Self::gen_inner(cur + 1, max, buf, out, res);

            //step back
            out.pop();
            buf.pop();
        }

        if rflag {
            //step right
            let v = buf.pop();
            out.push(')');
            Self::gen_inner(cur, max, buf, out, res);

            //step back 
            out.pop();
            if v.is_some() {
                buf.push(v.unwrap());
            }
        }
    }        
}

fn main() {
    println!("{:?}", Solution::generate_parenthesis(3));
    println!("{:?}", Solution::generate_parenthesis(1));
    println!("{:?}", Solution::generate_parenthesis(8));
}
