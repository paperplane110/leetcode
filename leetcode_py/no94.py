"""
Inorder traversal
"""

from typing import Optional, List


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def inorderTraversal(root: Optional[TreeNode]) -> List[int]:
    stack = []
    ans = []
    curr = root
    while curr or stack:
        if curr:
            stack.append(curr)
            curr = curr.left
        else:
            curr = stack.pop()
            ans.append(curr.val)
            curr = curr.right

    return ans
