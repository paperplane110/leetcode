/**
 * Definition for a binary tree node.
 * class TreeNode {
 *     val: number
 *     left: TreeNode | null
 *     right: TreeNode | null
 *     constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *     }
 * }
 */

/**
 Do not return anything, modify root in-place instead.
 */
function flatten(root: TreeNode | null): void {
    if (root === null) {
        return;
    }
    const stack: TreeNode[] = [];
    stack.push(root);
    let prev: TreeNode | null = null;
    while (stack.length) {
        const curr = stack.pop()!;
        if (prev !== null) {
            prev.left = null;
            prev.right = curr;
        }
        if (curr.right !== null) {
            stack.push(curr.right);
        }
        if (curr.left !== null) {
            stack.push(curr.left);
        }
        prev = curr;
    }
};

/**
 * 先向左遍历到最下面，记录为 tail
 * 然后遍历右侧，
 */