from math import inf


def minimun_score(nums: list[int], edges: list[list[int]]) -> int:
    n = len(nums)
    adj = [[] for _ in range(n)]
    for u, v in edges:
        adj[u].append(v)
        adj[v].append(u)
    
    xor, in_, out = [0] * n, [0] * n, [0] * n
    clock = 0
    def dfs(x: int, fa: int):
        nonlocal clock
        clock += 1
        in_[x] = clock
        xor[x] = nums[x]
        for y in adj[x]:
            if y != fa:
                dfs(y, x)
                xor[x] ^= xor[y]
        out[x] = clock
    dfs(0, -1)

    def is_ancestor(x: int, y: int) -> bool:
        return in_[x] <= in_[y] <= out[x]

    ans = inf
    for x in range(2, n):
        for y in range(1, x):
            if is_ancestor(x, y):
                a, b, c = xor[y], xor[x] ^ xor[y], xor[0] ^ xor[x]
            elif is_ancestor(y, x):
                a, b, c = xor[x], xor[y] ^ xor[x], xor[0] ^ xor[y]
            else:
                a, b, c = xor[x], xor[y], xor[0] ^ xor[x] ^ xor[y]
            ans = min(ans, max(a, b, c) - min(a, b, c)) 
            if ans == 0:
                return 0
    return ans