from typing import List


def combinationSum(candidates: List[int], target: int) -> List[List[int]]:
    ans = []

    def dfs(candidates, t, l):
        for c in candidates:
            # if len(l) and c < max(l):
            #     continue
            if c == t:
                ans.append(l+[c])
            elif c > t:
                continue
            else:
                dfs(candidates, t-c, l+[c])

    dfs(candidates, target, [])
    return ans


print(combinationSum([2, 3, 6, 7], 7))
