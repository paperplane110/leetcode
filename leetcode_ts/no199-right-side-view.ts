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

function rightSideView(root: TreeNode | null): number[] {
    let currNodes = [];
    if (!root) {
        return [];
    }
    let nextNodes = [root];
    const view = [];
    while (nextNodes.length) {
        currNodes = [...nextNodes];
        nextNodes = []
        for (let i = 0; i < currNodes.length; i++) {
            const node = currNodes[i];
            if (i === currNodes.length - 1) {
                view.push(node.val)
            }
            if (node.left) {
                nextNodes.push(node.left);
            }
            if (node.right) {
                nextNodes.push(node.right);
            }
        }
    }
    return view;
};