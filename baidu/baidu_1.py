"""
'aaabbxxyz' 压缩成‘3a2b2xyz'
"""


def compress_string(s: str):
    count = 0
    char = s[0]
    i = 0
    ans = ''

    while i < len(s):
        if s[i] == char:
            count += 1
            i += 1
        else:
            if count == 1:
                ans = ans + char
            else:
                ans = ans + str(count) + char
            char = s[i]
            count = 1
            i += 1

    if count == 1:
        ans = ans + char
    else:
        ans = ans + str(count) + char
    return ans


print(compress_string('aaabbxxyz'))
