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

struct Solution;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = root.unwrap();
        let root = root.borrow();
        Self::compare(&root.left, &root.right)
    }

    pub fn compare(left: &Option<Rc<RefCell<TreeNode>>>, right: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left, right) {
            (Some(left), Some(right)) => {
                let p = left.borrow();
                let q = right.borrow();
                return p.val == q.val && Self::compare(&p.left, &q.right) && Self::compare(&p.right, &q.left);
            }
            (None, None) => true,
            _ => false
        }
    }
}

fn main() {
    println!("Hello, world!");
}
