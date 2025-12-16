"""
请你将一些箱子装在 一辆卡车 上。给你一个二维数组 boxTypes ，其中 boxTypes[i] = [numberOfBoxesi, numberOfUnitsPerBoxi] ：

numberOfBoxesi 是类型 i 的箱子的数量。
numberOfUnitsPerBoxi 是类型 i 每个箱子可以装载的单元数量。
整数 truckSize 表示卡车上可以装载 箱子 的 最大数量 。只要箱子数量不超过 truckSize ，你就可以选择任意箱子装到卡车上。

返回卡车可以装载 单元 的 最大 总数。

输入：boxTypes = [[5,10],[2,5],[4,7],[3,9]], truckSize = 10
输出：91
"""
from typing import List


def maximumUnits(boxTypes: List[List[int]], truckSize: int) -> int:
    boxTypes.sort(key=lambda x: x[1], reverse=True)
    ans = 0
    for box_num, box_size in boxTypes:
        if truckSize == 0:
            break
        if box_num <= truckSize:
            ans += box_num * box_size
            truckSize -= box_num
        else:
            ans += truckSize * box_size
            break
    return ans


box_types = [[5, 10], [2, 5], [4, 7], [3, 9]]
truck_size = 15

print(maximumUnits(box_types, truck_size))
