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

function merge(left: ListNode | null, right: ListNode | null) {
    const dummy = new ListNode(0);
    let p = dummy;
    while (left && right) {
        if (left.val <= right.val) {
            p.next = left;
            left = left.next;
        } else {
            p.next = right;
            right = right.next;
        }
        p = p.next;
    }
    p.next = left || right;

    while (p.next) {
        p = p.next;
    }
    return [dummy.next, p];
}

function splitList(head: ListNode | null, step: number) {
    for (let i = 0; i < step - 1 && head; i++) {
        head = head.next;
    }

    if (!head || !head.next) {
        return null
    }

    const nxt = head.next;
    head.next = null;
    return nxt
}

function sortList(head: ListNode | null): ListNode | null {
    if (!head || !head.next) {
        return head;
    }

    let n = 0;
    let p: ListNode | null = head;
    while (p) {
        n++;
        p = p.next;
    }

    const dummy = new ListNode(0);
    dummy.next = head;

    for (let step = 1; step < n; step = step * 2) {
        let tail  = dummy;
        let cur: ListNode | null = dummy.next;
        while (cur) {
            const head1 = cur;
            const head2 = splitList(head1, step);
            cur = splitList(head2, step)
            const [newHead, newTail] = merge(head1, head2);
            tail.next = newHead;
            tail = newTail!;
        }
    }
    return dummy.next
}

// function sortList(head: ListNode | null): ListNode | null {
//     if (!head || !head.next) {
//         return head;
//     }
//     let n = 0;
//     let p: ListNode | null = head;
//     while (p) {
//         n++;
//         p = p.next;
//     }
//     const mid = Math.floor(n / 2);
//     let midNode = head;
//     for (let i = 0; i < mid; i++) {
//         midNode = midNode.next;
//     }
//     const head2 = midNode.next;
//     midNode.next = null;
//     const left = sortList(head);
//     const right = sortList(head2);
//     return merge(left, right);
// };