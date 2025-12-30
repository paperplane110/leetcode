// function rotate(nums: number[], k: number) {
//     const l = nums.length;
//     for (let i = 0; i < k; i++) {
//         for (let j = 0; j < k - 1; j++) {
//             nums[j], nums[l-1] = nums[l-1], nums[j];
//         }
//     }
// }

function rotate(nums: number[], k: number) {
    function reverse(i: number, j: number) {
        while (i < j) {
            [nums[i], nums[j]] = [nums[j], nums[i]];
            i++;
            j--;
        }
    }
    const n = nums.length;
    k = k % n;
    reverse(0, n - 1);
    reverse(0, k - 1);
    reverse(k, n - 1);
}