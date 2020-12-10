use rand::Rng;
use std::io;

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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::inter_add(l1, l2, 0)
    }

    fn inter_add(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
        if l1 == None && l2 == None && carry == 0{
            return None;
        }

        let (a, n1) = Self::extract(l1);
        let (b, n2) = Self::extract(l2);

        let sum = a + b + carry;

        let node = Box::new(ListNode {
            val: sum % 10,
            next: Self::inter_add(n1, n2, sum / 10),
        });

        Some(node)
    }

    fn extract (l1: Option<Box<ListNode>>) -> (i32, Option<Box<ListNode>>) {
        if let Some(n) = l1 {
            return (n.val, n.next);
        } else {
            return (0, None);
        }
    }

}

fn make_list(nums: u32) -> Option<Box<ListNode>> {
    let mut rng = rand::thread_rng();

    if nums == 0 {
        return None
    }

    let node = Some(Box::new(ListNode {
        val: rng.gen_range(0, 10),
        next: make_list(nums - 1),
    }));

    node
}

fn print_list(l1: &Option<Box<ListNode>>) {
    if let Some(n) = l1 {
        print!("{} -> ", n.val);
        print_list(&n.next);
    } else {
        println!("None"); 
    }
}

fn main() {
    let l1 = make_list(9);
    print_list(&l1);
    
    let l2 = make_list(9);
    print_list(&l2);
        
    print_list(&Solution::add_two_numbers(l1, l2));
}
