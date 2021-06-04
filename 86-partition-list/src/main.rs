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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        
        let mut v1: Vec<i32> = Vec::new();
        let mut v2: Vec<i32> = Vec::new();
        
        {
            let mut cur = &head;
            while cur.is_some() {
                let value = cur.as_ref().unwrap().val;
                if value < x {
                    v1.push(value);
                } else {
                    v2.push(value);
                }
                cur = &cur.as_ref().unwrap().next;
            }
        }

        let mut new_head = None;

        {
            let mut cur = &mut new_head;
            for v in v1.iter().chain(v2.iter()) {
                *cur = Some(Box::new(ListNode::new(*v)));
                cur = &mut cur.as_mut().unwrap().next;
            }
        }

        new_head
    }
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

pub fn print_list(head: &Option<Box<ListNode>>) {
    let mut cur = head;

    while cur.is_some() {
        print!("{} ", cur.as_ref().unwrap().val);
        cur = &cur.as_ref().unwrap().next;
    }
    println!("");
}

use std::time:: SystemTime;

fn main() {
    let start = SystemTime::now();

    Solution::partition(make_list(&vec![1, 4, 3, 2, 5, 2], 0), 3);
    Solution::partition(make_list(&vec![1, 4, 3, 3, 2, 5, 2], 0), 10);
    Solution::partition(None, 10);
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
