function maxAreaNestedForLoop(height: number[]): number {
    let area = 0;
    for (let i = 0; i < height.length - 1; i++) {
        for (let j = i + 1; j < height.length; j++) {
            const h = Math.min(height[i], height[j]);
            const w = j - i;
            const s = w * h;
            area = Math.max(area, s)
        }
    }
    return area;
}

function maxArea(height: number[]): number {
    let left = 0;
    let right = height.length - 1;
    let area = 0;
    while (left < right) {
        const w = right - left;
        const h = Math.min(height[left], height[right]);
        area = Math.max(area, w * h);

        if (height[left] < height[right]) {
            left += 1;
        } else {
            right -= 1;
        }
    }
    return area;
}

(function main() {
    const testCase = [1,8,6,2,5,4,8,3,7];
    const area = maxArea(testCase);
    console.log(area)
})()