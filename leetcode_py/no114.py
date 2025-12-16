from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def list2tree(nums: list):
    root = TreeNode(nums.pop(0))
    queue = [root]
    while queue and nums:
        node = queue.pop(0)
        if left := nums.pop(0):
            l_node = TreeNode(left)
            queue.append(l_node)
            node.left = l_node

        if right := nums.pop(0):
            r_node = TreeNode(right)
            queue.append(r_node)
            node.right = r_node
    return root


def flatten(root: Optional[TreeNode]) -> None:
    """
    Do not return anything, modify root in-place instead.
    """
    if not root or (not root.left and not root.right):
        return root

    curr = root
    while curr:
        if curr.left:
            curr.left, curr.right = curr.right, curr.left
            p = curr.right
            while p.right:
                p = p.right
            p.right = curr.left
            curr.left = None
            curr = curr.right
        elif curr.right:
            curr = curr.right
        else:
            break


root = list2tree([1, 2, 5, 3, 4, None, 6])
flatten(root)
