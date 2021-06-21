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
    pub fn reverse_between(mut head: Option<Box<ListNode>>,
                           left: i32, right: i32) -> Option<Box<ListNode>> {
        let (left, right) = (left as usize - 1, right as usize - 1);
                
        if left == right {
            return head;
        }
        
        let mut ints: Vec<i32> = Vec::new();
        {
            let mut cur = &head;
            while cur.is_some() {
                ints.push(cur.as_ref().unwrap().val);
                cur = &cur.as_ref().unwrap().next; 
            }
        }

        let mut cur = &mut head;

        for i in 0..ints.len() {
            let tmp;
            if i >= left && i <= right {
                tmp = ints[right - (i - left)];
            } else {
                tmp = ints[i];
            }
            
            cur.as_mut().unwrap().val = tmp;
            cur = &mut cur.as_mut().unwrap().next;
        }

        head
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

use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();

    print_list(&Solution::reverse_between(make_list(&vec![1, 2, 3, 4, 5], 0), 2, 4));
    print_list(&Solution::reverse_between(make_list(&vec![1, 2, 3, 4, 5, 6], 0), 1, 2));
    print_list(&Solution::reverse_between(make_list(&vec![1], 0), 1, 1));

    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
