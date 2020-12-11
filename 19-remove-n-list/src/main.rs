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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut dummy_head = Some(Box::new(ListNode {
            val: -1,
            next: head,
        }));
        
        let mut len = 0;
        {
            let mut cur = &dummy_head.as_ref().unwrap().next;
            while cur.is_some() {
                cur = &cur.as_ref().unwrap().next;
                len += 1;
            }
        }

        let pos = len - n as usize;
        {
            let mut prev = &mut dummy_head;

            for _ in 0..pos {
                prev = &mut prev.as_mut().unwrap().next;   
            }

            let mut cur = &mut prev.as_mut().unwrap().next;
            prev.as_mut().unwrap().next = cur.as_mut().unwrap().next.take();
        }

        dummy_head.unwrap().next
    }

    pub fn print_list(head: &Option<Box<ListNode>>) {
        let mut cur = head;

        while cur.is_some() {
            print!("{}->", cur.as_ref().unwrap().val);
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
            next: Self::make_list(nums, index + 1),
        }))
    }
}

fn main() {
    Solution::print_list(&Solution::remove_nth_from_end(Solution::make_list(&vec![1, 2, 3, 4, 5], 0), 2));
    Solution::print_list(&Solution::remove_nth_from_end(Solution::make_list(&vec![1, 2, 3, 4, 5], 0), 5));
    Solution::print_list(&Solution::remove_nth_from_end(Solution::make_list(&vec![1, 2], 0), 1));
}
