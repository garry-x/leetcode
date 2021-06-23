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
        travel_rec(&root, &mut res); 
        res        
    }
}

pub fn travel_rec(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
    if node.is_none() {
        return;
    }

    let n = node.as_ref().unwrap().borrow();
    travel_rec(&n.left, res);
    res.push(n.val);
    travel_rec(&n.right, res);
}

use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
