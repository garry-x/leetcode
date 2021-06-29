pub struct Solution {}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        
        let mut stack = Vec::new();
        let mut cur = root.clone();
        
        while cur.is_some() || !stack.is_empty() {
            while let Some(node) = cur {
                cur = node.borrow().left.clone();
                // Rc<RefCell<TreeNode>>
                stack.push(node);
            }
            
            // RefCell<TreeNode>
            let node = stack.pop().unwrap();
            res.push(node.borrow().val);
            cur = node.borrow().right.clone(); 
        }

        res        
    }
}

pub fn build_btree(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.len().trailing_zeros() != 0 {
        return None;
    }

    Some(Rc::new(RefCell::new(TreeNode {
        val: nums[0],
        left: build_btree(&nums[1..nums.len() / 2 + 1]),
        right: build_btree(&nums[nums.len() / 2 + 1..])}))) 
}

use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();
    
    println!("{:?}", Solution::inorder_traversal(build_btree(&(1..2i32.pow(3)).collect::<Vec<i32>>())));
    println!("{:?}", Solution::inorder_traversal(build_btree(&(1..2i32.pow(2)).collect::<Vec<i32>>())));
    println!("{:?}", Solution::inorder_traversal(build_btree(&(1..2i32.pow(1)).collect::<Vec<i32>>())));
    println!("{:?}", Solution::inorder_traversal(None));
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
