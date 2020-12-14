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
    // Input: head = [1,2,3,4]
    // Output: [2,1,4,3]
    // The number of nodes in the list is in the range [0, 100].
    // 0 <= Node.val <= 100
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode {
            val: 0,
            next: head,
        }); 
        let mut cur = &mut dummy_head;
        
        loop {
            if cur.next.is_none() || cur.next.as_ref().unwrap().next.is_none() {
                break;
            }

            let mut n1 = cur.next.take();
            let mut n2 = n1.as_mut().unwrap().next.take();
            let n3 = n2.as_mut().unwrap().next.take(); 

            // cur -> n2 -> n1 -> n3
            n1.as_mut().unwrap().next = n3;
            n2.as_mut().unwrap().next = n1;
            cur.next = n2;

            cur = cur.next.as_mut().unwrap().next.as_mut().unwrap();
        }

        dummy_head.next
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
            next: Self::make_list(nums, index + 1),
        }))
    }
}

fn main() {
    Solution::print_list(&Solution::swap_pairs(Solution::make_list(&vec![1, 2, 3, 4], 0)));
    Solution::print_list(&Solution::swap_pairs(Solution::make_list(&vec![], 0)));
    Solution::print_list(&Solution::swap_pairs(Solution::make_list(&vec![1], 0)));
    Solution::print_list(&Solution::swap_pairs(Solution::make_list(&vec![1, 2, 3, 4, 5], 0)));
}
