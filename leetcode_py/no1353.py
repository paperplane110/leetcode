# [ERROR] 这个答案是错误的，因为递归到一个无法安排的会议，会阻塞接下来的会议安排
# def max_events(events: list[list[int]]) -> int:
#     most = len(events)
#     current_most = 0
#     def _recursive(idx: int, scheduled: list[int]):
#         nonlocal current_most
#         print(idx, scheduled)
#         if idx == most:
#             current_most = most
#             return
#         for i in range(events[idx][0], events[idx][1] + 1):
#             if i not in scheduled:
#                 current_most = max(current_most, idx + 1)
#                 scheduled.append(i)
#                 _recursive(idx + 1, scheduled)
#                 scheduled.pop()
#                 if current_most == most:
#                     return
    
#     _recursive(0, [])
#     return current_most

import heapq


def max_events(events: list[list[int]]) -> int:
    # the latest last day
    mx = max(e[1] for e in events)

    # grouped by start date
    groups = [[] for _ in range(mx + 1)]
    for e in events:
        groups[e[0]].append(e[1])
    
    ans = 0
    h = []  # min heap, containing the meeting could be scheduled
    for i, g in enumerate(groups):
        while h and h[0] < i:
            # pop the out dated meeting
            heapq.heappop(h)
        for end in g:
            # add meeting to waiting list
            heapq.heappush(h, end)
        if h:
            # attend the meeting that ends earliest
            heapq.heappop(h)
            ans += 1
    
    return ans

# print(max_event([[1,2],[2,3],[3,4],[1,2]]))
print(max_events([[27,27],[8,10],[9,11],[20,21],[25,29],[17,20],[12,12],[12,12],[10,14],[7,7],[6,10],[7,7],[4,8],[30,31],[23,25],[4,6],[17,17],[13,14],[6,9],[13,14]]))