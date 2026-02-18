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

function isValidBST(root: TreeNode | null, left = -Infinity, right = Infinity): boolean {
    if (root == null) {
        return true;
    }
    const x = root.val;
    return left < x && x < right
        && isValidBST(root.left, left, x)
        && isValidBST(root.right, x, right)
};

/**
 * 获取左子树最大值，且小于当前 val
 * 获取右子树最小值，且大于当前 val
 */