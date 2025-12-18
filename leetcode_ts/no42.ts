function trap(height: number[]): number {
    // Method 1 O(nlog_n)
    // 找最大，分两边，分别进行递归
    // 左边递归：找最大，计算最大值右侧水量，对左侧进行递归；终止条件: endIndex <= 0
    // 右边递归：找最大，计算最大值左侧水量，对右侧进行递归；终止条件: startIndex >= height.length - 1

    // Method 2 O(3n) = O(n)
    // 从左往右，不断找 max，每当 max 更新时，记录 idx，存放在 l2r_max 数组中
    // 计算每个 idx 间能容纳的雨水
    // 从右往左，到l2r_max[-1]处结束，不断找 max，每当 max 更新时，记录 idx
    // 到l2r_max[-1]处结束，是因为，我们已经找到了l2r_max[-1]左侧所有的最大值了，其实只需要计算其右侧还有没有水坑
    // 计算每个 idx 间能容纳的雨水
    let rain = 0
    let max = height[0]
    const l2r_max: number[] = [0];
    for (let i = 1; i < height.length; i++) {
        if (height[i] >= max) {
            max = height[i];
            l2r_max.push(i)
        }
    }
    for (let i = 0; i < l2r_max.length - 1; i++) {
        const localStart = l2r_max[i]
        const localEnd = l2r_max[i + 1]
        const localHeight = height[l2r_max[i]];
        for (let j = localStart; j < localEnd; j++) {
            rain += localHeight - height[j];
        }
    }

    const lastIdx = height.length - 1;
    max = height[lastIdx]
    const r2l_max: number[] = [lastIdx]
    for (let i = lastIdx - 1; i >= l2r_max[l2r_max.length - 1]; i--) {
        if (height[i] >= max) {
            max = height[i];
            r2l_max.push(i)
        }
    }
    for (let i = 0; i < r2l_max.length - 1; i++) {
        const localStart = r2l_max[i]
        const localEnd = r2l_max[i + 1]
        const localHeight = height[r2l_max[i]];
        for (let j = localStart; j > localEnd; j--) {
            rain += localHeight - height[j];
        }
    }

    return rain;
};

function trapTwoPointer(height: number[]): number {
    let ans = 0, preMax = 0, sufMax = 0;
    let left = 0, right = height.length - 1;
    while (left < right) {
        preMax = Math.max(preMax, height[left]);
        sufMax = Math.max(sufMax, height[right]);
        if (preMax < sufMax) {
            const step = height[left++]
            ans += preMax - step;
            console.log(ans, preMax, step, left)
        } else {
            ans += sufMax - height[right--];
        }
        console.log(ans)
    }
    return ans
}


(function main() {
    const test1 = [1, 3, 1, 3, 1, 4]
    const rain1 = trap(test1)
    console.log(rain1)

    const test2 = [1, 1]
    const rain2 = trap(test2)
    console.log(rain2)

    const test3 = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    const rain3 = trapTwoPointer(test3)
    console.log(rain3)
})()

