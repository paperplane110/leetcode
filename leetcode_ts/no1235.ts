/* 
你打算利用空闲时间来做兼职工作赚些零花钱。

这里有 n 份兼职工作，每份工作预计从 startTime[i] 开始到 endTime[i] 结束，报酬为 profit[i]。

给你一份兼职工作表，包含开始时间 startTime，结束时间 endTime 和预计报酬 profit 三个数组，请你计算并返回可以获得的最大报酬。

注意，时间上出现重叠的 2 份工作不能同时进行。

如果你选择的工作在时间 X 结束，那么你可以立刻进行在时间 X 开始的下一份工作。
*/

function jobScheduling(startTime: number[], endTime: number[], profit: number[]): number {
    const jobs = startTime
        .map((st, idx) => [st, endTime[idx], profit[idx]])
        .sort((a, b) => a[1] - b[1])

    const f = [0]
    for (let idx = 0; idx < jobs.length; idx++) {
        const [st, et, p] = jobs[idx];
        let lo = 0, hi = idx;

        // 二分查找，找到大于 st 的最小 job 的下标
        while (lo < hi) {
            const mid = (lo + hi) >> 1;
            if (jobs[mid][1] <= st) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        f[idx + 1] = Math.max(f[idx], f[lo] + p);
    }
    return f[jobs.length];
}

// write test for me
console.log(jobScheduling([1,2,3,3], [3,4,5,6], [50,10,40,70]))
