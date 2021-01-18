pub struct Solution {}

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        
        let mut out: Vec<Vec<i32>> = Vec::new();
        sum_inner(&candidates[0..], target, &mut Vec::new(), &mut out);
        out     
    }

}

fn sum_inner(cand: &[i32], target: i32, buf: &mut Vec<i32>,
             out: &mut Vec<Vec<i32>>) {
    if target == 0 {
        out.push(buf.clone());
        return;
    }

    if target < cand[0] {
        return;
    }

    for (i, v) in cand.iter().enumerate() {
        if *v > target {
            break;
        }

        buf.push(*v);
        sum_inner(&cand[i..], target - *v, buf , out);
        buf.pop();
    }
}

fn main() {
    println!("Output:{:?}", Solution::combination_sum(vec![2, 3, 6, 7], 7));
    println!("Output:{:?}", Solution::combination_sum(vec![2, 3, 5], 8));
    println!("Output:{:?}", Solution::combination_sum(vec![1], 1));
    println!("Output:{:?}", Solution::combination_sum(vec![1], 2));
}
