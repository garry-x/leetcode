pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // brute way: merge arrays and find the median
        let res = Self::merge_vec(&nums1, &nums2);
        let pos = res.len() as f64 / 2.0;
        
        if pos.fract() != 0.0 {
            return res[pos.ceil() as usize - 1] as f64;
        } else {
            return (res[pos as usize - 1] + res[pos as usize]) as f64 / 2.0;
        }
    }

    fn merge_vec(nums1: &Vec<i32>, nums2: &Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();

        let mut i = 0;
        let mut j = 0;

        while i < nums1.len() && j < nums2.len() {
            let n1 = nums1[i];
            let n2 = nums2[j];

            if n1 <= n2 {
                res.push(n1);
                i = i + 1;
            } else {
                res.push(n2);
                j = j + 1;
            }
        }

       while i < nums1.len() {
            res.push(nums1[i]);
            i = i + 1;
       }

       while j < nums2.len() {
            res.push(nums2[j]);
            j = j + 1;
       }

       res
    }
}

fn main() {
    println!("result {}", Solution::find_median_sorted_arrays(vec![1, 3], vec![2]));
    println!("result {}", Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]));
    println!("result {}", Solution::find_median_sorted_arrays(vec![0, 0], vec![0, 0]));
    println!("result {}", Solution::find_median_sorted_arrays(vec![], vec![1]));
    println!("result {}", Solution::find_median_sorted_arrays(vec![2], vec![]));
}
