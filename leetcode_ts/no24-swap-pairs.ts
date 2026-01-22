/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */


function swapPairs(head: ListNode | null): ListNode | null {
    const dummy = new ListNode(0);
    dummy.next = head;
    let cur = dummy
    while (cur.next && cur.next.next) {
        const m = cur.next
        const n = cur.next.next.next;
        cur.next = cur.next.next;
        cur.next.next = m;
        cur.next.next.next = n;
        cur = cur.next.next
    }
    return dummy.next
};

// d - 1 - 2 - 3 - 4
// c
//     m       n

// c - 2
//     2 - m
//         m - n
// c -> -> c

// d - 2 - 1 - 3 - 4
//         c