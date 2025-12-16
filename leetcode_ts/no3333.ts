function possibleStringCount(word: string, k: number): number {
    const n = word.length;
    if (n < k) {
        return 0;
    }

    const MOD = 1_000_000_007;
    let freq = new Array<number>();
    let cnt = 0;
    for (let i = 0; i < n; i++) {
        if (i === 0) {
            cnt += 1;
        } else if (i === n - 1) {
            cnt += 1;
            freq.push(cnt);
        } else if (word[i] !== word[i - 1]) {
            freq.push(cnt)
            cnt = 1;
        } else {
            cnt += 1;
        }
    }

    // console.log(freq);

    const m = freq.length;
    let total = freq.reduce((a, b) => a * b)
    if (m >= k) {
        return total % MOD;
    }

    let f = Array.from({length: m + 1}, () => Array(k).fill(0))
    f[0][0] = 1;
    for (let i = 1; i <= m; i++) {
        let x = freq[i - 1];
        const s = [0];
        for (let idx = 0; idx < f[i - 1].length; idx++) {
            s.push(s[s.length - 1] + f[i - 1][idx]);
        }
        for (let j = 0; j < k; j++) {
            f[i][j] = (s[j + 1] - s[j - Math.min(x, j)] + MOD) % MOD;
        }
    } 
    console.log(f)
    return (total - f[m].reduce((sum, val) => sum + val, 0)) % MOD;
};

console.log(possibleStringCount('abbcccaa', 7))