function findAnagrams(s: String, p: String): number[] {
    const ans: number[] = [];
    const w = p.length;

    const cntP = new Array(26).fill(0);
    const aCharCode = 'a'.charCodeAt(0);
    for (const c of p) {
        cntP[c.charCodeAt(0) - aCharCode] += 1;
    }

    const cntS = new Array(26).fill(0);
    for (let i = 0; i <= s.length - w; i++) {
        cntS[s.charCodeAt(i) - aCharCode] += 1;

        const left = i - w + 1;
        if (left < 0) {
            continue;
        }

        if (isArrayEqual(cntP, cntS)) {
            ans.push(left)
        }

        cntS[s.charCodeAt(left) - aCharCode] -= 1;
    }
    return ans;
}

function isArrayEqual<T>(a: T[], b: T[]): boolean {
    for (let i = 0; i < a.length; i++) {
        if (a[i] !== b[i]) {
            return false;
        }
    }
    return true;
}