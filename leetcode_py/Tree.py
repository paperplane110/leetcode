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
