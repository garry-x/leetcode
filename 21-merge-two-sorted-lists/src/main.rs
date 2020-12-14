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
    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>,
                           mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = Box::new(ListNode::new(0));
        let mut tail = &mut new_head;
        let (mut p1, mut p2) = (&mut l1, &mut l2);

        loop {
            match (&p1, &p2) {
                (Some(_), Some(_)) => {
                    let v1 = p1.as_ref().unwrap().val;
                    let v2 = p2.as_ref().unwrap().val;
                    if v1 < v2 {
                        std::mem::swap(&mut tail.next, &mut p1);
                        tail = tail.next.as_mut().unwrap();
                        std::mem::swap(&mut tail.next, &mut p1);
                    } else {
                        std::mem::swap(&mut tail.next, &mut p2);
                        tail = tail.next.as_mut().unwrap();
                        std::mem::swap(&mut tail.next, &mut p2);
                    }
                },
                (None, Some(_)) => {
                    std::mem::swap(&mut tail.next, &mut p2);
                    break;
                },
                (Some(_), None) => {
                    std::mem::swap(&mut tail.next, &mut p1);
                    break;
                }
                (None, None) => break,
            }
        }

        new_head.next
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
    Solution::print_list(&Solution::merge_two_lists(Solution::make_list(&vec![1, 2, 4], 0), Solution::make_list(&vec![1, 3, 4], 0)));
    Solution::print_list(&Solution::merge_two_lists(Solution::make_list(&vec![], 0), Solution::make_list(&vec![], 0)));
    Solution::print_list(&Solution::merge_two_lists(Solution::make_list(&vec![], 0), Solution::make_list(&vec![0], 0)));
    Solution::print_list(&Solution::merge_two_lists(Solution::make_list(&vec![1], 0), Solution::make_list(&vec![], 0)));
}
