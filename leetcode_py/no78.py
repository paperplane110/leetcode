"""
给你一个整数数组 nums ，数组中的元素 互不相同 。返回该数组所有可能的子集（幂集）。

解集 不能 包含重复的子集。你可以按 任意顺序 返回解集。

 

示例 1：

输入：nums = [1,2,3]
输出：[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
示例 2：

输入：nums = [0]
输出：[[],[0]]
"""

from typing import List


def subsets(nums: List[int]) -> List[List[int]]:
    ans = [[]]

    def dfs(curr: list, remain: list):
        if not remain:
            return 0
        for idx, num in enumerate(remain):
            tmp = []
            tmp[:] = curr
            tmp.append(num)
            ans.append(tmp)
            dfs(tmp, remain[idx+1::])

    dfs([], nums)
    return ans


print(subsets([1, 2, 3]))
