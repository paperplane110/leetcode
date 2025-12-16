function longestConsecutiveSorted(nums: number[]): number {
    if (nums.length === 0) {
        return 0;
    }

    nums.sort((a, b) => a - b)

    let maxLength = 1;
    let l = 1;
    let prev = nums[0];
    for (let i = 1; i < nums.length; i++) {
        const n = nums[i]
        // console.log(prev, n, l)
        if (n === prev) {
            continue;
        } else if (n === prev + 1) {
            l += 1;
            prev = n;
        } else {
            maxLength = Math.max(maxLength, l);
            l = 1;
            prev = n;
        }
    }
    maxLength = Math.max(maxLength, l);
    return maxLength;
}

function longestConsecutive(nums: number[]): number {
    const numSet = new Set(nums);
    let maxLength = 0;

    for (const num of numSet) {
        if (!numSet.has(num - 1)) {
            let currentNum = num;
            let currentLength = 1;
            while (numSet.has(currentNum + 1)) {
                currentLength += 1;
                currentNum += 1;
            }
            maxLength = Math.max(maxLength, currentLength);
        }
    }
    return maxLength
}

// 脚本主入口点 (使用 IIFE 结构)
(function main() {
    const testCases: number[][] = [
        [0, 3, 7, 2, 5, 8, 4, 6, 0, 1], // 预期结果: 9 ([0..8])
        [100, 4, 200, 1, 3, 2],         // 预期结果: 4 ([1, 2, 3, 4])
        []                             // 预期结果: 0
    ];

    console.log("--- 规范化 O(n log n) 方案 ---");
    testCases.forEach((nums, index) => {
        const result = longestConsecutiveSorted(nums);
        console.log(`Case ${index + 1}: [${nums.join(', ')}] -> Length: ${result}`);
    })
})()