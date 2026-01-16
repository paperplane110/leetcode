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

function mergeTwoLists(list1: ListNode | null, list2: ListNode | null): ListNode | null {
    let head = new ListNode(0);
    let p = head;
    while (true) {
        if (list1 && list2) {
            if (list1.val <= list2.val) {
                p.next = list1;
                list1 = list1.next;
            } else {
                p.next = list2;
                list2 = list2.next;
            }
            p = p.next;
        } else if (list1) {
            // list1 exists, list2 is empty
            p.next = list1;
            break
        } else {
            // list1 is null, list2 exists
            p.next = list2;
            break
        }
    }
    return head.next;
};