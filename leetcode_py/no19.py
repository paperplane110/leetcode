"""
删除倒数第n个节点
"""
from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None) -> None:
        self.val = val
        self.next = next


def removeNthFromEnd(head: Optional[ListNode], n: int) -> Optional[ListNode]:
    dummy = ListNode(next=head)
    p, q = dummy, head
    for _ in range(n):
        q = q.next

    while q:
        q = q.next
        p = p.next

    p.next = p.next.next
    return dummy.next
