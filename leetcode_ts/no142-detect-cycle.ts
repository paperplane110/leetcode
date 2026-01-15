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

function detectCycle(head: ListNode | null): ListNode | null {
    let slow = head, fast = head;
    while (true) {
        if (fast === null || fast.next === null) {
            return null
        }
        slow = slow!.next;
        fast = fast.next.next
        if (fast == slow) {
            break
        }
    }
    let p3 = head;
    while (p3 != slow) {
        p3 = p3!.next;
        slow = slow!.next;
    }
    return p3;
};