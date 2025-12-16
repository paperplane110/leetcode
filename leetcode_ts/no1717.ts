function maximumGain(s: string, x: number, y: number): number {
    let a = 'a';
    let b = 'b';
    if (x > y) {
        [x, y] = [y, x];
        [a, b] = [b, a];
    }
    let cnt_b = 0;
    let cnt_a = 0;
    let ans = 0;
    for (const c of s) {
        if (c === b) {
            cnt_b++;
        } else if (c === a) {
            if (cnt_b > 0) {
                cnt_b--;
                ans += y;
            } else {
                cnt_a++;
            }
        } else {
            ans += Math.min(cnt_a, cnt_b) * x;
            cnt_a = 0;
            cnt_b = 0;
        }
    }
    ans += Math.min(cnt_a, cnt_b) * x;
    return ans;
}