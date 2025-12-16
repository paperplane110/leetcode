
def max_sum(nums: list[int]) -> int:
    s = set()
    have_element = False
    ans = 0
    for n in nums:
        if n not in s:
            if n > 0:
                s.add(n)
                have_element = True
                ans += n
    if not have_element:
        ans = max(nums)
    return ans