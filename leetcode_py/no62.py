"""
一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为 “Start” ）。

机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为 “Finish” ）。

问总共有多少条不同的路径？

输入：m = 3, n = 2
输出：3

-> x
[0][ ][ ]
[ ][ ][x]
"""


# def uniquePaths(m: int, n: int) -> int:
#     x, y = 1, 1
#     ans = [0]

#     def dfs(x, y, m, n):
#         if x == m and y == n:
#             ans[0] += 1
#             return 0
#         if x < m:
#             dfs(x+1, y, m, n)
#         if y < n:
#             dfs(x, y+1, m, n)
#     dfs(x, y, m, n)
#     return ans[0]


"""
def f(m, n):
    if m == 1 or n == 1:
        return 1
    else:
        return f(m, n-1)+f(m-1, n)
    return f(m, n)
"""


def uniquePaths(m: int, n: int) -> int:
    dp = [[1]*n] + [[1]+[0]*(n-1) for _ in range(m-1)]
    for i in range(1, m):
        for j in range(1, n):
            dp[i][j] = dp[i-1][j] + dp[i][j-1]
    return dp[m-1][n-1]


print(uniquePaths(23, 12))
