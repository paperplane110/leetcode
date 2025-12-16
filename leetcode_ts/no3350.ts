function maxIncreasingSubarrays(nums: number[]): number {
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
        ans = Math.max(ans, Math.min(cnt, preCnt));
        ans = Math.max(ans, Math.floor(cnt / 2));
    }
    return ans;
}