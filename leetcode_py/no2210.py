def count_hill_valley(nums: list[int]) -> int:
    n = len(nums)
    left = nums[0]
    ans = 0
    for i in range(1, n-1):
        curr = nums[i]
        right = nums[i + 1]
        print(f"({left}, {curr}, {right})")
        if curr == right:
            continue
        elif left > curr and right > curr:
            print(curr)
            ans += 1
        elif left < curr and right < curr:
            print(curr)
            ans += 1
        left = curr
    return ans

count_hill_valley([8,2,5,7,7,2,10,3,6,2])
