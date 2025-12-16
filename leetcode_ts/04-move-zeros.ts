function moveZeroes(nums: number[]): void {
    // [0,1,0,3,12]
    // [0,1] switch and move i, j
    // [1,1] move i, j
    // [0,0] move i
    // [1,0] move i, j
    let i = 0;
    let j = 0;
    while (i < nums.length) {
        if (nums[i] !== 0) {
            [nums[i], nums[j]] = [nums[j], nums[i]]
            j++;
        }
        i++;
    }
};

(function main() {
    const testCase = [0,1,0,3,12];
    moveZeroes(testCase);
    console.log(testCase);
})()