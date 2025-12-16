"""
给你一个整数 eventTime 表示一个活动的总时长，这个活动开始于 t = 0 ，结束于 t = eventTime 。

同时给你两个长度为 n 的整数数组 startTime 和 endTime 。它们表示这次活动中 n 个时间 没有重叠 的会议，其中第 i 个会议的时间为 [startTime[i], endTime[i]] 。

你可以重新安排 至多 k 个会议，安排的规则是将会议时间平移，且保持原来的 会议时长 ，你的目的是移动会议后 最大化 相邻两个会议之间的 最长 连续空余时间。

移动前后所有会议之间的 相对 顺序需要保持不变，而且会议时间也需要保持互不重叠。

请你返回重新安排会议以后，可以得到的 最大 空余时间。

注意，会议 不能 安排到整个活动的时间以外。

思考：
.___|----|___|----|__________|---|.
其实，我们可以先求出所有的间隔长度，存入一个数组。
当挪动次数 k=1 时，意味着我们可以合并两个间隔，即滑动窗口宽度为 w=k+1

"""
from typing import List

# Timeout
# 问题在于第二次遍历 for + sum 时间开销较大
# 其实可以通过一次遍历就求出所有符合滑动窗口宽度的和
# def max_free_time(eventTime: int, k: int, startTime: List[int], endTime: List[int]) -> int:
#     free_list = []
#     last_end_time = 0
#     for i in range(len(startTime)):
#         free_list.append(startTime[i] - last_end_time)
#         last_end_time = endTime[i]
#     free_list.append(eventTime - last_end_time)

#     w = k + 1
#     max_free = 0
#     for i in range(len(free_list) - w + 1):
#         max_free = max(max_free, sum(free_list[i:i+w]))
#     return max_free



def max_free_time(eventTime: int, k: int, startTime: List[int], endTime: List[int]) -> int:
    free_list = []
    last_end_time = 0
    for i in range(len(startTime)):
        free_list.append(startTime[i] - last_end_time)
        last_end_time = endTime[i]
    free_list.append(eventTime - last_end_time)

    w = k + 1
    w_list = []
    for i in range(len(free_list) - w + 1):
        if i == 0:
            w_list.append(sum(free_list[i:i+w]))
        else:
            w_list.append(w_list[-1] - free_list[i-1] + free_list[i+w-1])

    return max(w_list)


max_free_time(24, 2, [11, 20, 15], [20, 23, 20])