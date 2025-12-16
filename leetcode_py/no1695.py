"""
给你一个正整数数组 nums ，请你从中删除一个含有 若干不同元素 的子数组。删除子数组的 得分 就是子数组各元素之 和 。

返回 只删除一个 子数组可获得的 最大得分 。

如果数组 b 是数组 a 的一个连续子序列，即如果它等于 a[l],a[l+1],...,a[r] ，那么它就是 a 的一个子数组。
"""

# def maximum_unique_subarray(nums: list[int]) -> int:
#     tmp = []
#     mx = 0
#     for n in nums:
#         if n not in tmp:
#             tmp.append(n)
#         else:
#             mx = max(mx, sum(tmp))
#             tmp = tmp[tmp.index(n)+1:]
#             tmp.append(n)
#     mx = max(mx, sum(tmp))
#     return mx

def maximum_unique_subarray(nums: list[int]) -> int:
    left = 0
    sum = 0
    mx = 0
    tmp = set()
    for right in range(len(nums)):
        while nums[right] in tmp:
            sum -= nums[left]
            tmp.remove(nums[left])
            left += 1
        sum += nums[right]
        tmp.add(nums[right])
        mx = max(mx, sum)
    return mx