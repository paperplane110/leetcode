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

function removeNthFromEnd(head: ListNode | null, n: number): ListNode | null {
    const dummy = new ListNode(0)
    dummy.next = head

    let p1 = head, p2 = dummy;
    let ahead = 1;

    while (p1) {
        p1 = p1.next;
        if (ahead > n) {
            p2 = p2.next!;
        } else {
            ahead += 1
        }
    }
    if (p2.next) {
        p2.next = p2.next.next;
    }
    return dummy.next
};