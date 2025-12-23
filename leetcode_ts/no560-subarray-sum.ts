function subarraySum(nums: number[], k: number): number {
    let ans = 0, pre = 0;
    const sum2cnt: Record<number, number> = {};
    // 
    sum2cnt[0] = 1;
    for (let i = 0; i < nums.length; i++) {
        pre += nums[i];
        if (sum2cnt[pre - k] !== undefined) {
            ans += sum2cnt[pre - k];
        }
        sum2cnt[pre] = sum2cnt[pre] === undefined ? 1 : sum2cnt[pre] + 1;
    }
    return ans;
}