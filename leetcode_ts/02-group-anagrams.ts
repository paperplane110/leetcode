// function groupAnagrams(strs: string[]): string[][] {
//   const lettersToAnagrams: Record<string, string[]> = {};
//   for (const s of strs) {
//     const letters = Array.from(s).sort().join("");
//     if (lettersToAnagrams[letters] !== undefined) {
//       lettersToAnagrams[letters].push(s);
//     } else {
//       lettersToAnagrams[letters] = [s];
//     }
//   }
//   const ans: string[][] = [];
//   for (const [_key, anagram] of Object.entries(lettersToAnagrams)) {
//     ans.push(anagram)
//   }
//   return ans;
// };

function groupAnagrams(strs: string[]): string[][] {
  const map = new Map<string, string[]>();
  for (const s of strs) {
    const key = Array.from(s).sort().join('');
    const list = map.get(key) ?? [];
    list.push(s);
    map.set(key, list);
  }
  return Array.from(map.values())
};