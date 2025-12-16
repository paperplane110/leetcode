from math import inf

def max_distance(arrays) -> int:
    """
    :type arrays: List[List[int]]
    :rtype: int
    """
    max_1 = [arrays[0][-1], 0]
    max_2 = [-inf, None]
    min_1 = [arrays[0][0], 0]
    min_2 = [inf, None]
    for idx, l in enumerate(arrays[1:]):
        idx += 1
        l_max = l[-1]
        l_min = l[0]
        if l_max >= max_1[0]:
            max_2[0], max_2[1] = max_1[0], max_1[1]
            max_1[0], max_1[1] = l_max, idx
        elif l_max >= max_2[0]:
            max_2[0], max_2[1] = l_max, idx
        
        if l_min <= min_1[0]:
            min_2[0], min_2[1] = min_1[0], min_1[1]
            min_1[0], min_1[1] = l_min, idx
        elif l_min <= min_2[0]:
            min_2[0], min_2[1] = l_min, idx
            
    if max_1[1] != min_1[1]:
        return max_1[0] - min_1[0]
    else:
        a = max_1[0] - min_2[0]
        b = max_2[0] - min_1[0]
        return max(a, b)


print(max_distance([[1,5],[3,4]]))