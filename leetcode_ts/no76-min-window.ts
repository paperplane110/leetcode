// function minWindow(s: string, t: string): string {
//   // 只需要 for，就能得到 t 中字母对应的 idx 列表
//   const tChar2cnt: Record<string, number> = {};
//   const sChar2Cnt: Record<string, number> = {}

//   for (const char of t) {
//     tChar2cnt[char] = (tChar2cnt[char] || 0) + 1;
//   }

//   const idxs: number[] = [];
//   for (let i = 0; i < s.length; i++) {
//     const char = s[i];
//     if (tChar2cnt[char] !== undefined) {
//       idxs.push(i)
//     }
//   }
//   console.log(idxs);

//   if (idxs.length < t.length) {
//     return "";
//   }

//   let left = 0, right = 0;
//   let minLen = s.length;
//   let minSlice = ""

//   while (right < idxs.length) {
//     // 录入当前 right 的 char
//     const rightChar = s[idxs[right]]
//     sChar2Cnt[rightChar] = (sChar2Cnt[rightChar] ?? 0) + 1

//     // 满足要求吗
//     while (check(sChar2Cnt, tChar2cnt)) {
//       // 满足
//       const rightIdx = idxs[right];
//       const leftIdx = idxs[left];
//       const currL = rightIdx - leftIdx + 1;
//       if (currL <= minLen) {
//         // 当前字符串长度比较小，更新 min 信息
//         minLen = currL;
//         minSlice = s.slice(leftIdx, rightIdx + 1)
//       }
//       console.log(idxs[left], idxs[right], minSlice, sChar2Cnt)
//       // 将当前 left 字符计数减一后，移动 left
//       const leftChar = s[idxs[left]];
//       sChar2Cnt[leftChar] -= 1;
//       left++;
//     }

//     // 不满足，移动 right
//     console.log(idxs[left], idxs[right], minSlice, sChar2Cnt)
//     right++
//   }
//   return minSlice;
// }

// function check(s: Record<string, number>, t: Record<string, number>): boolean {
//   for (let char of Object.keys(t)) {
//     if (s[char] === undefined || s[char] < t[char]) {
//       return false;
//     }
//   }
//   return true;
// }

function minWindow(s: string, t: string): string {
  const sCodeCnt = new Array(128).fill(0);
  const tCodeCnt = new Array(128).fill(0);
  for (const char of t) {
    tCodeCnt[char.charCodeAt(0)] += 1
  }

  let ansRight = s.length, ansLeft = -1;
  let l = 0
  for (let r = 0; r < s.length; r++) {
    sCodeCnt[s[r].charCodeAt(0)] += 1;

    while (check(sCodeCnt, tCodeCnt)) {
      if (r - l < ansRight - ansLeft) {
        ansLeft = l;
        ansRight = r;
      }
      sCodeCnt[s[l].charCodeAt(0)] -= 1;
      l++;
    }
  }

  return ansLeft < 0 ? "" : s.substring(ansLeft, ansRight + 1);
}

function check(sCnt: number[], tCnt: number[]) {
  if (sCnt.length !== tCnt.length) {
    return false;
  }
  for (let i = 0; i < sCnt.length; i++) {
    if (sCnt[i] < tCnt[i]) {
      return false;
    }
  }
  return true;
}

(function main() {
  //         0123456789012
  const s = "ADOBECODEBANC";
  const t = "ABC";
  console.log(`result: ${minWindow(s, t)}`)
})()