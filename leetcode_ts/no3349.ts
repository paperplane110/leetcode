function hasIncreasingSubarrays(
    nums: number[],
    k: number
): boolean {
    let cnt = 1;
    let preCnt = 0;
    let ans = 0;
    for (let i = 1; i < nums.length; i++) {
        if (nums[i] > nums[i-1]) {
            cnt++;
        } else {
            preCnt = cnt;
            cnt = 1;
        }
        ans = Math.max(ans, Math.min(preCnt, cnt));
        ans = Math.max(ans, Math.floor(cnt / 2));
    }
    return ans >= k;
}

console.log(
    hasIncreasingSubarrays(
        [2,5,7,8,9,2,3,4,3,1],
        3
    )
)