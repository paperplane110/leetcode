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
 * 543. 二叉树的直径
 * 给定一棵二叉树，你需要计算它的直径长度。一棵二叉树的直径长度是任意两个结点路径长度中的最大值。这条路径可能穿过也可能不穿过根结点。
 * 
 * 思路：最大直径 = max(left最大深度+右最大深度， 最大直径)
 * 最大直径初始化为0
 */


function diameterOfBinaryTree(root: TreeNode | null): number {
    let maxDiameter = 0;
    function getMaxDepth(node: TreeNode | null): number {
        if (!node) {
            return 0;
        }
        const leftMaxDepth = getMaxDepth(node.left);
        const rightMaxDepth = getMaxDepth(node.right);
        maxDiameter = Math.max(maxDiameter, leftMaxDepth + rightMaxDepth);
        return Math.max(leftMaxDepth, rightMaxDepth) + 1;
    }
    getMaxDepth(root);
    return maxDiameter;
};