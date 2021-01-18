pub struct Solution {}

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();

        let mut out = Vec::new();
        sum2_rec(&candidates[0..], target, &mut Vec::new(), &mut out);
        out
    }

}

fn sum2_rec(cand: &[i32], target: i32, buf: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
    if target == 0 {
        out.push(buf.clone());
        return;
    }

    if target < 0 {
        return;
    }

    let mut prev = None;

    for (i, v) in cand.iter().enumerate() {
        // already calculated
        if prev.is_some() && prev.unwrap() == *v {
            continue;
        }

        buf.push(*v);
        if i + 1 < cand.len() {
            sum2_rec(&cand[i + 1..], target - *v, buf, out);
        } else {
            sum2_rec(&vec![], target - *v, buf, out);
        }
        prev = buf.pop();
    }
}

fn main() {
    println!("Output:{:?}", Solution::combination_sum2(vec![2, 3, 6, 7], 7));
    println!("Output:{:?}", Solution::combination_sum2(vec![2, 3, 5], 8));
    println!("Output:{:?}", Solution::combination_sum2(vec![1], 1));
    println!("Output:{:?}", Solution::combination_sum2(vec![1], 2));
    println!("Output:{:?}", Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8));
    println!("Output:{:?}", Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5));
}
