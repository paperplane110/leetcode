"""
给你一个正整数 days，表示员工可工作的总天数（从第 1 天开始）。
另给你一个二维数组 meetings，长度为 n，其中 meetings[i] = [start_i, end_i] 表示第 i 次会议的开始和结束天数（包含首尾）。

返回员工可工作且没有安排会议的天数。

注意：会议时间可能会有重叠。
"""

def count_days(days: int, meetings: list[list[int]]) -> int:
    end = 1
    free = 0
    meetings = sorted(meetings, key=lambda x: x[0])
    for idx, (s, e) in enumerate(meetings):
        if idx == 0:
            free += s - 1
        elif s > end:
            free += s - end - 1
        end = max(end, e)
    return free + days - end

days = 10
meetings = [[5,7],[1,3],[9,10]]
count_days(days, meetings)