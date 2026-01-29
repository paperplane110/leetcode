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

function merge(head1: ListNode | null, head2: ListNode | null): ListNode | null {
    const dummy = new ListNode(0)
    let cur = dummy
    while (head1 && head2) {
        if (head1.val < head2.val) {
            cur.next = head1;
            head1 = head1.next;
        } else {
            cur.next = head2;
            head2 = head2.next
        }
        cur = cur.next
    }
    cur.next = head1 ? head1 : head2;
    return dummy.next
}

function mergeKLists(lists: Array<ListNode | null>): ListNode | null {
    if (lists.length === 0) {
        return null
    }
    
    let curList: ListNode | null = lists.pop()!;
    while (lists.length > 0) {
        const newList = lists.pop()!
        if (!lists) {
            continue;
        }
        curList = merge(curList, newList)
    }
    return curList
};