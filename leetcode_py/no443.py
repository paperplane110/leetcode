from typing import List


def compress(chars: List[str]) -> int:
    curr = chars[0]
    count = 0
    i = 0
    j = 0
    while i < len(chars):
        if chars[i] == curr:
            count += 1
            i += 1
        else:
            chars[j] = curr
            j += 1

            if count > 1:
                for num in str(count):
                    chars[j] = num
                    j += 1

            curr = chars[i]
            count = 1
            i += 1

    chars[j] = curr
    j += 1

    if count > 1:
        for num in str(count):
            chars[j] = num
            j += 1

    return j
