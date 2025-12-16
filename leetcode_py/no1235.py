"""
你打算利用空闲时间来做兼职工作赚些零花钱。

这里有 n 份兼职工作，每份工作预计从 startTime[i] 开始到 endTime[i] 结束，报酬为 profit[i]。

给你一份兼职工作表，包含开始时间 startTime，结束时间 endTime 和预计报酬 profit 三个数组，请你计算并返回可以获得的最大报酬。

注意，时间上出现重叠的 2 份工作不能同时进行。

如果你选择的工作在时间 X 结束，那么你可以立刻进行在时间 X 开始的下一份工作。
"""
# O(n^2)
# def job_scheduling(startTime: list[int], endTime: list[int], profit: list[int]) -> int:
#     # sort by end time
#     jobs = sorted(zip(startTime, endTime, profit), key=lambda x: x[1])

#     f = [0]
#     for idx in range(1, len(jobs) + 1):
#         print(f"==== idx: {idx} ====")
#         not_chose = f[idx - 1]
#         chose = 0
#         for j in range(idx - 1, -1, -1):
#             print(f"j: {j}, endTime: {jobs[j][1]}, startTime: {jobs[idx - 1][0]}")
#             if jobs[j][1] <= jobs[idx - 1][0]:
#                 chose = f[j + 1]
#                 print(f"update chose: {chose}")
#                 break
#         chose += jobs[idx - 1][2]
#         print(idx, not_chose, chose)
#         f.append(max(not_chose, chose))
#     print(f)
#     return f[-1]


from bisect import bisect_left


def job_scheduling(startTime: list[int], endTime: list[int], profit: list[int]) -> int:
    # sort by end time
    jobs = sorted(zip(startTime, endTime, profit), key=lambda x: x[1])

    f = [0]
    for idx, (st, et, p) in enumerate(jobs):
        # sorted list: jobs
        # find the first job whose end time is: e >= (st + 1)
        # -> find the first job whose end time is: e > st
        # range: [lo, hi)
        j = bisect_left(jobs, (st + 1), hi=idx, key=lambda x: x[1])

        # 上面我们找到了 j job 的结束时间是大于 st 前提下最靠前的 job
        # 所以 j - 1 的 job 结束时间就是小于等于 st 的
        # 但是，由于 f 列表中，0 号 job 实际对应的是 f[1]，即 f[0 + 1]，即 f[idx + 1]
        # 所以我们要取 f[j - 1 + 1]
        f.append(max(f[idx], f[j - 1 + 1] + p))
    return f[-1]


print(job_scheduling([1,2,3,3], [3,4,5,6], [50,10,40,70]))