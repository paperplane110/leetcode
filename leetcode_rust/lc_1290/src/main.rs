fn main() {
    let node3 = ListNode::new(1);
    let node2 = ListNode {
        val: 0,
        next: Some(Box::new(node3))
    };
    let head = ListNode {
        val: 1,
        next: Some(Box::new(node2))
    };
    println!("{:?}", Solution::get_decimal_value(Some(Box::new(head))));
}


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

struct Solution;

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut res = 0;
        let mut head = head;
        while let Some(node) = head {
            res = res * 2 + node.val;
            head = node.next;
        }
        res
    }
}