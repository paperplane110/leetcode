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

function reverseKGroup(head: ListNode | null, k: number): ListNode | null {
    const dummy = new ListNode(0);
    dummy.next = head;
    let p0 = dummy;

    const isKExists = (node: ListNode | null, k: number) => {
        for (let i = 0; i < k; i++) {
            if (!node || !node.next) {
                return false;
            }
            node = node.next;
        }
        return true;
    }

    while (isKExists(p0, k)) {
        let pre = null
        let cur = p0.next;
        for (let i = 0; i < k; i++) {
            const nxt = cur!.next;
            cur!.next = pre;
            pre = cur;
            cur = nxt;
        }
        const pnxt = p0.next;
        p0.next!.next = cur;
        p0.next = pre;
        p0 = pnxt;
    }

    return dummy.next;
};

// k = 4
// d - 1 - 2 - 3 - 4 - null
// p0  c   nxt    
//     1 - 2 - 3 - 4 - ...

