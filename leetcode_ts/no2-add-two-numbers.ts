/**
 * Definition for singly-linked list.
 * */

class ListNode {
    val: number
    next: ListNode | null
    constructor(val?: number, next?: ListNode | null) {
        this.val = (val === undefined ? 0 : val)
        this.next = (next === undefined ? null : next)
    }
}

// function addTwoNumbers(l1: ListNode | null, l2: ListNode | null): ListNode | null {
//     const dummy = new ListNode(-1);
//     let curr = dummy;
//     let carry = 0
//     while (l1 || l2 || carry !== 0) {
//         const n1 = l1 ? l1.val : 0;
//         const n2 = l2 ? l2.val : 0;
//         curr.next = new ListNode((n1 + n2 + carry) % 10);
//         curr = curr.next
//         carry = Math.floor((n1 + n2 + carry) / 10)
//         if (l1) {
//             l1 = l1.next;
//         }
//         if (l2) {
//             l2 = l2.next;
//         }
//     }
//     return dummy.next;
// };

function addTwoNumbers(l1: ListNode | null, l2: ListNode | null): ListNode | null {
    const dummy = new ListNode(-1);
    let curr = dummy;
    let carry = 0
    while (l1 || l2 || carry !== 0) {
        if (l1) {
            carry += l1.val;
            l1 = l1.next;
        }
        if (l2) {
            carry += l2.val;
            l2 = l2.next;
        }
        curr.next = new ListNode((carry) % 10);
        curr = curr.next
        carry = Math.floor((carry) / 10)
    }
    return dummy.next;
};

function arrayToListNode(a: number[]): ListNode | null {
    const prehead = new ListNode(0)
    let node = prehead
    for (let i = 0; i < a.length; i++) {
        node.next = new ListNode(a[i])
        node = node.next;
    }
    node.next = null;
    return prehead.next;
}

(function main() {
    const a1 = [2, 4, 3];
    const a2 = [5, 6, 4];
    const l1 = arrayToListNode(a1);
    const l2 = arrayToListNode(a2)
    addTwoNumbers(l1, l2);
})()