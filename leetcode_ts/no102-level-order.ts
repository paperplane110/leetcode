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

function levelOrder(root: TreeNode | null): number[][] {
    let ans: number[][] = [];
    if (!root) {
        return ans;
    }
    let nextLevel = [root];
    while (nextLevel.length !== 0) {
        const currLevel = nextLevel;
        nextLevel = []
        const currAns = [];
        for (let node of currLevel) {
            currAns.push(node.val)
            if (node.left) {
                nextLevel.push(node.left)
            }
            if (node.right) {
                nextLevel.push(node.right)
            }
        }
        ans.push(currAns)
    }
    return ans
};