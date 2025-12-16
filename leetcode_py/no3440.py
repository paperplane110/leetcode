"""给你一个整数 eventTime 表示一个活动的总时长，这个活动开始于 t = 0 ，结束于 t = eventTime 。

同时给你两个长度为 n 的整数数组 startTime 和 endTime 。它们表示这次活动中 n 个时间 没有重叠 的会议，其中第 i 个会议的时间为 [startTime[i], endTime[i]] 。

你可以重新安排 至多 一个会议，安排的规则是将会议时间平移，且保持原来的 会议时长 ，你的目的是移动会议后 最大化 相邻两个会议之间的 最长 连续空余时间。

请你返回重新安排会议以后，可以得到的 最大 空余时间。

注意，会议 不能 安排到整个活动的时间以外，且会议之间需要保持互不重叠。

注意：重新安排会议以后，会议之间的顺序可以发生改变。
"""
from typing import List

# Timeout
# 时间复杂度 O(n^2)，空间复杂度 O(n)
# def max_free_time(eventTime: int, startTime: List[int], endTime: List[int]) -> int:

#     def is_insertable(m_idx: int, free_list: list[int]) -> bool:
#         for i, f in enumerate(free_list):
#             if i == m_idx or i == m_idx + 1:
#                 continue
#             if free_list[i] >= endTime[m_idx] - startTime[m_idx]:
#                 return True
#         return False

#     free_list = []
#     free_list.append(startTime[0])
#     for i in range(1, len(startTime)):
#         free_list.append(startTime[i] - endTime[i-1])
#     free_list.append(eventTime - endTime[-1])

#     f_max = 0
#     for i in range(len(startTime)):
#         f = free_list[i] + free_list[i+1]
#         if is_insertable(i, free_list):
#             f += endTime[i] - startTime[i]
#         f_max = max(f_max, f)
#     return f_max


def max_free_time(eventTime: int, startTime: List[int], endTime: List[int]) -> int:
    n = len(startTime)

    def get_free_before_meetting(idx: int) -> int:
        if idx == 0:
            return startTime[0]
        elif idx == n:
            return eventTime - endTime[-1]
        else:
            return startTime[idx] - endTime[idx-1]
    
    a, b, c = 0, 0, 0
    for i in range(n+1):
        f = get_free_before_meetting(i)
        if f > a:
            a, b, c = i, a, b
        elif f > b:
            b, c = i, b
        elif f > c:
            c = i
    
    f_max = 0
    for i, (e, s) in enumerate(zip(endTime, startTime)):
        meeting_len = e - s
        if a != i and a != i+1 and meeting_len <= get_free_before_meetting(a) or \
            b != i and b != i+1 and meeting_len <= get_free_before_meetting(b) or \
            meeting_len <= get_free_before_meetting(c):
            f_max = max(
                f_max,
                get_free_before_meetting(i) + meeting_len + get_free_before_meetting(i+1)
            )
        else:
            f_max = max(
                f_max,
                get_free_before_meetting(i) + get_free_before_meetting(i+1)
            )
    return f_max
        


print(max_free_time(24, [11, 20, 15], [20, 23, 20]))
print(max_free_time(10, [0, 3, 7, 9], [1, 4, 8, 10]))
