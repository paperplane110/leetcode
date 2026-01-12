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
    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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

    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        // get length
        let mut len = 0;
        let mut curr = &head;
        while let Some(node) = curr {
            len += 1;
            curr = &node.next;
        }

        // get middle node
        let mut mid = &mut head;
        // ceil division （len + 2 - 1）/ 2
        for i in 0..(len + 1) / 2 - 1 {
            mid = &mut mid.as_mut().unwrap().next;
        }

        let second_half = mid.as_mut().unwrap().next.take();

        let mut reversed_second = Self::reverse_list(second_half);

        let mut first_half = &head;
        while let (Some(n1), Some(n2)) = (first_half, reversed_second) {
            if n1.val != n2.val {
                return false;
            }
            first_half = &n1.next;
            reversed_second = n2.next.clone()
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
}

