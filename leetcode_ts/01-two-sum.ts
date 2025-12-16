function twoSum(nums: number[], target: number): number[] {
    const hashTable: Record<number, number> = {}
    for (let i = 0; i < nums.length; i++) {
        // 判断一个 hashTable 中是否存在 target - nums[i]
        if (hashTable[target - nums[i]] !== undefined) {
            return [hashTable[target - nums[i]], i]
        }
        hashTable[nums[i]] = i
    }
    return []
};

try {
    console.log(twoSum([2,7,11,15], 9))
} catch (error) {
    console.log(error)
}