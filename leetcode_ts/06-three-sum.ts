function threeSum(nums: number[]): number[][] {
  const ans: number[][] = [];
  const sortedNum = nums.sort((a, b) => a - b);
  for (let i = 0; i < nums.length - 2; i++) {
    if (i > 0 && sortedNum[i] === sortedNum[i - 1]) {
      continue;
    }
    let right = nums.length - 1;
    const target = -sortedNum[i];
    for (let left = i + 1; left < sortedNum.length - 1; left++) {
      if (left > i + 1 && sortedNum[left] === sortedNum[left - 1]) {
        continue;
      }
      while (left < right && sortedNum[left] + sortedNum[right] > target) {
        right -= 1;
      }
      if (left === right) {
        break;
      }
      if (sortedNum[left] + sortedNum[right] === target) {
        ans.push([sortedNum[i], sortedNum[left], sortedNum[right]]);
      }
    }
  }
  return ans
}

(function main() {
  const testCase = [-1,0,1,2,-1,-4];
  const ans = threeSum(testCase);
  console.log(ans)
})()