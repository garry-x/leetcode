pub struct Solution {}

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

use std::collections::HashMap;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head: Option<Box<ListNode>> = None;

        let mut map: HashMap<i32, i32> = HashMap::new();
        {
            let mut prev = &mut new_head;
            let mut cur = &head;
            while cur.is_some() {
                let value = cur.as_ref().unwrap().val;
                
                let entry = map.entry(value).or_insert(0);
                *entry += 1;
                
                if *entry == 1 {
                    *prev = Some(Box::new(ListNode::new(value)));
                    prev = &mut prev.as_mut().unwrap().next;
                }
                cur = &cur.as_ref().unwrap().next;
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
    
    print_list(&Solution::delete_duplicates(make_list(&vec![1, 2, 3, 3, 4, 4, 5], 0)));
    print_list(&Solution::delete_duplicates(make_list(&vec![], 0)));
    print_list(&Solution::delete_duplicates(make_list(&vec![1, 1, 2, 2, 3, 4], 0)));
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
