function maxSlidingWindowBrutalForce(nums: number[], k: number): number[] {
    const ans = new Array<number>();
    for (let i = 0; i <= nums.length - k; i++) {
        const slice = nums.slice(i, i+k);
        ans.push(Math.max(...slice))
    }
    return ans
}

function maxSlidingWindow(nums: number[], k: number): number[] {
    const ans: number[] = [];
    const deque: number[] = [];
    for (let i = 0; i < nums.length; i++) {
        while (deque.length > 0 && nums[i] > nums[deque[deque.length - 1]]) {
            deque.pop();
        }
        deque.push(i);
        if (deque[0] < i - k + 1) {
            deque.shift();
        }
        if (i >= k - 1) {
            ans.push(nums[deque[0]]);
        }
    }
    return ans;
}