pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 0 || head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut vals = Vec::new();
       
        { 
            let mut cur = &head;
            while cur.is_some() {
                vals.push(cur.as_ref().unwrap().val);
                cur = &cur.as_ref().unwrap().next;
            }
        }

        let k = k as usize;
        let n = vals.len();
        let mut i = (n - k % n) % n;

        let mut cur = &mut head;
        while cur.is_some() {
            let node  = cur.as_mut().unwrap();
            node.val = vals[i % n];
            cur = &mut node.next; 
            i += 1;          
        }

        head
    }
}

pub fn print_list(head: &Option<Box<ListNode>>) {
    let mut cur = head;

    while cur.is_some() {
        print!("{} ", cur.as_ref().unwrap().val);
        cur = &cur.as_ref().unwrap().next;
    }
    println!("");
}

pub fn make_list(nums: &Vec<i32>, index: usize) -> Option<Box<ListNode>> {
    if index >= nums.len() {
        return None;
    }

    Some(Box::new(ListNode {
        val: nums[index],
        next: make_list(nums, index + 1),
    }))
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();
    
    print_list(&Solution::rotate_right(make_list(&vec![0], 0), 0));
   
    for i in 0..10 {
        print_list(&Solution::rotate_right(make_list(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 0), i));
    } 

    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
