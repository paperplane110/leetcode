// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut curr = &mut dummy;
        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry != 0 {
            if let Some(n1) = l1 {
                carry += n1.val;
                l1 = n1.next;
            }
            if let Some(node) = l2 {
                carry += node.val;
                l2 = node.next;
            }
            curr.next = Some(Box::new(ListNode::new(carry % 10)));
            carry /= 10;
            curr = curr.next.as_mut()?;
        }
        dummy.next
    }
}

fn main() {
    println!("Hello, world!");
}
