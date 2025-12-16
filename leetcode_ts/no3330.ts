function possibleStringCount(word: string): number {
    let r_cnt = 0;
    let f_cnt = 0;
    let res = 0;
    let current = word[0];
    for (let i = 1; i < word.length; i++) {
        if (word[i] !== current) {
            if (r_cnt === 1) {
                f_cnt += 1;
            }
            r_cnt += 1;
        } else {
            if (r_cnt > 1) {
                res += r_cnt;
            }
            // reset
            r_cnt = 0;
            current = word[i];
        }
    }
    if (r_cnt > 1) {
        res += r_cnt;
    }
    return (res + 1 - f_cnt);
};

// 测试上述代码
console.log("aaaa   :", possibleStringCount('aaaa'));
console.log("abbccc :", possibleStringCount('abbccc'));
console.log("abcd   :", possibleStringCount('abcd'));
