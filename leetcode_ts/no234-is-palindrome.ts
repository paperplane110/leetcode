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

function middleNode(head: ListNode | null): ListNode | null {
    let slow = head, fast = head;
    while (slow !== null && fast !== null && fast.next !== null) {
        slow = slow.next;
        fast = fast.next.next;
    }
    return slow;
}

function reverseList(head: ListNode | null): ListNode | null {
    let prev = null;
    let curr = head;
    while (curr !== null) {
        const nxt = curr.next;
        curr.next = prev;
        prev = curr;
        curr = nxt;
    }
    return prev
}

function isPalindrome(head: ListNode | null): boolean {
    const mid = middleNode(head);
    let p2 = reverseList(mid);
    while (p2 !== null) {
        if (head?.val !== p2.val) {
            return false
        }
        head = head.next;
        p2 = p2.next;
    }
    return true
};