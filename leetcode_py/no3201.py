"""
给你一个整数数组 nums。

nums 的子序列 sub 的长度为 x ，如果其满足以下条件，则称其为 有效子序列：

(sub[0] + sub[1]) % 2 == (sub[1] + sub[2]) % 2 == ... == (sub[x - 2] + sub[x - 1]) % 2
返回 nums 的 最长的有效子序列 的长度。

一个 子序列 指的是从原数组中删除一些元素（也可以不删除任何元素），剩余元素保持原来顺序组成的新数组。
"""

import numbers


def maximum_length(nums: list[int]) -> int:
    even_l, odd_l = 0, 0
    alter_l = 1
    pre_num = nums[0]
    for num in nums:
        if num % 2 == 0:
            even_l += 1
        else:
            odd_l += 1
        if (num - pre_num) % 2 == 1:
            alter_l += 1
            pre_num = num
    return max(even_l, odd_l, alter_l)