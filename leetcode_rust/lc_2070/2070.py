from typing import List

def maximumBeauty(items: List[List[int]], queries: List[int]) -> List[int]:
    items.sort(lambda v_b: v_b[0])
    