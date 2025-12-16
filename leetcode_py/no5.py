"""
给你一个字符串 s，找到 s 中最长的回文子串。
示例 1：

输入：s = "babad"
输出："bab"
解释："aba" 同样是符合题意的答案。
示例 2：

输入：s = "cbbd"
输出："bb"
"""


def longestPalindrome(s: str) -> str:
    n = len(s)
    if n == 1:
        return s

    dp = [[False] * n for _ in range(n)]
    for i in range(n):
        dp[i][i] = True

    max_l = 1
    start = 0

    for length in range(2, n+1):
        for i in range(n):
            j = length + i - 1
            if j >= n:
                break
            if s[i] == s[j]:
                if length == 2:
                    dp[i][j] = True
                else:
                    dp[i][j] = dp[i+1][j-1]

            if dp[i][j] and length > max_l:
                max_l = length
                start = i

    return s[start: start+max_l]


print(longestPalindrome('babad'))
