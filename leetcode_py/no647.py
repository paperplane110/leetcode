"""
回文字串数目
"""


def countSubstrings(s: str) -> int:
    n = len(s)
    dp = [[False]*n for _ in range(n)]

    ans = 0
    for i in range(n):
        dp[i][i] = True

    for j in range(n):
        for i in range(0, j+1):
            if i == j:
                ans += 1
            else:
                if s[i] == s[j]:
                    if j-i < 2:
                        dp[i][j] = True
                        ans += 1
                    elif dp[i+1][j-1]:
                        dp[i][j] = True
                        ans += 1
    return ans
