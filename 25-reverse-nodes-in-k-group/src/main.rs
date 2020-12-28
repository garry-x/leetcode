pub struct Solution {}

// LEARN FROM: https://github.com/aylei/leetcode-rust

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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut dummy_node = Some(Box::new(ListNode {
            val: 0,
            next: head,
        }));
        
        let mut cur = &mut dummy_node;

        'outer: loop {
            if cur.is_none() {
                break;
            }

            let mut start = cur.as_mut().unwrap().next.take();
            if start.is_none() {
                break;
            }
            
            let mut tail = &mut start;
            for _ in 0..(k - 1) {
                tail = &mut tail.as_mut().unwrap().next;
                if tail.is_none() {
                    cur.as_mut().unwrap().next = start;
                    break 'outer;
                }
            }

            let mut end = tail.as_mut().unwrap().next.take();
            
            // before: cur x start -> 1 -> 2 -> end
            // after: cur x 2 -> 1 -> start -> end
            end = Self::reverse_group(start, end);
            cur.as_mut().unwrap().next = end;

            for _ in 0..k {
                cur = &mut cur.as_mut().unwrap().next; 
            }
        }

        dummy_node.unwrap().next
    }
    
    // start: the start node of this group  
    // end: the firt node beyond this group
    fn reverse_group(start: Option<Box<ListNode>>,
                     end: Option<Box<ListNode>>) -> Option<Box<ListNode>>{

        let mut prev = start;
        let mut cur = prev.as_mut().unwrap().next.take();

        prev.as_mut().unwrap().next = end;

        loop {
            if cur.is_none() {
                break;
            }
           
            let next = cur.as_mut().unwrap().next.take(); 
            cur.as_mut().unwrap().next = prev;

            prev = cur;
            cur = next;
        }
             
        prev        
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
    Solution::print_list(&Solution::reverse_k_group(Solution::make_list(&vec![1, 2, 3, 4, 5], 0), 3));
    Solution::print_list(&Solution::reverse_k_group(Solution::make_list(&vec![1, 2, 3, 4, 5], 0), 2));
    Solution::print_list(&Solution::reverse_k_group(Solution::make_list(&vec![1, 2, 3, 4, 5], 0), 1));
    Solution::print_list(&Solution::reverse_k_group(Solution::make_list(&vec![1], 0), 1));
}
