function productExceptSelf(nums: number[]): number[] {
    const ans: number[] = [1];
    let tmp = 1;
    const n = nums.length;
    for (let i = 1; i < n; i++) {
        ans[i] = ans[i-1] * nums[i-1];
    }
    for (let i = n-2; i >= 0; i--) {
        tmp = tmp * nums[i + 1];
        ans[i] = ans[i] * tmp
    }
    return ans;
}

(function main() {
    // const test = [1,2,3,4];
    const test = [-1,1,0,-3,3];
    console.log(productExceptSelf(test));
})()

/**
 * 1 2 3 4
 * 1 1 3 4
 * 1 2 1 4
 * 1 2 3 1
 */