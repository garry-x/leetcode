pub struct Solution {}

//Definition for singly-linked list.
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
	pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut cur = &mut dummy_head;
        loop {
            let mut min_idx = 0;
            let mut min_val = std::i32::MAX; 
            let mut found = false;

            for (i, head) in lists.iter_mut().enumerate() {
                if head.is_some() && head.as_ref().unwrap().val < min_val {
                    min_idx = i;
                    min_val = head.as_ref().unwrap().val;
                    found = true;
                }
            }

            if !found {
                // get the end of all arrays
                break;
            }

            // the ith node is the least node
            let mut new_node = lists.swap_remove(min_idx);
            let mut next = new_node.as_mut().unwrap().next.take(); 
            
            // forward result list
            cur.next = new_node;
            cur = cur.next.as_mut().unwrap();
            
            if next.is_some() {
                lists.push(next);
            }
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

//Input: lists = [[1,4,5],[1,3,4],[2,6]]
//Output: [1,1,2,3,4,4,5,6]
fn main() {
	Solution::print_list(&Solution::merge_k_lists(vec![Solution::make_list(&vec![1, 4, 5], 0),
                                                       Solution::make_list(&vec![1, 3, 4], 0),
                                                       Solution::make_list(&vec![2, 6], 0)]));

	Solution::print_list(&Solution::merge_k_lists(vec![]));
	Solution::print_list(&Solution::merge_k_lists(vec![Solution::make_list(&vec![], 0),]));
}
