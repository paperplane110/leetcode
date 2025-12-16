"""
给你一个下标从 0 开始的整数数组 nums ，它包含 3 * n 个元素。

你可以从 nums 中删除 恰好 n 个元素，剩下的 2 * n 个元素将会被分成两个 相同大小 的部分。

前面 n 个元素属于第一部分，它们的和记为 sumfirst 。
后面 n 个元素属于第二部分，它们的和记为 sumsecond 。
两部分和的 差值 记为 sumfirst - sumsecond 。

比方说，sumfirst = 3 且 sumsecond = 2 ，它们的差值为 1 。
再比方，sumfirst = 2 且 sumsecond = 3 ，它们的差值为 -1 。
请你返回删除 n 个元素之后，剩下两部分和的 差值的最小值 是多少。
"""

from heapq import heapify, heappushpop
from math import inf


def minimum_difference(nums: list[int]) -> int:
    l = len(nums)
    n = l // 3

    max_of_suffix = [0] * (l - n + 1)
    suffix_heap = nums[-n:]
    heapify(suffix_heap)
    max_of_suffix[-1] = sum(suffix_heap)

    # a, a, b, b, c, c
    # 0, 1, 2, 3, 4, 5
    # 0, 1,|2, 3,|4, 5
    #0  1  2  3  4
    # l = 6, n = 2
    # i = [3, 1) = 3, 2
    for i in range(l - n - 1, n - 1, -1):
        max_of_suffix[i] = max_of_suffix[i + 1] + nums[i] - heappushpop(suffix_heap, nums[i])
    
    prefix_heap = [-x for x in nums[:n]]
    heapify(prefix_heap)
    min_of_prefix = -sum(prefix_heap)
    ans = min_of_prefix - max_of_suffix[n]
    for i in range(n, l - n):
        min_of_prefix += nums[i] + heappushpop(prefix_heap, -nums[i])
        ans = min(ans, min_of_prefix - max_of_suffix[i + 1])
    return ans

minimum_difference([3,1,2])