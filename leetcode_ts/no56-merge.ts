function merge(intervals: number[][]): number[][] {
    intervals.sort((a, b) => a[0] - b[0]);
    const ans = []
    let prev: number[] = [];
    for (let i = 0; i < intervals.length; i++) {
        if (i === 0) {
            prev = intervals[0]
        } else {
            const curr = intervals[i]
            if (curr[0] <= prev[1]) {
                if (curr[1] >= prev[1]) {
                    // p0 c0 p1 c1, merge
                    prev = [prev[0], curr[1]]
                } else {
                    // p0 c0 c1 p1, keep
                    continue;
                }
            } else {
                ans.push(prev);
                prev = curr;
            }
        }
    }
    ans.push(prev);
    return ans;
}