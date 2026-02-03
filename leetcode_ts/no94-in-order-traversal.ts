/**
 * Definition for a binary tree node. */

class TreeNode {
    val: number
    left: TreeNode | null
    right: TreeNode | null
    constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
        this.val = (val === undefined ? 0 : val)
        this.left = (left === undefined ? null : left)
        this.right = (right === undefined ? null : right)
    }
}



function inorderTraversal(root: TreeNode | null): number[] {
    enum Color {
        WHITE = 0,
        GRAY = 1,
    }
    const stack: [TreeNode | null, Color][] = [[root, Color.WHITE]]
    const ans: number[] = []
    while (stack.length > 0) {
        const [node, color] = stack.pop()!;
        if (!node) {
            continue;
        }
        if (color === Color.WHITE) {
            stack.push([node.right, Color.WHITE]);
            stack.push([node, Color.GRAY]);
            stack.push([node.left, Color.WHITE]);
        } else {
            ans.push(node.val);
        }
    }
    return ans;
}

// dfs recursion
// function inorderTraversal(root: TreeNode | null): number[] {
//     if (!root) {
//         return []
//     } else {
//         return [...inorderTraversal(root.left), root.val, ...inorderTraversal(root.right)]
//     }
// };