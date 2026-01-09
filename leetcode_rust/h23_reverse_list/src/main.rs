use std::ptr::null;

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

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;
        while let Some(mut node) = curr {
            let next = node.next;
            node.next = prev;
            prev = Some(node);
            curr = next;
        }
        prev
    }
}

pub fn to_list(list: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in list.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

pub fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    while let Some(node) = head {
        v.push(node.val);
        head = node.next;
    }
    v
}

fn main() {
    println!("Hello, world!");
}
