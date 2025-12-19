function lengthOfLongestSubstring(s: string): number {
    if (s.length === 0) {
        return 0;
    }

    let start = 0, end = 1;
    const letterToIndex: Map<string, number> = new Map();
    letterToIndex.set(s[start], start);
    let max = 0, temp = 1

    while (end < s.length) {
        const letter = s[end]
        
        const index = letterToIndex.get(letter)
        if (index === undefined || index < start) {
            // 如果没有这个字母：temp += 1
            temp += 1;
        } else {
            // 如果已经有了这个字母：更新 max，start 跳到字母 index +1 的位置（保证不包含这个字母), 更新 temp
            max = Math.max(max, temp);
            start = index + 1;
            temp = end - start + 1;
        }
        
        // 记录字母出现的 index
        letterToIndex.set(letter, end)
        end += 1;
    }
    max = Math.max(max, temp)
    return max;
}