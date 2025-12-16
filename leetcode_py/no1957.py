"""
一个字符串如果没有 三个连续 相同字符，那么它就是一个 好字符串 。

给你一个字符串 s ，请你从 s 删除 最少 的字符，使它变成一个 好字符串 。

请你返回删除后的字符串。题目数据保证答案总是 唯一的 。
"""

def make_fancy_string(s: str) -> str:
    cnt = 1
    curr = s[0]
    res = ""
    for i in range(1, len(s)):
        if s[i] == curr:
            cnt += 1
            if cnt >= 3:
                continue
            else:
                res += curr
        else:
            curr = s[i]
            cnt = 1
            res += curr
    return res
        