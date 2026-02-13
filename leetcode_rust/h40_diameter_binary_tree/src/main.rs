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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, max_d: &mut i32) -> i32 {
            match root {
                Some(node) => {
                    let node = node.borrow();
                    let l_depth = dfs(&node.left, max_d) + 1;
                    let r_depth = dfs(&node.right, max_d) + 1;
                    *max_d = (*max_d).max(l_depth + r_depth);
                    return l_depth.max(r_depth);
                }
                None => 0
            }
        }
        let mut max_d = 0;
        dfs(&root, &mut max_d);
        max_d
    }
}

fn main() {
    println!("Hello, world!");
}
