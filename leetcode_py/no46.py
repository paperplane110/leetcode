"""
给定一个不含重复数字的数组 nums ，返回其 所有可能的全排列 。你可以 按任意顺序 返回答案。

输入：nums = [1,2,3]
输出：[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
"""

from typing import List


def permute(nums: List[int]) -> List[List[int]]:
    if len(nums) == 1:
        return [nums]

    ans = []

    def dfs(l):
        remain = len(nums) - len(l)
        if remain == 0:
            ans.append(l)
            return 0
        for i in nums:
            if i in l:
                continue
            else:
                dfs(l+[i])

    dfs([])
    return ans


print(permute([1, 2]))
