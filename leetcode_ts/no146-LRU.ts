class DoubleLinkNode {
    prev: DoubleLinkNode | null = null;
    next: DoubleLinkNode | null = null;
    constructor(public key: number, public value: number) { }
}

class LRUCache {
    capacity: number;
    cache: Map<number, DoubleLinkNode>;
    head: DoubleLinkNode
    tail: DoubleLinkNode

    constructor(capacity: number) {
        this.capacity = capacity;

        this.head = new DoubleLinkNode(0, 0)
        this.tail = new DoubleLinkNode(0, 0)
        this.head.next = this.tail
        this.tail.prev = this.head

        this.cache = new Map()
    }

    get(key: number): number {
        const node = this.cache.get(key)
        if (node) {
            this.moveToHead(node)
            return node.value
        } else {
            return -1
        }
    }

    put(key: number, value: number): void {
        const node = this.cache.get(key)
        if (node) {
            node.value = value;
            this.moveToHead(node);
        } else {
            const newNode = new DoubleLinkNode(key, value)
            this.addToHead(newNode);
            this.cache.set(key, newNode)
            if (this.cache.size > this.capacity) {
                const deletedNode = this.removeTailNode()
                this.cache.delete(deletedNode.key)
            }
        }
    }

    moveToHead(node: DoubleLinkNode) {
        this.removeNode(node)
        this.addToHead(node)
    }

    removeNode(node: DoubleLinkNode) {
        const prev = node.prev
        const nxt = node.next
        prev!.next = nxt
        nxt!.prev = prev
    }

    addToHead(node: DoubleLinkNode) {
       const nxt = this.head.next!
       // head -> new -> next
       this.head.next = node
       node.next = nxt

       // next -> new -> head
       nxt.prev = node
       node.prev = this.head
    }

    removeTailNode(): DoubleLinkNode {
        const lastNode = this.tail.prev!;
        this.removeNode(lastNode)
        return lastNode
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * var obj = new LRUCache(capacity)
 * var param_1 = obj.get(key)
 * obj.put(key,value)
 */