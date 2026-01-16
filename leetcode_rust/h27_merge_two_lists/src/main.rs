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
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre_head = ListNode::new(-1);
        let mut cur = &mut pre_head;
        while let (Some(node1), Some(node2)) = (&list1, &list2) {
            if node1.val <= node2.val {
                // 相当于 cur.next = list1 把 list1 全部接到 cur.next 上
                // 此时 list1 不拥有任何东西了
                cur.next = list1;

                // 而 take 做了两件事
                // 1 让 cur.next.next 填入了一个 None 
                // 2 把 cur.next 之后的链表拆下来又给回到 list1 所有
                // 而正是因为 take 对 cur.next 之后的改动，所以需要 as_mut()
                list1 = cur.next.as_mut().unwrap().next.take();
            } else {
                cur.next = list2;
                list2 = cur.next.as_mut().unwrap().next.take();
            }
            cur = cur.next.as_mut().unwrap();
        }
        cur.next = if list1.is_some() {list1} else {list2};
        pre_head.next
    }
}

fn main() {
    println!("Hello, world!");
}
