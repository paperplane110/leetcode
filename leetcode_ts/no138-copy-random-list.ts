/**
 * Definition for _Node. 
 * */
class _Node {
    val: number
    next: _Node | null
    random: _Node | null

    constructor(val?: number, next?: _Node, random?: _Node) {
        this.val = (val===undefined ? 0 : val)
        this.next = (next===undefined ? null : next)
        this.random = (random===undefined ? null : random)
    }
}



function copyRandomList(head: _Node | null): _Node | null {
  // copy node interlace
  let n = head;
  while (n) {
    n.next = new _Node(n.val, n.next, null)
    n = n.next.next
  }

  // copy random
  let cur = head
  while (cur) {
    if (cur.random) {
      cur.next!.random = cur.random.next;
    }
    cur = cur.next!.next
  }

  // splite into two nodeList
  const dummy = new _Node(0)
  let p = dummy;
  let q = head
  while (q) {
    p.next = q.next;  // 把新节点接到新链上
    p = p.next;       // 把新指针，移动到新链的下一个节点
    q.next = q.next?.next // 复原原链表
    q = q.next;
  }
  return dummy.next
};