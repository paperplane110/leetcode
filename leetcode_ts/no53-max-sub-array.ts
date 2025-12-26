// function maxSubArray(nums: number[]): number {
//     if (nums.length === 1) {
//         return nums[0];
//     }

//     let max = nums[0], maxIdx = 0;
//     let min = Math.min(nums[0], 0), minIdx = -1;
//     for (let i = 1; i < nums.length; i++) {
//         const sum = nums[i-1] + nums[i]
//         if (sum >= max) {
//             max = sum;
//             maxIdx = i;
//         }
//         if (sum < min) {
//             min = sum;
//             minIdx = i;
//         }
//         nums[i] = sum;
//     }
//     console.log(nums)
//     if (minIdx < maxIdx) {
//         return max - min;
//     } else {
//         let leftMax;
//         if (maxIdx === 0) {
//             leftMax = max
//         } else {
//             leftMax = max - Math.min(...nums.slice(0, maxIdx));
//         }

//         let rightMax;
//         if (minIdx === nums.length - 1) {
//             return max
//         } else {
//             rightMax = Math.max(...nums.slice(minIdx, nums.length)) - min;
//         }
//         return Math.max(leftMax, rightMax);
//     }
// }

function maxSubArray(nums: number[]): number {
    let last_sub_arr_max = 0;
    let ans = -Infinity;
    for (const n of nums) {
        last_sub_arr_max = Math.max(last_sub_arr_max + n, n);
        ans = Math.max(ans, last_sub_arr_max)
    }
    return ans
}

(function main() {
    const testSet = [[-2, 1, -3, 4, -1, 2, 1, -5, 4], [1, 1], [-2, 1], [-2, -1]];
    testSet.map((testcase) => {
        console.log(maxSubArray(testcase))
    })
})()

/**
 * 逐一累加，得到：-2, -1, -4, 0, -1, 1, 2, -3, 1
 *                       l            r
 * 寻找一个 right - left 的最大差值，2 - -4 = 6
 * 
 * 
 * 其他情况，最大值idx 小于 最小值idx：-2, -1, 2, 0, -1, 1, -4, -3, 1
 *                                        max          min
 * 最大值 2，然后往左找最小值 -2，差值 4
 * 最小值 -4，然后往右找最大值 1，差值 5
 * 所以最大为 5
 * 
 * [-2,-1,-4]
 * -2, -3, -7
 * max     min
 * 
 * [1,-2,-3,4,-1,2,1,-5,4]
 * [1] 1
 * [1, -2]     max(-2 + 1, -2) = -1, max(1, -1) = 1
 * [1, -2, -3] max(-1 -3, -3) = -3, max(1, -3) = 1
 * 
 * 所以每一步做两次 max，
 * 第一个max是 （以i-1结尾的最大连续字数组合 + nums[i], nums[i]）
 * 第二个max是全局的 max 比较
 * 
 */
