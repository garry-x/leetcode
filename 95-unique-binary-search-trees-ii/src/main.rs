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

type NodeVec = Vec<Option<Rc<RefCell<TreeNode>>>>;

impl Solution {
    pub fn generate_trees(n: i32) -> NodeVec{
        generate_rec(1, n)
    }
}

pub fn generate_rec(start: i32, end: i32) -> NodeVec{
    if start > end {
        return vec![None];
    }

    let mut res = Vec::new();

    for i in start..end + 1 {
        let mut left = generate_rec(start, i - 1);
        let mut right = generate_rec(i + 1, end);
        
        for l in left.iter_mut() {
            for r in right.iter_mut() {
                let mut node = TreeNode::new(i);
                if l.is_some() {
                    node.left = Some(l.as_ref().unwrap().clone());
                }
                if r.is_some() {
                    node.right = Some(r.as_ref().unwrap().clone());
                }
                res.push(Some(Rc::new(RefCell::new(node))));
            }
        }

    }

    res
}

pub fn print_tree(root: &Option<Rc<RefCell<TreeNode>>>, level: usize, left: bool) {
    if root.is_none() {
        return;
    }
    let prefix = vec![' '; level].iter().collect::<String>();
    if left {
        println!("{}->{}", prefix, root.as_ref().unwrap().borrow().val);
    } else {
        println!("{}|->{}", prefix, root.as_ref().unwrap().borrow().val);
    }
    print_tree(&root.as_ref().unwrap().borrow().left, level + 1, true);
    print_tree(&root.as_ref().unwrap().borrow().right, level + 1, false);
}

use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();

    assert_eq!(1, Solution::generate_trees(1).len());
    assert_eq!(2, Solution::generate_trees(2).len());
    assert_eq!(5, Solution::generate_trees(3).len());
    assert_eq!(14, Solution::generate_trees(4).len());
    
    println!("Time elapsed:{} us", SystemTime::now().duration_since(start).unwrap().as_micros());
}
